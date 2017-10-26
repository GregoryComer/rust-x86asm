use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vrsqrt28sd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRT28SD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 213, 156, 205, 212], OperandSize::Dword)
}

#[test]
fn vrsqrt28sd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRT28SD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM7)), operand3: Some(Indirect(ESI, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 197, 141, 205, 54], OperandSize::Dword)
}

#[test]
fn vrsqrt28sd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRT28SD, operand1: Some(Direct(XMM26)), operand2: Some(Direct(XMM31)), operand3: Some(Direct(XMM26)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K1), broadcast: None }, &[98, 2, 133, 145, 205, 210], OperandSize::Qword)
}

#[test]
fn vrsqrt28sd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRT28SD, operand1: Some(Direct(XMM15)), operand2: Some(Direct(XMM9)), operand3: Some(IndirectDisplaced(RDX, 1782125153, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 114, 181, 143, 205, 186, 97, 18, 57, 106], OperandSize::Qword)
}

