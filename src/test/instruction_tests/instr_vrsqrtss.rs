use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vrsqrtss_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRTSS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 226, 82, 236], OperandSize::Dword)
}

#[test]
fn vrsqrtss_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRTSS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectDisplaced(EAX, 2105771827, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 194, 82, 176, 51, 135, 131, 125], OperandSize::Dword)
}

#[test]
fn vrsqrtss_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRTSS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 226, 82, 220], OperandSize::Qword)
}

#[test]
fn vrsqrtss_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRTSS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectDisplaced(RSI, 5312666, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 226, 82, 182, 154, 16, 81, 0], OperandSize::Qword)
}

