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
}

impl CpuState {
    pub fn new() -> Self {
        CpuState {
            X: [0u64; 32], R: [0u64; 16], S: [0u64; 32], VD: [0u64; 32],
            SP: 0, PC: 0,
            N: false, Z: false, C: false, V: false,
            EL: 0, M: 0, T: false, nRW: false,
            SS: false, IL: false, D: false, A: false, I: false, F: false,
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

// ── ARM operation stubs ──────────────────────────────────────────────────────

// PC-write stubs (branch instructions)
pub fn BXWritePC(_addr: impl AslValue, _btype: i128) { /* TODO: cpu.PC = addr */ }
pub fn ALUWritePC(_result: impl AslValue) { /* TODO: cpu.PC = result */ }
pub fn BranchWritePC(_addr: impl AslValue) { /* TODO: cpu.PC = addr */ }
pub fn BranchTo(_addr: impl AslValue, _btype: i128) { /* TODO: cpu.PC = addr */ }
pub fn LoadWritePC(_addr: impl AslValue) { /* TODO: cpu.PC = addr */ }

// PSR access stubs (MRS / MSR instructions)
pub fn get_SPSR(_cpu: &CpuState) -> i128 { 0 /* TODO: return SPSR value */ }
pub fn GetPSRFromPSTATE() -> i128 { 0 /* TODO: pack PSTATE fields into PSR */ }
pub fn SPSRWriteByInstr(_val: impl AslValue, _mask: impl AslValue) { /* TODO: write SPSR */ }
pub fn CPSRWriteByInstr(_val: impl AslValue, _mask: impl AslValue) { /* TODO: write CPSR/PSTATE */ }

// Enabled / validity checks (decode guards)
pub fn CheckVFPEnabled(_exc_on_failure: impl AslValue) { /* TODO: check VFP enabled */ }
pub fn CheckAdvSIMDOrFPEnabled(_exc: impl AslValue) { /* TODO */ }
pub fn AArch64_BranchTo(_addr: impl AslValue, _btype: i128) { /* TODO: cpu.PC = addr */ }
