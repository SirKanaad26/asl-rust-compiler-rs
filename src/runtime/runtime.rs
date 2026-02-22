// ASL runtime — shared preamble for all generated instruction files.
// Imported via `mod runtime; use runtime::*;` in the generated crate root.
use crate::bitvec::{AslValue, BitVec};

// ── ASL built-in runtime stubs ───────────────────────────────────────────────

/// Unsigned integer conversion.  Accepts any AslValue so u64, i128, and
/// BitVec<N> all flow through without explicit casts in generated code.
pub fn UInt(x: impl AslValue) -> i128 { x.to_u128() as i128 }

/// Signed integer conversion with sign-extension at bit width-1.
pub fn SInt(x: impl AslValue) -> i128 {
    let w = x.asl_bit_width();
    let v = x.to_u128();
    if w == 0 || (v >> (w - 1)) & 1 == 0 { v as i128 }
    else { (v | !((1u128 << w).wrapping_sub(1))) as i128 }
}

pub fn IsZero(x: impl AslValue) -> bool { x.to_u128() == 0 }
pub fn IsOnes(x: impl AslValue) -> bool { x.to_u64() == u64::MAX }
pub fn Zeros(_n: impl AslValue) -> i128 { 0 }
pub fn Ones(n: impl AslValue) -> i128 {
    let n = n.to_u64();
    if n >= 64 { u64::MAX as i128 } else { ((1u64 << n) - 1) as i128 }
}
pub fn ZeroExtend(x: impl AslValue, _n: impl AslValue) -> i128 { x.to_u128() as i128 }

pub fn HaveFP16Ext() -> bool { false }
pub fn HaveBF16Ext() -> bool { false }
pub fn HaveSVE() -> bool { false }
pub fn HaveSVE2() -> bool { false }
pub fn HaveMTE() -> bool { false }

/// Mathematical modulo (always non-negative, unlike Rust `%`).
pub fn asl_mod(a: i128, b: i128) -> i128 { ((a % b) + b) % b }

// ── CPU state ────────────────────────────────────────────────────────────────

pub struct CpuState {
    pub X: [u64; 32],   // AArch64 64-bit GPRs
    pub R: [u64; 16],   // AArch32 32-bit GPRs
    pub S: [u64; 32],   // VFP single-precision
    pub VD: [u64; 32],  // VFP double-precision (D renamed to avoid clash with PSTATE.D)
    pub SP: u64,
    pub PC: u64,
    // PSTATE flags
    pub N: bool,
    pub Z: bool,
    pub C: bool,
    pub V: bool,
    pub EL: u8,
    pub M: u8,
    pub T: bool,
    pub nRW: bool,
    pub SS: bool,
    pub IL: bool,
    pub D: bool,
    pub A: bool,
    pub I: bool,
    pub F: bool,
    // Saved Program Status Register (AArch32)
    pub SPSR: u32,
}

impl CpuState {
    pub fn new() -> Self {
        CpuState {
            X: [0u64; 32], R: [0u64; 16], S: [0u64; 32], VD: [0u64; 32],
            SP: 0, PC: 0,
            N: false, Z: false, C: false, V: false,
            EL: 0, M: 0, T: false, nRW: false,
            SS: false, IL: false, D: false, A: false, I: false, F: false,
            SPSR: 0,
        }
    }
}

// Register read accessors return i128 (ASL integer) so the result can be stored
// directly in integer or bits(N) decoded vars without explicit casts.
pub fn Xreg(cpu: &CpuState, n: impl AslValue) -> i128 { cpu.X[n.to_u64() as usize] as i128 }
pub fn Wreg(cpu: &CpuState, n: impl AslValue) -> i128 { (cpu.X[n.to_u64() as usize] & 0xFFFF_FFFF) as i128 }
pub fn set_Xreg(cpu: &mut CpuState, n: impl AslValue, val: impl AslValue) { cpu.X[n.to_u64() as usize] = val.to_u64() }
pub fn set_Wreg(cpu: &mut CpuState, n: impl AslValue, val: impl AslValue) { cpu.X[n.to_u64() as usize] = val.to_u64() & 0xFFFF_FFFF }
pub fn Rreg(cpu: &CpuState, n: impl AslValue) -> i128 { cpu.R[n.to_u64() as usize] as i128 }
pub fn set_Rreg(cpu: &mut CpuState, n: impl AslValue, val: impl AslValue) { cpu.R[n.to_u64() as usize] = val.to_u64() & 0xFFFF_FFFF }
pub fn Sreg(cpu: &CpuState, n: impl AslValue) -> i128 { cpu.S[n.to_u64() as usize] as i128 }
pub fn set_Sreg(cpu: &mut CpuState, n: impl AslValue, val: impl AslValue) { cpu.S[n.to_u64() as usize] = val.to_u64() }
pub fn Dreg(cpu: &CpuState, n: impl AslValue) -> i128 { cpu.VD[n.to_u64() as usize] as i128 }
pub fn set_Dreg(cpu: &mut CpuState, n: impl AslValue, val: impl AslValue) { cpu.VD[n.to_u64() as usize] = val.to_u64() }
pub fn check_condition(cpu: &CpuState) -> bool { true /* TODO: evaluate CPSR/PSTATE condition codes */ }

