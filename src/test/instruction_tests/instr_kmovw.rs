use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn kmovw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::KMOVW, operand1: Some(Direct(K4)), operand2: Some(Direct(K1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 144, 225], OperandSize::Dword)
}

#[test]
fn kmovw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::KMOVW, operand1: Some(Direct(K7)), operand2: Some(IndirectScaledDisplaced(ECX, Eight, 488375136, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 144, 60, 205, 96, 3, 28, 29], OperandSize::Dword)
}

#[test]
fn kmovw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::KMOVW, operand1: Some(Direct(K6)), operand2: Some(Direct(K7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 144, 247], OperandSize::Qword)
}

#[test]
fn kmovw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::KMOVW, operand1: Some(Direct(K3)), operand2: Some(Indirect(RSI, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 144, 30], OperandSize::Qword)
}

#[test]
fn kmovw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::KMOVW, operand1: Some(IndirectScaledIndexed(EAX, EAX, Two, Some(OperandSize::Word), None)), operand2: Some(Direct(K7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 145, 60, 64], OperandSize::Dword)
}

#[test]
fn kmovw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::KMOVW, operand1: Some(IndirectScaledIndexed(RBX, RAX, Two, Some(OperandSize::Word), None)), operand2: Some(Direct(K2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 145, 20, 67], OperandSize::Qword)
}

#[test]
fn kmovw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::KMOVW, operand1: Some(Direct(K1)), operand2: Some(Direct(EBX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 146, 203], OperandSize::Dword)
}

#[test]
fn kmovw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::KMOVW, operand1: Some(Direct(K6)), operand2: Some(Direct(EBP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 146, 245], OperandSize::Qword)
}

#[test]
fn kmovw_9() {
    run_test(&Instruction { mnemonic: Mnemonic::KMOVW, operand1: Some(Direct(ESP)), operand2: Some(Direct(K2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 147, 226], OperandSize::Dword)
}

#[test]
fn kmovw_10() {
    run_test(&Instruction { mnemonic: Mnemonic::KMOVW, operand1: Some(Direct(ESI)), operand2: Some(Direct(K6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 147, 246], OperandSize::Qword)
}

