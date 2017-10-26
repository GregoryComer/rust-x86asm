use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vminsd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VMINSD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 235, 93, 226], OperandSize::Dword)
}

#[test]
fn vminsd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VMINSD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM5)), operand3: Some(Indirect(EBX, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 211, 93, 35], OperandSize::Dword)
}

#[test]
fn vminsd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VMINSD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 243, 93, 207], OperandSize::Qword)
}

#[test]
fn vminsd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VMINSD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectDisplaced(RBX, 1346523999, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 211, 93, 139, 95, 83, 66, 80], OperandSize::Qword)
}

#[test]
fn vminsd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VMINSD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 199, 156, 93, 237], OperandSize::Dword)
}

#[test]
fn vminsd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VMINSD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectScaledIndexedDisplaced(EBX, EBX, Two, 550018351, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 247, 139, 93, 148, 91, 47, 157, 200, 32], OperandSize::Dword)
}

#[test]
fn vminsd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VMINSD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM26)), operand3: Some(Direct(XMM11)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K5), broadcast: None }, &[98, 209, 175, 149, 93, 235], OperandSize::Qword)
}

#[test]
fn vminsd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VMINSD, operand1: Some(Direct(XMM10)), operand2: Some(Direct(XMM28)), operand3: Some(IndirectScaledDisplaced(RAX, Eight, 688961961, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 113, 159, 135, 93, 20, 197, 169, 185, 16, 41], OperandSize::Qword)
}

