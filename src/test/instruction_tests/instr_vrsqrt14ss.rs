use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vrsqrt14ss_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRT14SS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 85, 141, 79, 240], OperandSize::Dword)
}

#[test]
fn vrsqrt14ss_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRT14SS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledIndexedDisplaced(ECX, ESI, Four, 1712547150, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 109, 142, 79, 180, 177, 78, 101, 19, 102], OperandSize::Dword)
}

#[test]
fn vrsqrt14ss_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRT14SS, operand1: Some(Direct(XMM10)), operand2: Some(Direct(XMM15)), operand3: Some(Direct(XMM9)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 82, 5, 141, 79, 209], OperandSize::Qword)
}

#[test]
fn vrsqrt14ss_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRT14SS, operand1: Some(Direct(XMM16)), operand2: Some(Direct(XMM19)), operand3: Some(IndirectScaledDisplaced(RBX, Four, 596470978, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 226, 101, 134, 79, 4, 157, 194, 108, 141, 35], OperandSize::Qword)
}

