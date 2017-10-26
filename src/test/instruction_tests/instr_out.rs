use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn out_1() {
    run_test(&Instruction { mnemonic: Mnemonic::OUT, operand1: Some(Literal8(45)), operand2: Some(Direct(AL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[230, 45], OperandSize::Word)
}

#[test]
fn out_2() {
    run_test(&Instruction { mnemonic: Mnemonic::OUT, operand1: Some(Literal8(69)), operand2: Some(Direct(AL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[230, 69], OperandSize::Dword)
}

#[test]
fn out_3() {
    run_test(&Instruction { mnemonic: Mnemonic::OUT, operand1: Some(Literal8(69)), operand2: Some(Direct(AL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[230, 69], OperandSize::Qword)
}

#[test]
fn out_4() {
    run_test(&Instruction { mnemonic: Mnemonic::OUT, operand1: Some(Literal8(47)), operand2: Some(Direct(AX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[231, 47], OperandSize::Word)
}

#[test]
fn out_5() {
    run_test(&Instruction { mnemonic: Mnemonic::OUT, operand1: Some(Literal8(27)), operand2: Some(Direct(AX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 231, 27], OperandSize::Dword)
}

#[test]
fn out_6() {
    run_test(&Instruction { mnemonic: Mnemonic::OUT, operand1: Some(Literal8(81)), operand2: Some(Direct(AX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 231, 81], OperandSize::Qword)
}

#[test]
fn out_7() {
    run_test(&Instruction { mnemonic: Mnemonic::OUT, operand1: Some(Literal8(89)), operand2: Some(Direct(EAX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 231, 89], OperandSize::Word)
}

#[test]
fn out_8() {
    run_test(&Instruction { mnemonic: Mnemonic::OUT, operand1: Some(Literal8(72)), operand2: Some(Direct(EAX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[231, 72], OperandSize::Dword)
}

#[test]
fn out_9() {
    run_test(&Instruction { mnemonic: Mnemonic::OUT, operand1: Some(Literal8(118)), operand2: Some(Direct(EAX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[231, 118], OperandSize::Qword)
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

