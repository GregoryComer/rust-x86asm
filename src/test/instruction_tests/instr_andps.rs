use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn andps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::ANDPS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 84, 242], OperandSize::Dword)
}

#[test]
fn andps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::ANDPS, operand1: Some(Direct(XMM3)), operand2: Some(IndirectDisplaced(EBX, 1751442974, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 84, 155, 30, 230, 100, 104], OperandSize::Dword)
}

#[test]
fn andps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::ANDPS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 84, 251], OperandSize::Qword)
}

#[test]
fn andps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::ANDPS, operand1: Some(Direct(XMM6)), operand2: Some(IndirectDisplaced(RDX, 925820545, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 84, 178, 129, 230, 46, 55], OperandSize::Qword)
}

