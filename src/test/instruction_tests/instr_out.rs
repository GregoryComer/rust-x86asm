use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn out_1() {
    run_test(&Instruction { mnemonic: Mnemonic::OUT, operand1: Some(Literal8(4)), operand2: Some(Direct(AL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[230, 4], OperandSize::Word)
}

#[test]
fn out_2() {
    run_test(&Instruction { mnemonic: Mnemonic::OUT, operand1: Some(Literal8(47)), operand2: Some(Direct(AL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[230, 47], OperandSize::Dword)
}

#[test]
fn out_3() {
    run_test(&Instruction { mnemonic: Mnemonic::OUT, operand1: Some(Literal8(0)), operand2: Some(Direct(AL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[230, 0], OperandSize::Qword)
}

#[test]
fn out_4() {
    run_test(&Instruction { mnemonic: Mnemonic::OUT, operand1: Some(Literal8(19)), operand2: Some(Direct(AX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[231, 19], OperandSize::Word)
}

#[test]
fn out_5() {
    run_test(&Instruction { mnemonic: Mnemonic::OUT, operand1: Some(Literal8(16)), operand2: Some(Direct(AX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 231, 16], OperandSize::Dword)
}

#[test]
fn out_6() {
    run_test(&Instruction { mnemonic: Mnemonic::OUT, operand1: Some(Literal8(87)), operand2: Some(Direct(AX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 231, 87], OperandSize::Qword)
}

#[test]
fn out_7() {
    run_test(&Instruction { mnemonic: Mnemonic::OUT, operand1: Some(Literal8(93)), operand2: Some(Direct(EAX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 231, 93], OperandSize::Word)
}

#[test]
fn out_8() {
    run_test(&Instruction { mnemonic: Mnemonic::OUT, operand1: Some(Literal8(55)), operand2: Some(Direct(EAX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[231, 55], OperandSize::Dword)
}

#[test]
fn out_9() {
    run_test(&Instruction { mnemonic: Mnemonic::OUT, operand1: Some(Literal8(61)), operand2: Some(Direct(EAX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[231, 61], OperandSize::Qword)
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

