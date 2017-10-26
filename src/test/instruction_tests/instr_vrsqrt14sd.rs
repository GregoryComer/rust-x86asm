use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vrsqrt14sd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRT14SD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 221, 142, 79, 206], OperandSize::Dword)
}

#[test]
fn vrsqrt14sd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRT14SD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledIndexed(EDI, EDX, Two, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 253, 143, 79, 36, 87], OperandSize::Dword)
}

#[test]
fn vrsqrt14sd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRT14SD, operand1: Some(Direct(XMM29)), operand2: Some(Direct(XMM8)), operand3: Some(Direct(XMM17)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 34, 189, 138, 79, 233], OperandSize::Qword)
}

#[test]
fn vrsqrt14sd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRT14SD, operand1: Some(Direct(XMM11)), operand2: Some(Direct(XMM9)), operand3: Some(Indirect(RCX, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 114, 181, 137, 79, 25], OperandSize::Qword)
}

