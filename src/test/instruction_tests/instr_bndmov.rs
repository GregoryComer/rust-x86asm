use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn bndmov_1() {
    run_test(&Instruction { mnemonic: Mnemonic::BNDMOV, operand1: Some(Direct(BND3)), operand2: Some(Direct(BND0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 26, 216], OperandSize::Dword)
}

#[test]
fn bndmov_2() {
    run_test(&Instruction { mnemonic: Mnemonic::BNDMOV, operand1: Some(Direct(BND3)), operand2: Some(IndirectScaledIndexedDisplaced(EBX, EAX, Four, 378298555, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 26, 156, 131, 187, 96, 140, 22], OperandSize::Dword)
}

#[test]
fn bndmov_3() {
    run_test(&Instruction { mnemonic: Mnemonic::BNDMOV, operand1: Some(Direct(BND3)), operand2: Some(Direct(BND3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 26, 219], OperandSize::Qword)
}

#[test]
fn bndmov_4() {
    run_test(&Instruction { mnemonic: Mnemonic::BNDMOV, operand1: Some(Direct(BND2)), operand2: Some(IndirectScaledIndexed(RCX, RBX, Eight, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 26, 20, 217], OperandSize::Qword)
}

#[test]
fn bndmov_5() {
    run_test(&Instruction { mnemonic: Mnemonic::BNDMOV, operand1: Some(Direct(BND2)), operand2: Some(Direct(BND2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 26, 210], OperandSize::Dword)
}

#[test]
fn bndmov_6() {
    run_test(&Instruction { mnemonic: Mnemonic::BNDMOV, operand1: Some(IndirectScaledDisplaced(EDX, Eight, 1095309300, Some(OperandSize::Qword), None)), operand2: Some(Direct(BND2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 27, 20, 213, 244, 23, 73, 65], OperandSize::Dword)
}

#[test]
fn bndmov_7() {
    run_test(&Instruction { mnemonic: Mnemonic::BNDMOV, operand1: Some(Direct(BND0)), operand2: Some(Direct(BND2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 26, 194], OperandSize::Qword)
}

#[test]
fn bndmov_8() {
    run_test(&Instruction { mnemonic: Mnemonic::BNDMOV, operand1: Some(Indirect(RCX, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(BND2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 27, 17], OperandSize::Qword)
}

