use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vrcp28sd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCP28SD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 253, 158, 203, 207], OperandSize::Dword)
}

#[test]
fn vrcp28sd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCP28SD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledIndexed(EBX, EBX, Eight, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 253, 140, 203, 12, 219], OperandSize::Dword)
}

#[test]
fn vrcp28sd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCP28SD, operand1: Some(Direct(XMM22)), operand2: Some(Direct(XMM28)), operand3: Some(Direct(XMM13)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K7), broadcast: None }, &[98, 194, 157, 151, 203, 245], OperandSize::Qword)
}

#[test]
fn vrcp28sd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCP28SD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledDisplaced(RDX, Two, 910424306, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 237, 142, 203, 44, 85, 242, 248, 67, 54], OperandSize::Qword)
}

