use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn unpcklps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::UNPCKLPS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 20, 227], OperandSize::Dword)
}

#[test]
fn unpcklps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::UNPCKLPS, operand1: Some(Direct(XMM3)), operand2: Some(Indirect(EAX, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 20, 24], OperandSize::Dword)
}

#[test]
fn unpcklps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::UNPCKLPS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 20, 218], OperandSize::Qword)
}

#[test]
fn unpcklps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::UNPCKLPS, operand1: Some(Direct(XMM2)), operand2: Some(IndirectScaledIndexed(RCX, RCX, Four, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 20, 20, 137], OperandSize::Qword)
}

