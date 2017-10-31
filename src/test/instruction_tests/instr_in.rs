use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn in_1() {
    run_test(&Instruction { mnemonic: Mnemonic::IN, operand1: Some(Direct(AL)), operand2: Some(Literal8(88)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[228, 88], OperandSize::Word)
}

#[test]
fn in_2() {
    run_test(&Instruction { mnemonic: Mnemonic::IN, operand1: Some(Direct(AL)), operand2: Some(Literal8(84)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[228, 84], OperandSize::Dword)
}

#[test]
fn in_3() {
    run_test(&Instruction { mnemonic: Mnemonic::IN, operand1: Some(Direct(AL)), operand2: Some(Literal8(114)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[228, 114], OperandSize::Qword)
}

#[test]
fn in_4() {
    run_test(&Instruction { mnemonic: Mnemonic::IN, operand1: Some(Direct(AX)), operand2: Some(Literal8(99)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[229, 99], OperandSize::Word)
}

#[test]
fn in_5() {
    run_test(&Instruction { mnemonic: Mnemonic::IN, operand1: Some(Direct(AX)), operand2: Some(Literal8(43)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 229, 43], OperandSize::Dword)
}

#[test]
fn in_6() {
    run_test(&Instruction { mnemonic: Mnemonic::IN, operand1: Some(Direct(AX)), operand2: Some(Literal8(22)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 229, 22], OperandSize::Qword)
}

#[test]
fn in_7() {
    run_test(&Instruction { mnemonic: Mnemonic::IN, operand1: Some(Direct(EAX)), operand2: Some(Literal8(90)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 229, 90], OperandSize::Word)
}

#[test]
fn in_8() {
    run_test(&Instruction { mnemonic: Mnemonic::IN, operand1: Some(Direct(EAX)), operand2: Some(Literal8(52)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[229, 52], OperandSize::Dword)
}

#[test]
fn in_9() {
    run_test(&Instruction { mnemonic: Mnemonic::IN, operand1: Some(Direct(EAX)), operand2: Some(Literal8(55)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[229, 55], OperandSize::Qword)
}

#[test]
fn in_10() {
    run_test(&Instruction { mnemonic: Mnemonic::IN, operand1: Some(Direct(AL)), operand2: Some(Direct(DX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[236], OperandSize::Word)
}

#[test]
fn in_11() {
    run_test(&Instruction { mnemonic: Mnemonic::IN, operand1: Some(Direct(AL)), operand2: Some(Direct(DX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[236], OperandSize::Dword)
}

#[test]
fn in_12() {
    run_test(&Instruction { mnemonic: Mnemonic::IN, operand1: Some(Direct(AL)), operand2: Some(Direct(DX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[236], OperandSize::Qword)
}

#[test]
fn in_13() {
    run_test(&Instruction { mnemonic: Mnemonic::IN, operand1: Some(Direct(AX)), operand2: Some(Direct(DX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[237], OperandSize::Word)
}

#[test]
fn in_14() {
    run_test(&Instruction { mnemonic: Mnemonic::IN, operand1: Some(Direct(AX)), operand2: Some(Direct(DX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 237], OperandSize::Dword)
}

#[test]
fn in_15() {
    run_test(&Instruction { mnemonic: Mnemonic::IN, operand1: Some(Direct(AX)), operand2: Some(Direct(DX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 237], OperandSize::Qword)
}

#[test]
fn in_16() {
    run_test(&Instruction { mnemonic: Mnemonic::IN, operand1: Some(Direct(EAX)), operand2: Some(Direct(DX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 237], OperandSize::Word)
}

#[test]
fn in_17() {
    run_test(&Instruction { mnemonic: Mnemonic::IN, operand1: Some(Direct(EAX)), operand2: Some(Direct(DX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[237], OperandSize::Dword)
}

#[test]
fn in_18() {
    run_test(&Instruction { mnemonic: Mnemonic::IN, operand1: Some(Direct(EAX)), operand2: Some(Direct(DX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[237], OperandSize::Qword)
}

