use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vrndscalesd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VRNDSCALESD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM7)), operand4: Some(Literal8(46)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K5), broadcast: None }, &[98, 243, 205, 157, 11, 255, 46], OperandSize::Dword)
}

#[test]
fn vrndscalesd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VRNDSCALESD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectScaledIndexedDisplaced(ECX, EDX, Four, 224973919, Some(OperandSize::Qword), None)), operand4: Some(Literal8(36)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 243, 245, 138, 11, 140, 145, 95, 212, 104, 13, 36], OperandSize::Dword)
}

#[test]
fn vrndscalesd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VRNDSCALESD, operand1: Some(Direct(XMM14)), operand2: Some(Direct(XMM10)), operand3: Some(Direct(XMM2)), operand4: Some(Literal8(93)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K3), broadcast: None }, &[98, 115, 173, 155, 11, 242, 93], OperandSize::Qword)
}

#[test]
fn vrndscalesd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VRNDSCALESD, operand1: Some(Direct(XMM23)), operand2: Some(Direct(XMM20)), operand3: Some(Indirect(RAX, Some(OperandSize::Qword), None)), operand4: Some(Literal8(124)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 227, 221, 129, 11, 56, 124], OperandSize::Qword)
}

