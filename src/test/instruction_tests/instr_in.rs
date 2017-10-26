use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn in_1() {
    run_test(&Instruction { mnemonic: Mnemonic::IN, operand1: Some(Direct(AL)), operand2: Some(Literal8(26)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[228, 26], OperandSize::Word)
}

#[test]
fn in_2() {
    run_test(&Instruction { mnemonic: Mnemonic::IN, operand1: Some(Direct(AL)), operand2: Some(Literal8(69)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[228, 69], OperandSize::Dword)
}

#[test]
fn in_3() {
    run_test(&Instruction { mnemonic: Mnemonic::IN, operand1: Some(Direct(AL)), operand2: Some(Literal8(60)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[228, 60], OperandSize::Qword)
}

#[test]
fn in_4() {
    run_test(&Instruction { mnemonic: Mnemonic::IN, operand1: Some(Direct(AX)), operand2: Some(Literal8(16)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[229, 16], OperandSize::Word)
}

#[test]
fn in_5() {
    run_test(&Instruction { mnemonic: Mnemonic::IN, operand1: Some(Direct(AX)), operand2: Some(Literal8(23)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 229, 23], OperandSize::Dword)
}

#[test]
fn in_6() {
    run_test(&Instruction { mnemonic: Mnemonic::IN, operand1: Some(Direct(AX)), operand2: Some(Literal8(31)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 229, 31], OperandSize::Qword)
}

#[test]
fn in_7() {
    run_test(&Instruction { mnemonic: Mnemonic::IN, operand1: Some(Direct(EAX)), operand2: Some(Literal8(30)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 229, 30], OperandSize::Word)
}

#[test]
fn in_8() {
    run_test(&Instruction { mnemonic: Mnemonic::IN, operand1: Some(Direct(EAX)), operand2: Some(Literal8(2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[229, 2], OperandSize::Dword)
}

#[test]
fn in_9() {
    run_test(&Instruction { mnemonic: Mnemonic::IN, operand1: Some(Direct(EAX)), operand2: Some(Literal8(84)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[229, 84], OperandSize::Qword)
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

