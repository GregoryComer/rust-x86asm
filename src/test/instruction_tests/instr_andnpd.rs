use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn andnpd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::ANDNPD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 85, 253], OperandSize::Dword)
}

#[test]
fn andnpd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::ANDNPD, operand1: Some(Direct(XMM3)), operand2: Some(IndirectScaledIndexed(EDI, EAX, Four, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 85, 28, 135], OperandSize::Dword)
}

#[test]
fn andnpd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::ANDNPD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 85, 217], OperandSize::Qword)
}

#[test]
fn andnpd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::ANDNPD, operand1: Some(Direct(XMM1)), operand2: Some(Indirect(RAX, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 85, 8], OperandSize::Qword)
}