// ── Branch type constants ────────────────────────────────────────────────────
// ASL enum BranchType → named i128 constants.

pub const BranchType_INDIR: i128 = 0;
pub const BranchType_DIR: i128 = 1;
pub const BranchType_DIRCALL: i128 = 2;
pub const BranchType_INDIRCALL: i128 = 3;
pub const BranchType_ERET: i128 = 4;
pub const BranchType_DBGEXIT: i128 = 5;

// ── ARM operation implementations ────────────────────────────────────────────

/// Pack PSTATE (cpu.*) fields into a 32-bit PSR word.
pub fn GetPSRFromPSTATE(cpu: &CpuState) -> i128 {
    let mut psr: u32 = 0;
    if cpu.N  { psr |= 1 << 31; }
    if cpu.Z  { psr |= 1 << 30; }
    if cpu.C  { psr |= 1 << 29; }
    if cpu.V  { psr |= 1 << 28; }
    psr |= (cpu.M as u32) & 0x1F;  // mode bits [4:0]
    if cpu.T  { psr |= 1 << 5; }
    if cpu.F  { psr |= 1 << 6; }
    if cpu.I  { psr |= 1 << 7; }
    if cpu.A  { psr |= 1 << 8; }
    if cpu.D  { psr |= 1 << 9; }
    psr |= (cpu.IL as u32) << 20;
    psr |= (cpu.SS as u32) << 21;
    psr as i128
}

/// Read SPSR (Saved Program Status Register).
pub fn get_SPSR(cpu: &CpuState) -> i128 { cpu.SPSR as i128 }

/// Write selected byte lanes of SPSR from `val` using `mask` as byte-lane enables.
pub fn SPSRWriteByInstr(cpu: &mut CpuState, val: impl AslValue, mask: impl AslValue) {
    let v = val.to_u64() as u32;
    let m = mask.to_u64() as u32;
    let byte_mask =
        (if m & 1 != 0 { 0x000000FFu32 } else { 0 }) |
        (if m & 2 != 0 { 0x0000FF00u32 } else { 0 }) |
        (if m & 4 != 0 { 0x00FF0000u32 } else { 0 }) |
        (if m & 8 != 0 { 0xFF000000u32 } else { 0 });
    cpu.SPSR = (cpu.SPSR & !byte_mask) | (v & byte_mask);
}

/// Write selected byte lanes of CPSR from `val` and unpack back into cpu.* fields.
pub fn CPSRWriteByInstr(cpu: &mut CpuState, val: impl AslValue, mask: impl AslValue) {
    let v = val.to_u64() as u32;
    let m = mask.to_u64() as u32;
    let byte_mask =
        (if m & 1 != 0 { 0x000000FFu32 } else { 0 }) |
        (if m & 2 != 0 { 0x0000FF00u32 } else { 0 }) |
        (if m & 4 != 0 { 0x00FF0000u32 } else { 0 }) |
        (if m & 8 != 0 { 0xFF000000u32 } else { 0 });
    let new_psr = (GetPSRFromPSTATE(cpu) as u32 & !byte_mask) | (v & byte_mask);
    cpu.N   = (new_psr >> 31) & 1 == 1;
    cpu.Z   = (new_psr >> 30) & 1 == 1;
    cpu.C   = (new_psr >> 29) & 1 == 1;
    cpu.V   = (new_psr >> 28) & 1 == 1;
    cpu.T   = (new_psr >>  5) & 1 == 1;
    cpu.F   = (new_psr >>  6) & 1 == 1;
    cpu.I   = (new_psr >>  7) & 1 == 1;
    cpu.A   = (new_psr >>  8) & 1 == 1;
    cpu.M   =  (new_psr & 0x1F) as u8;
}

// PC-write helpers (branch instructions)

/// BX-style write: bit 0 selects Thumb mode, then stripped from address.
pub fn BXWritePC(cpu: &mut CpuState, addr: impl AslValue, _btype: i128) {
    let a = addr.to_u64();
    cpu.T  = (a & 1) == 1;
    cpu.PC = a & !1u64;
}

pub fn ALUWritePC(cpu: &mut CpuState, result: impl AslValue) {
    BXWritePC(cpu, result, BranchType_DIR);
}

pub fn BranchWritePC(cpu: &mut CpuState, addr: impl AslValue) {
    cpu.PC = addr.to_u64() & !3u64;  // word-align for AArch32
}

