use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vrndscalesd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VRNDSCALESD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM0)), operand4: Some(Literal8(76)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K7), broadcast: None }, &[98, 243, 221, 159, 11, 200, 76], OperandSize::Dword)
}

#[test]
fn vrndscalesd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VRNDSCALESD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectDisplaced(EAX, 496454770, Some(OperandSize::Qword), None)), operand4: Some(Literal8(49)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 243, 213, 140, 11, 136, 114, 76, 151, 29, 49], OperandSize::Dword)
}

#[test]
fn vrndscalesd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VRNDSCALESD, operand1: Some(Direct(XMM18)), operand2: Some(Direct(XMM18)), operand3: Some(Direct(XMM8)), operand4: Some(Literal8(38)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K3), broadcast: None }, &[98, 195, 237, 147, 11, 208, 38], OperandSize::Qword)
}

#[test]
fn vrndscalesd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VRNDSCALESD, operand1: Some(Direct(XMM8)), operand2: Some(Direct(XMM29)), operand3: Some(Indirect(RDI, Some(OperandSize::Qword), None)), operand4: Some(Literal8(112)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 115, 149, 132, 11, 7, 112], OperandSize::Qword)
}

