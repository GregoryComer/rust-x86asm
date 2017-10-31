use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vrsqrt14sd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRT14SD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 197, 140, 79, 240], OperandSize::Dword)
}

#[test]
fn vrsqrt14sd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRT14SD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledIndexed(EAX, ECX, Four, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 253, 137, 79, 20, 136], OperandSize::Dword)
}

#[test]
fn vrsqrt14sd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRT14SD, operand1: Some(Direct(XMM19)), operand2: Some(Direct(XMM30)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 226, 141, 134, 79, 221], OperandSize::Qword)
}

#[test]
fn vrsqrt14sd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRT14SD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM27)), operand3: Some(IndirectScaledIndexed(RBX, RSI, Eight, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 165, 135, 79, 4, 243], OperandSize::Qword)
}

