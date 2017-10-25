use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn blendvpd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::BLENDVPD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 21, 211], OperandSize::Dword)
}

#[test]
fn blendvpd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::BLENDVPD, operand1: Some(Direct(XMM2)), operand2: Some(IndirectScaledIndexedDisplaced(ESI, EBX, Four, 1828270616, Some(OperandSize::Xmmword), None)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 21, 148, 158, 24, 50, 249, 108], OperandSize::Dword)
}

#[test]
fn blendvpd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::BLENDVPD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 21, 240], OperandSize::Qword)
}

#[test]
fn blendvpd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::BLENDVPD, operand1: Some(Direct(XMM2)), operand2: Some(IndirectScaledIndexed(RAX, RDI, Eight, Some(OperandSize::Xmmword), None)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 21, 20, 248], OperandSize::Qword)
}

