use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vrndscalesd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VRNDSCALESD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM2)), operand4: Some(Literal8(25)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K3), broadcast: None }, &[98, 243, 253, 155, 11, 194, 25], OperandSize::Dword)
}

#[test]
fn vrndscalesd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VRNDSCALESD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledIndexedDisplaced(EDX, ESI, Eight, 1861956020, Some(OperandSize::Qword), None)), operand4: Some(Literal8(90)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 243, 237, 141, 11, 188, 242, 180, 49, 251, 110, 90], OperandSize::Dword)
}

#[test]
fn vrndscalesd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VRNDSCALESD, operand1: Some(Direct(XMM28)), operand2: Some(Direct(XMM20)), operand3: Some(Direct(XMM6)), operand4: Some(Literal8(86)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K5), broadcast: None }, &[98, 99, 221, 149, 11, 230, 86], OperandSize::Qword)
}

#[test]
fn vrndscalesd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VRNDSCALESD, operand1: Some(Direct(XMM25)), operand2: Some(Direct(XMM13)), operand3: Some(Indirect(RDI, Some(OperandSize::Qword), None)), operand4: Some(Literal8(121)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 99, 149, 140, 11, 15, 121], OperandSize::Qword)
}

