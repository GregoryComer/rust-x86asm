use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vrcp14sd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCP14SD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 245, 139, 77, 223], OperandSize::Dword)
}

#[test]
fn vrcp14sd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCP14SD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledDisplaced(ESI, Eight, 1625299287, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 229, 138, 77, 36, 245, 87, 25, 224, 96], OperandSize::Dword)
}

#[test]
fn vrcp14sd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCP14SD, operand1: Some(Direct(XMM21)), operand2: Some(Direct(XMM8)), operand3: Some(Direct(XMM30)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 130, 189, 139, 77, 238], OperandSize::Qword)
}

#[test]
fn vrcp14sd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCP14SD, operand1: Some(Direct(XMM25)), operand2: Some(Direct(XMM13)), operand3: Some(IndirectDisplaced(RBX, 1228874886, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 98, 149, 142, 77, 139, 134, 36, 63, 73], OperandSize::Qword)
}

