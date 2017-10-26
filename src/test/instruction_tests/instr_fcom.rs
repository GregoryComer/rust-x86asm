use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn fcom_1() {
    run_test(&Instruction { mnemonic: Mnemonic::FCOM, operand1: Some(IndirectDisplaced(DI, 180, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[216, 149, 180, 0], OperandSize::Word)
}

#[test]
fn fcom_2() {
    run_test(&Instruction { mnemonic: Mnemonic::FCOM, operand1: Some(IndirectScaledDisplaced(EAX, Four, 59492804, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[216, 20, 133, 196, 201, 139, 3], OperandSize::Dword)
}

#[test]
fn fcom_3() {
    run_test(&Instruction { mnemonic: Mnemonic::FCOM, operand1: Some(Indirect(RDI, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[216, 23], OperandSize::Qword)
}

#[test]
fn fcom_4() {
    run_test(&Instruction { mnemonic: Mnemonic::FCOM, operand1: Some(Direct(ST7)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[216, 215], OperandSize::Word)
}

#[test]
fn fcom_5() {
    run_test(&Instruction { mnemonic: Mnemonic::FCOM, operand1: Some(Direct(ST5)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[216, 213], OperandSize::Dword)
}

#[test]
fn fcom_6() {
    run_test(&Instruction { mnemonic: Mnemonic::FCOM, operand1: Some(Direct(ST3)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[216, 211], OperandSize::Qword)
}

#[test]
fn fcom_7() {
    run_test(&Instruction { mnemonic: Mnemonic::FCOM, operand1: Some(IndirectDisplaced(DI, 25377, Some(OperandSize::Qword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[220, 149, 33, 99], OperandSize::Word)
}

#[test]
fn fcom_8() {
    run_test(&Instruction { mnemonic: Mnemonic::FCOM, operand1: Some(Indirect(ESI, Some(OperandSize::Qword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[220, 22], OperandSize::Dword)
}

#[test]
fn fcom_9() {
    run_test(&Instruction { mnemonic: Mnemonic::FCOM, operand1: Some(IndirectScaledDisplaced(RDX, Four, 792684041, Some(OperandSize::Qword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[220, 20, 149, 9, 102, 63, 47], OperandSize::Qword)
}

