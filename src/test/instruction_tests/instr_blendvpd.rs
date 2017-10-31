use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn blendvpd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::BLENDVPD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 21, 208], OperandSize::Dword)
}

#[test]
fn blendvpd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::BLENDVPD, operand1: Some(Direct(XMM6)), operand2: Some(IndirectScaledIndexedDisplaced(ESI, EDX, Two, 1964759861, Some(OperandSize::Xmmword), None)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 21, 180, 86, 53, 219, 27, 117], OperandSize::Dword)
}

#[test]
fn blendvpd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::BLENDVPD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 21, 213], OperandSize::Qword)
}

#[test]
fn blendvpd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::BLENDVPD, operand1: Some(Direct(XMM6)), operand2: Some(IndirectScaledDisplaced(RDX, Two, 719430110, Some(OperandSize::Xmmword), None)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 21, 52, 85, 222, 161, 225, 42], OperandSize::Qword)
}

