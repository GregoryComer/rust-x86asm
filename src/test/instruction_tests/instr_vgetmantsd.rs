use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vgetmantsd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETMANTSD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM4)), operand4: Some(Literal8(42)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K5), broadcast: None }, &[98, 243, 205, 157, 39, 228, 42], OperandSize::Dword)
}

#[test]
fn vgetmantsd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETMANTSD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledIndexed(EAX, ECX, Two, Some(OperandSize::Qword), None)), operand4: Some(Literal8(115)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 243, 213, 143, 39, 28, 72, 115], OperandSize::Dword)
}

#[test]
fn vgetmantsd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETMANTSD, operand1: Some(Direct(XMM20)), operand2: Some(Direct(XMM9)), operand3: Some(Direct(XMM3)), operand4: Some(Literal8(56)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K1), broadcast: None }, &[98, 227, 181, 153, 39, 227, 56], OperandSize::Qword)
}

#[test]
fn vgetmantsd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETMANTSD, operand1: Some(Direct(XMM19)), operand2: Some(Direct(XMM14)), operand3: Some(IndirectScaledIndexedDisplaced(RCX, RAX, Two, 551473847, Some(OperandSize::Qword), None)), operand4: Some(Literal8(7)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 227, 141, 140, 39, 156, 65, 183, 210, 222, 32, 7], OperandSize::Qword)
}

