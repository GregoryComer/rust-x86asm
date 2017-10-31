use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn andnpd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::ANDNPD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 85, 194], OperandSize::Dword)
}

#[test]
fn andnpd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::ANDNPD, operand1: Some(Direct(XMM2)), operand2: Some(IndirectScaledIndexedDisplaced(EAX, ECX, Two, 1621949703, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 85, 148, 72, 7, 253, 172, 96], OperandSize::Dword)
}

#[test]
fn andnpd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::ANDNPD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 85, 235], OperandSize::Qword)
}

#[test]
fn andnpd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::ANDNPD, operand1: Some(Direct(XMM4)), operand2: Some(IndirectScaledIndexedDisplaced(RCX, RCX, Four, 898702028, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 85, 164, 137, 204, 26, 145, 53], OperandSize::Qword)
}

