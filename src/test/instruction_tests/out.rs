use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn out_1() {
    run_test(&Instruction { mnemonic: Mnemonic::OUT, operand1: Some(Literal8(109)), operand2: Some(Direct(AL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[230, 109], OperandSize::Word)
}

fn out_2() {
    run_test(&Instruction { mnemonic: Mnemonic::OUT, operand1: Some(Literal8(37)), operand2: Some(Direct(AL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[230, 37], OperandSize::Dword)
}

fn out_3() {
    run_test(&Instruction { mnemonic: Mnemonic::OUT, operand1: Some(Literal8(33)), operand2: Some(Direct(AL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[230, 33], OperandSize::Qword)
}

fn out_4() {
    run_test(&Instruction { mnemonic: Mnemonic::OUT, operand1: Some(Literal8(13)), operand2: Some(Direct(AX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[231, 13], OperandSize::Word)
}

fn out_5() {
    run_test(&Instruction { mnemonic: Mnemonic::OUT, operand1: Some(Literal8(13)), operand2: Some(Direct(AX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 231, 13], OperandSize::Dword)
}

fn out_6() {
    run_test(&Instruction { mnemonic: Mnemonic::OUT, operand1: Some(Literal8(3)), operand2: Some(Direct(AX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 231, 3], OperandSize::Qword)
}

fn out_7() {
    run_test(&Instruction { mnemonic: Mnemonic::OUT, operand1: Some(Literal8(49)), operand2: Some(Direct(EAX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 231, 49], OperandSize::Word)
}

fn out_8() {
    run_test(&Instruction { mnemonic: Mnemonic::OUT, operand1: Some(Literal8(91)), operand2: Some(Direct(EAX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[231, 91], OperandSize::Dword)
}

fn out_9() {
    run_test(&Instruction { mnemonic: Mnemonic::OUT, operand1: Some(Literal8(61)), operand2: Some(Direct(EAX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[231, 61], OperandSize::Qword)
}

fn out_10() {
    run_test(&Instruction { mnemonic: Mnemonic::OUT, operand1: Some(Direct(DX)), operand2: Some(Direct(AL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[238], OperandSize::Word)
}

fn out_11() {
    run_test(&Instruction { mnemonic: Mnemonic::OUT, operand1: Some(Direct(DX)), operand2: Some(Direct(AL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[238], OperandSize::Dword)
}

fn out_12() {
    run_test(&Instruction { mnemonic: Mnemonic::OUT, operand1: Some(Direct(DX)), operand2: Some(Direct(AL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[238], OperandSize::Qword)
}

fn out_13() {
    run_test(&Instruction { mnemonic: Mnemonic::OUT, operand1: Some(Direct(DX)), operand2: Some(Direct(AX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[239], OperandSize::Word)
}

fn out_14() {
    run_test(&Instruction { mnemonic: Mnemonic::OUT, operand1: Some(Direct(DX)), operand2: Some(Direct(AX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 239], OperandSize::Dword)
}

fn out_15() {
    run_test(&Instruction { mnemonic: Mnemonic::OUT, operand1: Some(Direct(DX)), operand2: Some(Direct(AX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 239], OperandSize::Qword)
}

fn out_16() {
    run_test(&Instruction { mnemonic: Mnemonic::OUT, operand1: Some(Direct(DX)), operand2: Some(Direct(EAX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 239], OperandSize::Word)
}

fn out_17() {
    run_test(&Instruction { mnemonic: Mnemonic::OUT, operand1: Some(Direct(DX)), operand2: Some(Direct(EAX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[239], OperandSize::Dword)
}

fn out_18() {
    run_test(&Instruction { mnemonic: Mnemonic::OUT, operand1: Some(Direct(DX)), operand2: Some(Direct(EAX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[239], OperandSize::Qword)
}

