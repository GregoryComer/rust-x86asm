use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn lfs_1() {
    run_test(&Instruction { mnemonic: Mnemonic::LFS, operand1: Some(Direct(DX)), operand2: Some(IndirectScaledIndexed(BP, SI, One, Some(OperandSize::Far16), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 180, 18], OperandSize::Word)
}

#[test]
fn lfs_2() {
    run_test(&Instruction { mnemonic: Mnemonic::LFS, operand1: Some(Direct(SP)), operand2: Some(Indirect(EDI, Some(OperandSize::Far16), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 180, 39], OperandSize::Dword)
}

#[test]
fn lfs_3() {
    run_test(&Instruction { mnemonic: Mnemonic::LFS, operand1: Some(Direct(DI)), operand2: Some(IndirectScaledIndexed(RBX, RDI, Four, Some(OperandSize::Far16), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 180, 60, 187], OperandSize::Qword)
}

#[test]
fn lfs_4() {
    run_test(&Instruction { mnemonic: Mnemonic::LFS, operand1: Some(Direct(EDX)), operand2: Some(IndirectDisplaced(ESI, 1744175626, Some(OperandSize::Far32), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 180, 150, 10, 2, 246, 103], OperandSize::Dword)
}

#[test]
fn lfs_5() {
    run_test(&Instruction { mnemonic: Mnemonic::LFS, operand1: Some(Direct(EDI)), operand2: Some(IndirectScaledIndexed(RAX, RAX, Four, Some(OperandSize::Far32), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 180, 60, 128], OperandSize::Qword)
}

#[test]
fn lfs_6() {
    run_test(&Instruction { mnemonic: Mnemonic::LFS, operand1: Some(Direct(RDI)), operand2: Some(IndirectScaledIndexedDisplaced(RSI, RCX, Four, 473286109, Some(OperandSize::Far64), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 180, 188, 142, 221, 197, 53, 28], OperandSize::Qword)
}

