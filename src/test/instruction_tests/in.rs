use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn in_1() {
    run_test(&Instruction { mnemonic: Mnemonic::IN, operand1: Some(Direct(AL)), operand2: Some(Literal8(127)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[228, 127], OperandSize::Word)
}

fn in_2() {
    run_test(&Instruction { mnemonic: Mnemonic::IN, operand1: Some(Direct(AL)), operand2: Some(Literal8(64)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[228, 64], OperandSize::Dword)
}

fn in_3() {
    run_test(&Instruction { mnemonic: Mnemonic::IN, operand1: Some(Direct(AL)), operand2: Some(Literal8(68)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[228, 68], OperandSize::Qword)
}

fn in_4() {
    run_test(&Instruction { mnemonic: Mnemonic::IN, operand1: Some(Direct(AX)), operand2: Some(Literal8(34)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[229, 34], OperandSize::Word)
}

fn in_5() {
    run_test(&Instruction { mnemonic: Mnemonic::IN, operand1: Some(Direct(AX)), operand2: Some(Literal8(117)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 229, 117], OperandSize::Dword)
}

fn in_6() {
    run_test(&Instruction { mnemonic: Mnemonic::IN, operand1: Some(Direct(AX)), operand2: Some(Literal8(6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 229, 6], OperandSize::Qword)
}

fn in_7() {
    run_test(&Instruction { mnemonic: Mnemonic::IN, operand1: Some(Direct(EAX)), operand2: Some(Literal8(79)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 229, 79], OperandSize::Word)
}

fn in_8() {
    run_test(&Instruction { mnemonic: Mnemonic::IN, operand1: Some(Direct(EAX)), operand2: Some(Literal8(56)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[229, 56], OperandSize::Dword)
}

fn in_9() {
    run_test(&Instruction { mnemonic: Mnemonic::IN, operand1: Some(Direct(EAX)), operand2: Some(Literal8(18)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[229, 18], OperandSize::Qword)
}

fn in_10() {
    run_test(&Instruction { mnemonic: Mnemonic::IN, operand1: Some(Direct(AL)), operand2: Some(Direct(DX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[236], OperandSize::Word)
}

fn in_11() {
    run_test(&Instruction { mnemonic: Mnemonic::IN, operand1: Some(Direct(AL)), operand2: Some(Direct(DX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[236], OperandSize::Dword)
}

fn in_12() {
    run_test(&Instruction { mnemonic: Mnemonic::IN, operand1: Some(Direct(AL)), operand2: Some(Direct(DX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[236], OperandSize::Qword)
}

fn in_13() {
    run_test(&Instruction { mnemonic: Mnemonic::IN, operand1: Some(Direct(AX)), operand2: Some(Direct(DX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[237], OperandSize::Word)
}

fn in_14() {
    run_test(&Instruction { mnemonic: Mnemonic::IN, operand1: Some(Direct(AX)), operand2: Some(Direct(DX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 237], OperandSize::Dword)
}

fn in_15() {
    run_test(&Instruction { mnemonic: Mnemonic::IN, operand1: Some(Direct(AX)), operand2: Some(Direct(DX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 237], OperandSize::Qword)
}

fn in_16() {
    run_test(&Instruction { mnemonic: Mnemonic::IN, operand1: Some(Direct(EAX)), operand2: Some(Direct(DX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 237], OperandSize::Word)
}

fn in_17() {
    run_test(&Instruction { mnemonic: Mnemonic::IN, operand1: Some(Direct(EAX)), operand2: Some(Direct(DX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[237], OperandSize::Dword)
}

fn in_18() {
    run_test(&Instruction { mnemonic: Mnemonic::IN, operand1: Some(Direct(EAX)), operand2: Some(Direct(DX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[237], OperandSize::Qword)
}

