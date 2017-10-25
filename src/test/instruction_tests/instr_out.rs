use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn out_1() {
    run_test(&Instruction { mnemonic: Mnemonic::OUT, operand1: Some(Literal8(74)), operand2: Some(Direct(AL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[230, 74], OperandSize::Word)
}

#[test]
fn out_2() {
    run_test(&Instruction { mnemonic: Mnemonic::OUT, operand1: Some(Literal8(123)), operand2: Some(Direct(AL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[230, 123], OperandSize::Dword)
}

#[test]
fn out_3() {
    run_test(&Instruction { mnemonic: Mnemonic::OUT, operand1: Some(Literal8(31)), operand2: Some(Direct(AL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[230, 31], OperandSize::Qword)
}

#[test]
fn out_4() {
    run_test(&Instruction { mnemonic: Mnemonic::OUT, operand1: Some(Literal8(101)), operand2: Some(Direct(AX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[231, 101], OperandSize::Word)
}

#[test]
fn out_5() {
    run_test(&Instruction { mnemonic: Mnemonic::OUT, operand1: Some(Literal8(124)), operand2: Some(Direct(AX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 231, 124], OperandSize::Dword)
}

#[test]
fn out_6() {
    run_test(&Instruction { mnemonic: Mnemonic::OUT, operand1: Some(Literal8(115)), operand2: Some(Direct(AX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 231, 115], OperandSize::Qword)
}

#[test]
fn out_7() {
    run_test(&Instruction { mnemonic: Mnemonic::OUT, operand1: Some(Literal8(124)), operand2: Some(Direct(EAX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 231, 124], OperandSize::Word)
}

#[test]
fn out_8() {
    run_test(&Instruction { mnemonic: Mnemonic::OUT, operand1: Some(Literal8(81)), operand2: Some(Direct(EAX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[231, 81], OperandSize::Dword)
}

#[test]
fn out_9() {
    run_test(&Instruction { mnemonic: Mnemonic::OUT, operand1: Some(Literal8(103)), operand2: Some(Direct(EAX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[231, 103], OperandSize::Qword)
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

