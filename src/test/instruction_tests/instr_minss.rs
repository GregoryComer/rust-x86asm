use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn minss_1() {
    run_test(&Instruction { mnemonic: Mnemonic::MINSS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 93, 222], OperandSize::Dword)
}

#[test]
fn minss_2() {
    run_test(&Instruction { mnemonic: Mnemonic::MINSS, operand1: Some(Direct(XMM0)), operand2: Some(IndirectDisplaced(EAX, 1742228650, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 93, 128, 170, 76, 216, 103], OperandSize::Dword)
}

#[test]
fn minss_3() {
    run_test(&Instruction { mnemonic: Mnemonic::MINSS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 93, 213], OperandSize::Qword)
}

#[test]
fn minss_4() {
    run_test(&Instruction { mnemonic: Mnemonic::MINSS, operand1: Some(Direct(XMM0)), operand2: Some(IndirectScaledIndexed(RDI, RBX, Eight, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 93, 4, 223], OperandSize::Qword)
}

