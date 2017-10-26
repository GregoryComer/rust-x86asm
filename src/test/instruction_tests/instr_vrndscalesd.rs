use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vrndscalesd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VRNDSCALESD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM3)), operand4: Some(Literal8(53)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K7), broadcast: None }, &[98, 243, 253, 159, 11, 243, 53], OperandSize::Dword)
}

#[test]
fn vrndscalesd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VRNDSCALESD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledIndexedDisplaced(EBX, EAX, Eight, 1703175318, Some(OperandSize::Qword), None)), operand4: Some(Literal8(111)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 243, 237, 143, 11, 148, 195, 150, 100, 132, 101, 111], OperandSize::Dword)
}

#[test]
fn vrndscalesd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VRNDSCALESD, operand1: Some(Direct(XMM22)), operand2: Some(Direct(XMM15)), operand3: Some(Direct(XMM25)), operand4: Some(Literal8(22)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K2), broadcast: None }, &[98, 131, 133, 154, 11, 241, 22], OperandSize::Qword)
}

#[test]
fn vrndscalesd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VRNDSCALESD, operand1: Some(Direct(XMM31)), operand2: Some(Direct(XMM21)), operand3: Some(IndirectDisplaced(RBX, 51428242, Some(OperandSize::Qword), None)), operand4: Some(Literal8(42)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 99, 213, 134, 11, 187, 146, 187, 16, 3, 42], OperandSize::Qword)
}

