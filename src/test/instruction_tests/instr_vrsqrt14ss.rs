use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vrsqrt14ss_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRT14SS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 125, 143, 79, 198], OperandSize::Dword)
}

#[test]
fn vrsqrt14ss_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRT14SS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM0)), operand3: Some(Indirect(EBX, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 125, 137, 79, 19], OperandSize::Dword)
}

#[test]
fn vrsqrt14ss_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRT14SS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM29)), operand3: Some(Direct(XMM21)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 178, 21, 129, 79, 237], OperandSize::Qword)
}

#[test]
fn vrsqrt14ss_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRT14SS, operand1: Some(Direct(XMM19)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledDisplaced(RDI, Two, 1438973436, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 226, 85, 142, 79, 28, 125, 252, 253, 196, 85], OperandSize::Qword)
}

