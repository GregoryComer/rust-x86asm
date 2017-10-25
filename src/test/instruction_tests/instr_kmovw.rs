use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn kmovw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::KMOVW, operand1: Some(Direct(K7)), operand2: Some(Direct(K3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 144, 251], OperandSize::Dword)
}

#[test]
fn kmovw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::KMOVW, operand1: Some(Direct(K6)), operand2: Some(IndirectScaledIndexed(EDX, EDI, Eight, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 144, 52, 250], OperandSize::Dword)
}

#[test]
fn kmovw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::KMOVW, operand1: Some(Direct(K7)), operand2: Some(Direct(K1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 144, 249], OperandSize::Qword)
}

#[test]
fn kmovw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::KMOVW, operand1: Some(Direct(K3)), operand2: Some(IndirectScaledIndexedDisplaced(RAX, RCX, Two, 754339786, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 144, 156, 72, 202, 79, 246, 44], OperandSize::Qword)
}

#[test]
fn kmovw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::KMOVW, operand1: Some(IndirectDisplaced(EBX, 329936002, Some(OperandSize::Word), None)), operand2: Some(Direct(K7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 145, 187, 130, 108, 170, 19], OperandSize::Dword)
}

#[test]
fn kmovw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::KMOVW, operand1: Some(IndirectScaledDisplaced(RBX, Eight, 1808473269, Some(OperandSize::Word), None)), operand2: Some(Direct(K4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 145, 36, 221, 181, 28, 203, 107], OperandSize::Qword)
}

#[test]
fn kmovw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::KMOVW, operand1: Some(Direct(K7)), operand2: Some(Direct(EBP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 146, 253], OperandSize::Dword)
}

#[test]
fn kmovw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::KMOVW, operand1: Some(Direct(K3)), operand2: Some(Direct(EBX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 146, 219], OperandSize::Qword)
}

#[test]
fn kmovw_9() {
    run_test(&Instruction { mnemonic: Mnemonic::KMOVW, operand1: Some(Direct(ESI)), operand2: Some(Direct(K2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 147, 242], OperandSize::Dword)
}

#[test]
fn kmovw_10() {
    run_test(&Instruction { mnemonic: Mnemonic::KMOVW, operand1: Some(Direct(ESP)), operand2: Some(Direct(K1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 147, 225], OperandSize::Qword)
}

