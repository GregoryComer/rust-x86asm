use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn lfs_1() {
    run_test(&Instruction { mnemonic: Mnemonic::LFS, operand1: Some(Direct(CX)), operand2: Some(IndirectScaledIndexedDisplaced(BX, SI, One, 85, Some(OperandSize::Far16), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 180, 72, 85], OperandSize::Word)
}

#[test]
fn lfs_2() {
    run_test(&Instruction { mnemonic: Mnemonic::LFS, operand1: Some(Direct(DX)), operand2: Some(IndirectScaledDisplaced(ESI, Four, 200462484, Some(OperandSize::Far16), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 180, 20, 181, 148, 208, 242, 11], OperandSize::Dword)
}

#[test]
fn lfs_3() {
    run_test(&Instruction { mnemonic: Mnemonic::LFS, operand1: Some(Direct(SI)), operand2: Some(IndirectScaledDisplaced(RAX, Two, 680365710, Some(OperandSize::Far16), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 180, 52, 69, 142, 142, 141, 40], OperandSize::Qword)
}

#[test]
fn lfs_4() {
    run_test(&Instruction { mnemonic: Mnemonic::LFS, operand1: Some(Direct(EDX)), operand2: Some(IndirectScaledIndexed(EDX, EDX, Eight, Some(OperandSize::Far32), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 180, 20, 210], OperandSize::Dword)
}

#[test]
fn lfs_5() {
    run_test(&Instruction { mnemonic: Mnemonic::LFS, operand1: Some(Direct(ESI)), operand2: Some(IndirectDisplaced(RBX, 2090509387, Some(OperandSize::Far32), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 180, 179, 75, 164, 154, 124], OperandSize::Qword)
}

#[test]
fn lfs_6() {
    run_test(&Instruction { mnemonic: Mnemonic::LFS, operand1: Some(Direct(RCX)), operand2: Some(IndirectScaledDisplaced(RAX, Four, 1308461700, Some(OperandSize::Far64), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 180, 12, 133, 132, 138, 253, 77], OperandSize::Qword)
}

