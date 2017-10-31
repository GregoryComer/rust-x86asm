use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vgetmantsd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETMANTSD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM2)), operand4: Some(Literal8(118)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K6), broadcast: None }, &[98, 243, 213, 158, 39, 210, 118], OperandSize::Dword)
}

#[test]
fn vgetmantsd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETMANTSD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectScaledIndexed(EBX, EAX, Four, Some(OperandSize::Qword), None)), operand4: Some(Literal8(73)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 243, 205, 142, 39, 52, 131, 73], OperandSize::Dword)
}

#[test]
fn vgetmantsd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETMANTSD, operand1: Some(Direct(XMM12)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM13)), operand4: Some(Literal8(46)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K4), broadcast: None }, &[98, 83, 245, 156, 39, 229, 46], OperandSize::Qword)
}

#[test]
fn vgetmantsd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETMANTSD, operand1: Some(Direct(XMM18)), operand2: Some(Direct(XMM22)), operand3: Some(Indirect(RBX, Some(OperandSize::Qword), None)), operand4: Some(Literal8(30)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 227, 205, 133, 39, 19, 30], OperandSize::Qword)
}

