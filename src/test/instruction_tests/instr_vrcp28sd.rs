use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vrcp28sd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCP28SD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 237, 153, 203, 196], OperandSize::Dword)
}

#[test]
fn vrcp28sd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCP28SD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledDisplaced(EDX, Two, 2029787514, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 253, 140, 203, 44, 85, 122, 25, 252, 120], OperandSize::Dword)
}

#[test]
fn vrcp28sd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCP28SD, operand1: Some(Direct(XMM25)), operand2: Some(Direct(XMM11)), operand3: Some(Direct(XMM30)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K5), broadcast: None }, &[98, 2, 165, 157, 203, 206], OperandSize::Qword)
}

#[test]
fn vrcp28sd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCP28SD, operand1: Some(Direct(XMM8)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledIndexed(RDX, RBX, Two, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 114, 229, 142, 203, 4, 90], OperandSize::Qword)
}

