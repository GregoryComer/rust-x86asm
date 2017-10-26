use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vrcp28ss_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCP28SS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 117, 159, 203, 212], OperandSize::Dword)
}

#[test]
fn vrcp28ss_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCP28SS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectDisplaced(EBX, 1698405630, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 117, 138, 203, 187, 254, 156, 59, 101], OperandSize::Dword)
}

#[test]
fn vrcp28ss_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCP28SS, operand1: Some(Direct(XMM12)), operand2: Some(Direct(XMM11)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K7), broadcast: None }, &[98, 114, 37, 159, 203, 224], OperandSize::Qword)
}

#[test]
fn vrcp28ss_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCP28SS, operand1: Some(Direct(XMM31)), operand2: Some(Direct(XMM29)), operand3: Some(IndirectScaledIndexed(RDI, RDX, Eight, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 98, 21, 135, 203, 60, 215], OperandSize::Qword)
}

