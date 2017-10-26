use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vrsqrt28ss_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRT28SS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 125, 156, 205, 245], OperandSize::Dword)
}

#[test]
fn vrsqrt28ss_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRT28SS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledDisplaced(EBX, Four, 2108421687, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 101, 140, 205, 60, 157, 55, 246, 171, 125], OperandSize::Dword)
}

#[test]
fn vrsqrt28ss_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRT28SS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM24)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K4), broadcast: None }, &[98, 146, 117, 156, 205, 248], OperandSize::Qword)
}

#[test]
fn vrsqrt28ss_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRT28SS, operand1: Some(Direct(XMM16)), operand2: Some(Direct(XMM22)), operand3: Some(IndirectDisplaced(RSI, 223405058, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 226, 77, 129, 205, 134, 2, 228, 80, 13], OperandSize::Qword)
}

