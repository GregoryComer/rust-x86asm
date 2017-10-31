use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vrsqrtss_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRTSS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 210, 82, 196], OperandSize::Dword)
}

#[test]
fn vrsqrtss_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRTSS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM6)), operand3: Some(Indirect(EDX, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 202, 82, 26], OperandSize::Dword)
}

#[test]
fn vrsqrtss_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRTSS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 242, 82, 246], OperandSize::Qword)
}

#[test]
fn vrsqrtss_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRTSS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledDisplaced(RCX, Two, 1050940418, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 250, 82, 44, 77, 2, 20, 164, 62], OperandSize::Qword)
}