pub fn BranchTo(cpu: &mut CpuState, addr: impl AslValue, _btype: i128) {
    cpu.PC = addr.to_u64();
}

pub fn LoadWritePC(cpu: &mut CpuState, addr: impl AslValue) {
    BXWritePC(cpu, addr, BranchType_INDIR);
}

pub fn AArch64_BranchTo(cpu: &mut CpuState, addr: impl AslValue, _btype: i128) {
    cpu.PC = addr.to_u64();
}

// Enabled / validity checks (decode guards)
pub fn CheckVFPEnabled(_cpu: &CpuState, _exc_on_failure: impl AslValue) { /* TODO: check VFP enabled */ }
pub fn CheckAdvSIMDOrFPEnabled(_cpu: &CpuState, _exc: impl AslValue) { /* TODO */ }

// ── Shift-type constants (ASL enum SRType) ───────────────────────────────────
pub const SRType_LSL: i128 = 0;
pub const SRType_LSR: i128 = 1;
pub const SRType_ASR: i128 = 2;
pub const SRType_ROR: i128 = 3;
pub const SRType_RRX: i128 = 4;

/// Single-bit extraction: asl_bit(x, i) == true iff bit i of x is set.
/// Used in generated code for `result<31>` etc. (single-bit slice on any AslValue).
pub fn asl_bit(x: impl AslValue, i: usize) -> bool {
    (x.to_u128() >> i) & 1 != 0
}

/// ARM shift-with-carry: returns (shifted_result, carry_out).
/// Implements ARMv7-M Shift_C pseudocode for all shift types.
pub fn Shift_C(val: impl AslValue, shift_t: impl AslValue, shift_n: impl AslValue, carry_in: impl AslValue) -> (i128, bool) {
    let v = val.to_u64() & 0xFFFF_FFFF;
    let st = shift_t.to_u64() as i128;
    let sn = shift_n.to_u64() as u32;
    let cin = carry_in.to_u128() & 1 != 0;

    let (result32, carry_out): (u32, bool) = match st {
        SRType_LSL => {
            if sn == 0 { (v as u32, cin) }
            else if sn < 32 {
                let carry = (v >> (32 - sn)) & 1 != 0;
                (((v << sn) & 0xFFFF_FFFF) as u32, carry)
            } else if sn == 32 {
                (0u32, v & 1 != 0)
            } else {
                (0u32, false)
            }
        }
        SRType_LSR => {
            if sn == 0 { (v as u32, cin) }
            else if sn < 32 {
                let carry = (v >> (sn - 1)) & 1 != 0;
                ((v >> sn) as u32, carry)
            } else if sn == 32 {
                (0u32, v >> 31 != 0)
            } else {
                (0u32, false)
            }
        }
        SRType_ASR => {
            let n = sn.min(32);
            let sv = v as i32;
            let carry = (v >> (n - 1)) & 1 != 0;
            ((sv >> n) as u32, carry)
        }
        SRType_ROR => {
            let n = sn % 32;
            if n == 0 {
                let carry = v >> 31 != 0;
                (v as u32, carry)
            } else {
                let result = v.rotate_right(n);
                let carry = (result >> 31) & 1 != 0;
                (result as u32, carry)
            }
        }
        SRType_RRX => {
            let carry = v & 1 != 0;
            let result = ((cin as u32) << 31) | ((v >> 1) as u32);
            (result, carry)
        }
        _ => (v as u32, cin),
    };
    (result32 as i128, carry_out)
}

/// True iff all bits of x are zero.
pub fn IsZeroBit(x: impl AslValue) -> bool { x.to_u128() == 0 }

/// Stub: returns false (we never model IT-block state in simulation).
pub fn InITBlock() -> bool { false }

/// Decode immediate shift field → (SRType, shift_amount).
/// ARM: LSR/ASR encoding of 0 means shift by 32.
pub fn DecodeImmShift(stype: impl AslValue, imm: impl AslValue) -> (i128, i128) {
    let st = stype.to_u64();
    let n  = imm.to_u64();
    let (shift_t, shift_n) = match (st, n) {
        (0b00, _)      => (SRType_LSL, n as i128),
        (0b01, 0)      => (SRType_LSR, 32i128),
        (0b01, _)      => (SRType_LSR, n as i128),
        (0b10, 0)      => (SRType_ASR, 32i128),
        (0b10, _)      => (SRType_ASR, n as i128),
        (0b11, 0)      => (SRType_RRX, 1i128),
        _              => (SRType_ROR, n as i128),
    };
    (shift_t, shift_n)
}

/// ARM exception return via PC (A32 mode only; stub for Thumb simulation).
pub fn ALUExceptionReturn(cpu: &mut CpuState, _result: impl AslValue) {
    // TODO: restore CPSR from SPSR, branch to result address
    let _ = cpu;
}
