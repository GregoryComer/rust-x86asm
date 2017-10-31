use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn bndcu_1() {
    run_test(&Instruction { mnemonic: Mnemonic::BNDCU, operand1: Some(Direct(BND1)), operand2: Some(Direct(ECX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 26, 201], OperandSize::Dword)
}

#[test]
fn bndcu_2() {
    run_test(&Instruction { mnemonic: Mnemonic::BNDCU, operand1: Some(Direct(BND2)), operand2: Some(Indirect(EDI, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 26, 23], OperandSize::Dword)
}

#[test]
fn bndcu_3() {
    run_test(&Instruction { mnemonic: Mnemonic::BNDCU, operand1: Some(Direct(BND1)), operand2: Some(Direct(RBP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 26, 205], OperandSize::Qword)
}

#[test]
fn bndcu_4() {
    run_test(&Instruction { mnemonic: Mnemonic::BNDCU, operand1: Some(Direct(BND3)), operand2: Some(IndirectScaledDisplaced(RAX, Four, 574046270, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 26, 28, 133, 62, 64, 55, 34], OperandSize::Qword)
}

