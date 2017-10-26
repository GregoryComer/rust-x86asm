use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn kmovw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::KMOVW, operand1: Some(Direct(K7)), operand2: Some(Direct(K7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 144, 255], OperandSize::Dword)
}

#[test]
fn kmovw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::KMOVW, operand1: Some(Direct(K5)), operand2: Some(IndirectDisplaced(ECX, 605692918, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 144, 169, 246, 35, 26, 36], OperandSize::Dword)
}

#[test]
fn kmovw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::KMOVW, operand1: Some(Direct(K4)), operand2: Some(Direct(K2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 144, 226], OperandSize::Qword)
}

#[test]
fn kmovw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::KMOVW, operand1: Some(Direct(K1)), operand2: Some(IndirectScaledIndexed(RAX, RAX, Eight, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 144, 12, 192], OperandSize::Qword)
}

#[test]
fn kmovw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::KMOVW, operand1: Some(IndirectScaledIndexed(ESI, EAX, Two, Some(OperandSize::Word), None)), operand2: Some(Direct(K3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 145, 28, 70], OperandSize::Dword)
}

#[test]
fn kmovw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::KMOVW, operand1: Some(IndirectScaledDisplaced(RSI, Two, 1078848048, Some(OperandSize::Word), None)), operand2: Some(Direct(K6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 145, 52, 117, 48, 234, 77, 64], OperandSize::Qword)
}

#[test]
fn kmovw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::KMOVW, operand1: Some(Direct(K5)), operand2: Some(Direct(EDI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 146, 239], OperandSize::Dword)
}

#[test]
fn kmovw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::KMOVW, operand1: Some(Direct(K1)), operand2: Some(Direct(EDI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 146, 207], OperandSize::Qword)
}

#[test]
fn kmovw_9() {
    run_test(&Instruction { mnemonic: Mnemonic::KMOVW, operand1: Some(Direct(EDI)), operand2: Some(Direct(K2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 147, 250], OperandSize::Dword)
}

#[test]
fn kmovw_10() {
    run_test(&Instruction { mnemonic: Mnemonic::KMOVW, operand1: Some(Direct(EBX)), operand2: Some(Direct(K6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 147, 222], OperandSize::Qword)
}

