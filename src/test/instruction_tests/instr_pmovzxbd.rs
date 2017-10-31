use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn pmovzxbd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PMOVZXBD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 49, 198], OperandSize::Dword)
}

#[test]
fn pmovzxbd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PMOVZXBD, operand1: Some(Direct(XMM2)), operand2: Some(IndirectDisplaced(EAX, 1245742596, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 49, 144, 4, 134, 64, 74], OperandSize::Dword)
}

#[test]
fn pmovzxbd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PMOVZXBD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 49, 205], OperandSize::Qword)
}

#[test]
fn pmovzxbd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PMOVZXBD, operand1: Some(Direct(XMM0)), operand2: Some(IndirectDisplaced(RAX, 1706637476, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 49, 128, 164, 56, 185, 101], OperandSize::Qword)
}

