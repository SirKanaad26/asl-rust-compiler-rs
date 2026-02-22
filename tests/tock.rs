mod common;
use common::run_tock_test;

// BX R1 — branch to register
// R1 = 0x00001001 (odd address → switch to Thumb mode)
// Expected: PC = 0x00001000, T = true
#[test]
fn tock_bx() {
    run_tock_test("examples/tock/tock_bx.asl", r#"
fn main() {
    let bits: u64 = 0x47080000; // BX R1 (Rm=1 in bits[22:19])
    let mut cpu = CpuState::new();
    cpu.R[1] = 0x00001001;      // odd address → Thumb
    let enc = aarch32_BX_T1_A::decode(bits).unwrap();
    execute_aarch32_BX_A(&enc, &mut cpu);
    assert_eq!(cpu.PC, 0x00001000); // bit 0 stripped from address
    assert_eq!(cpu.T, true);        // Thumb mode selected by bit 0
}
"#);
}

// MRS R0, CPSR — read CPSR into register
// PSTATE: N=true, C=true → PSR word = 0xA0000000
// Expected: R[0] = 0xA0000000
#[test]
fn tock_mrs() {
    run_tock_test("examples/tock/tock_mrs.asl", r#"
fn main() {
    let bits: u64 = 0xF3E08000; // MRS R0, CPSR (R=0, Rd=0)
    let mut cpu = CpuState::new();
    cpu.N = true; // bit 31
    cpu.C = true; // bit 29
    let enc = aarch32_MRS_T1_AS::decode(bits).unwrap();
    execute_aarch32_MRS_AS(&enc, &mut cpu);
    assert_eq!(cpu.R[0], 0xA0000000); // N|C packed into PSR
}
"#);
}

// MSR CPSR_f, R1 — write flags byte of CPSR from register
// R1 = 0x80000000 (N flag set), mask = 8 (top byte only)
// Expected: N=true, Z=false
#[test]
fn tock_msr() {
    run_tock_test("examples/tock/tock_msr.asl", r#"
fn main() {
    let bits: u64 = 0xF3818800; // MSR CPSR_f, R1 (R=0, Rn=1, mask=8)
    let mut cpu = CpuState::new();
    cpu.R[1] = 0x80000000; // N flag in bit 31
    let enc = aarch32_MSR_r_T1_AS::decode(bits).unwrap();
    execute_aarch32_MSR_r_AS(&enc, &mut cpu);
    assert_eq!(cpu.N, true);  // bit 31 of R1 written to PSTATE.N
    assert_eq!(cpu.Z, false); // bit 30 of R1 is 0
}
"#);
}

// MOV R0, #42 — move immediate into register
// Expected: R[0] = 42
#[test]
fn tock_mov_imm() {
    run_tock_test("examples/tock/tock_mov_imm.asl", r#"
fn main() {
    let bits: u64 = 0x202A0000; // MOV R0, #42 (Rd=0, imm8=42)
    let mut cpu = CpuState::new();
    let enc = aarch32_MOV_i_T1_A::decode(bits).unwrap();
    execute_aarch32_MOV_i_A(&enc, &mut cpu);
    assert_eq!(cpu.R[0], 42);
}
"#);
}

// ANDS R1, R2 — bitwise AND R1 with R2, set flags
// R1 = 0xFF00FF00, R2 = 0x0F0F0F0F → result = 0x0F000F00
// Expected: R[1] = 0x0F000F00, N=false, Z=false, C=false
#[test]
fn tock_and() {
    run_tock_test("examples/tock/tock_and.asl", r#"
fn main() {
    let bits: u64 = 0x40110000; // ANDS R1, R2 (Rdn=1, Rm=2)
    let mut cpu = CpuState::new();
    cpu.R[1] = 0xFF00FF00;
    cpu.R[2] = 0x0F0F0F0F;
    let enc = aarch32_AND_r_T1_A::decode(bits).unwrap();
    execute_aarch32_AND_r_A(&enc, &mut cpu);
    assert_eq!(cpu.R[1], 0x0F000F00); // 0xFF00FF00 & 0x0F0F0F0F
    assert_eq!(cpu.N, false);         // bit 31 of result is 0
    assert_eq!(cpu.Z, false);         // result is non-zero
    assert_eq!(cpu.C, false);         // LSL by 0: carry = carry_in = 0
}
"#);
}
