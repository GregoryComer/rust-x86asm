use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn in_1() {
    run_test(&Instruction { mnemonic: Mnemonic::IN, operand1: Some(Direct(AL)), operand2: Some(Literal8(127)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[228, 127], OperandSize::Word)
}

#[test]
fn in_2() {
    run_test(&Instruction { mnemonic: Mnemonic::IN, operand1: Some(Direct(AL)), operand2: Some(Literal8(105)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[228, 105], OperandSize::Dword)
}

#[test]
fn in_3() {
    run_test(&Instruction { mnemonic: Mnemonic::IN, operand1: Some(Direct(AL)), operand2: Some(Literal8(106)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[228, 106], OperandSize::Qword)
}

#[test]
fn in_4() {
    run_test(&Instruction { mnemonic: Mnemonic::IN, operand1: Some(Direct(AX)), operand2: Some(Literal8(70)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[229, 70], OperandSize::Word)
}

#[test]
fn in_5() {
    run_test(&Instruction { mnemonic: Mnemonic::IN, operand1: Some(Direct(AX)), operand2: Some(Literal8(40)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 229, 40], OperandSize::Dword)
}

#[test]
fn in_6() {
    run_test(&Instruction { mnemonic: Mnemonic::IN, operand1: Some(Direct(AX)), operand2: Some(Literal8(104)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 229, 104], OperandSize::Qword)
}

#[test]
fn in_7() {
    run_test(&Instruction { mnemonic: Mnemonic::IN, operand1: Some(Direct(EAX)), operand2: Some(Literal8(93)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 229, 93], OperandSize::Word)
}

#[test]
fn in_8() {
    run_test(&Instruction { mnemonic: Mnemonic::IN, operand1: Some(Direct(EAX)), operand2: Some(Literal8(108)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[229, 108], OperandSize::Dword)
}

#[test]
fn in_9() {
    run_test(&Instruction { mnemonic: Mnemonic::IN, operand1: Some(Direct(EAX)), operand2: Some(Literal8(59)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[229, 59], OperandSize::Qword)
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

