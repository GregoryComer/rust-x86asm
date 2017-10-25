use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn lfs_1() {
    run_test(&Instruction { mnemonic: Mnemonic::LFS, operand1: Some(Direct(DX)), operand2: Some(IndirectScaledIndexedDisplaced(BX, DI, One, 147, Some(OperandSize::Far16), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 180, 145, 147, 0], OperandSize::Word)
}

#[test]
fn lfs_2() {
    run_test(&Instruction { mnemonic: Mnemonic::LFS, operand1: Some(Direct(SP)), operand2: Some(Indirect(EDX, Some(OperandSize::Far16), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 180, 34], OperandSize::Dword)
}

#[test]
fn lfs_3() {
    run_test(&Instruction { mnemonic: Mnemonic::LFS, operand1: Some(Direct(DX)), operand2: Some(IndirectScaledDisplaced(RBX, Two, 1210807725, Some(OperandSize::Far16), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 180, 20, 93, 173, 117, 43, 72], OperandSize::Qword)
}

#[test]
fn lfs_4() {
    run_test(&Instruction { mnemonic: Mnemonic::LFS, operand1: Some(Direct(EBP)), operand2: Some(Indirect(EDI, Some(OperandSize::Far32), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 180, 47], OperandSize::Dword)
}

#[test]
fn lfs_5() {
    run_test(&Instruction { mnemonic: Mnemonic::LFS, operand1: Some(Direct(EDI)), operand2: Some(Indirect(RDX, Some(OperandSize::Far32), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 180, 58], OperandSize::Qword)
}

#[test]
fn lfs_6() {
    run_test(&Instruction { mnemonic: Mnemonic::LFS, operand1: Some(Direct(RDI)), operand2: Some(IndirectScaledDisplaced(RDX, Four, 1029898186, Some(OperandSize::Far64), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 180, 60, 149, 202, 255, 98, 61], OperandSize::Qword)
}

