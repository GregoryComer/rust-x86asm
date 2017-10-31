use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vrcp14sd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCP14SD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 237, 139, 77, 218], OperandSize::Dword)
}

#[test]
fn vrcp14sd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCP14SD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledIndexedDisplaced(EDX, ESI, Eight, 1307021031, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 221, 142, 77, 180, 242, 231, 142, 231, 77], OperandSize::Dword)
}

#[test]
fn vrcp14sd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCP14SD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM14)), operand3: Some(Direct(XMM25)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 146, 141, 143, 77, 241], OperandSize::Qword)
}

#[test]
fn vrcp14sd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCP14SD, operand1: Some(Direct(XMM25)), operand2: Some(Direct(XMM30)), operand3: Some(IndirectDisplaced(RCX, 1540075568, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 98, 141, 133, 77, 137, 48, 176, 203, 91], OperandSize::Qword)
}

