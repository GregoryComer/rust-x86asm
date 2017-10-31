use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn out_1() {
    run_test(&Instruction { mnemonic: Mnemonic::OUT, operand1: Some(Literal8(97)), operand2: Some(Direct(AL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[230, 97], OperandSize::Word)
}

#[test]
fn out_2() {
    run_test(&Instruction { mnemonic: Mnemonic::OUT, operand1: Some(Literal8(113)), operand2: Some(Direct(AL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[230, 113], OperandSize::Dword)
}

#[test]
fn out_3() {
    run_test(&Instruction { mnemonic: Mnemonic::OUT, operand1: Some(Literal8(21)), operand2: Some(Direct(AL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[230, 21], OperandSize::Qword)
}

#[test]
fn out_4() {
    run_test(&Instruction { mnemonic: Mnemonic::OUT, operand1: Some(Literal8(22)), operand2: Some(Direct(AX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[231, 22], OperandSize::Word)
}

#[test]
fn out_5() {
    run_test(&Instruction { mnemonic: Mnemonic::OUT, operand1: Some(Literal8(70)), operand2: Some(Direct(AX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 231, 70], OperandSize::Dword)
}

#[test]
fn out_6() {
    run_test(&Instruction { mnemonic: Mnemonic::OUT, operand1: Some(Literal8(72)), operand2: Some(Direct(AX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 231, 72], OperandSize::Qword)
}

#[test]
fn out_7() {
    run_test(&Instruction { mnemonic: Mnemonic::OUT, operand1: Some(Literal8(64)), operand2: Some(Direct(EAX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 231, 64], OperandSize::Word)
}

#[test]
fn out_8() {
    run_test(&Instruction { mnemonic: Mnemonic::OUT, operand1: Some(Literal8(76)), operand2: Some(Direct(EAX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[231, 76], OperandSize::Dword)
}

#[test]
fn out_9() {
    run_test(&Instruction { mnemonic: Mnemonic::OUT, operand1: Some(Literal8(75)), operand2: Some(Direct(EAX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[231, 75], OperandSize::Qword)
}

#[test]
fn out_10() {
    run_test(&Instruction { mnemonic: Mnemonic::OUT, operand1: Some(Direct(DX)), operand2: Some(Direct(AL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[238], OperandSize::Word)
}

#[test]
fn out_11() {
    run_test(&Instruction { mnemonic: Mnemonic::OUT, operand1: Some(Direct(DX)), operand2: Some(Direct(AL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[238], OperandSize::Dword)
}

#[test]
fn out_12() {
    run_test(&Instruction { mnemonic: Mnemonic::OUT, operand1: Some(Direct(DX)), operand2: Some(Direct(AL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[238], OperandSize::Qword)
}

#[test]
fn out_13() {
    run_test(&Instruction { mnemonic: Mnemonic::OUT, operand1: Some(Direct(DX)), operand2: Some(Direct(AX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[239], OperandSize::Word)
}

#[test]
fn out_14() {
    run_test(&Instruction { mnemonic: Mnemonic::OUT, operand1: Some(Direct(DX)), operand2: Some(Direct(AX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 239], OperandSize::Dword)
}

#[test]
fn out_15() {
    run_test(&Instruction { mnemonic: Mnemonic::OUT, operand1: Some(Direct(DX)), operand2: Some(Direct(AX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 239], OperandSize::Qword)
}

#[test]
fn out_16() {
    run_test(&Instruction { mnemonic: Mnemonic::OUT, operand1: Some(Direct(DX)), operand2: Some(Direct(EAX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 239], OperandSize::Word)
}

#[test]
fn out_17() {
    run_test(&Instruction { mnemonic: Mnemonic::OUT, operand1: Some(Direct(DX)), operand2: Some(Direct(EAX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[239], OperandSize::Dword)
}

#[test]
fn out_18() {
    run_test(&Instruction { mnemonic: Mnemonic::OUT, operand1: Some(Direct(DX)), operand2: Some(Direct(EAX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[239], OperandSize::Qword)
}

