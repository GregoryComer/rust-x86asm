use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vrsqrt14sd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRT14SD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 229, 143, 79, 211], OperandSize::Dword)
}

#[test]
fn vrsqrt14sd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRT14SD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectDisplaced(EDI, 119033170, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 237, 141, 79, 143, 82, 77, 24, 7], OperandSize::Dword)
}

#[test]
fn vrsqrt14sd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRT14SD, operand1: Some(Direct(XMM31)), operand2: Some(Direct(XMM23)), operand3: Some(Direct(XMM29)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 2, 197, 133, 79, 253], OperandSize::Qword)
}

#[test]
fn vrsqrt14sd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRT14SD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM12)), operand3: Some(Indirect(RCX, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 157, 137, 79, 17], OperandSize::Qword)
}

