use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn blendpd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::BLENDPD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM0)), operand3: Some(Literal8(79)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 13, 224, 79], OperandSize::Dword)
}

#[test]
fn blendpd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::BLENDPD, operand1: Some(Direct(XMM4)), operand2: Some(IndirectScaledIndexed(EDI, EDX, Two, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(98)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 13, 36, 87, 98], OperandSize::Dword)
}

#[test]
fn blendpd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::BLENDPD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM3)), operand3: Some(Literal8(119)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 13, 251, 119], OperandSize::Qword)
}

#[test]
fn blendpd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::BLENDPD, operand1: Some(Direct(XMM0)), operand2: Some(IndirectScaledDisplaced(RBX, Four, 868756171, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(13)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 13, 4, 157, 203, 42, 200, 51, 13], OperandSize::Qword)
}

