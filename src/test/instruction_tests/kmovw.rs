use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn kmovw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::KMOVW, operand1: Some(Direct(K1)), operand2: Some(Direct(K3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 144, 203], OperandSize::Dword)
}

fn kmovw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::KMOVW, operand1: Some(Direct(K3)), operand2: Some(Indirect(ESI, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 144, 30], OperandSize::Dword)
}

fn kmovw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::KMOVW, operand1: Some(Direct(K3)), operand2: Some(Direct(K1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 144, 217], OperandSize::Qword)
}

fn kmovw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::KMOVW, operand1: Some(Direct(K4)), operand2: Some(IndirectScaledIndexed(RAX, RCX, Eight, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 144, 36, 200], OperandSize::Qword)
}

fn kmovw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::KMOVW, operand1: Some(IndirectScaledDisplaced(ESI, Two, 5273165, Some(OperandSize::Word), None)), operand2: Some(Direct(K6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 145, 52, 117, 77, 118, 80, 0], OperandSize::Dword)
}

fn kmovw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::KMOVW, operand1: Some(Indirect(RSI, Some(OperandSize::Word), None)), operand2: Some(Direct(K4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 145, 38], OperandSize::Qword)
}

fn kmovw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::KMOVW, operand1: Some(Direct(K6)), operand2: Some(Direct(ESI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 146, 246], OperandSize::Dword)
}

fn kmovw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::KMOVW, operand1: Some(Direct(K6)), operand2: Some(Direct(ECX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 146, 241], OperandSize::Qword)
}

fn kmovw_9() {
    run_test(&Instruction { mnemonic: Mnemonic::KMOVW, operand1: Some(Direct(ESP)), operand2: Some(Direct(K2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 147, 226], OperandSize::Dword)
}

fn kmovw_10() {
    run_test(&Instruction { mnemonic: Mnemonic::KMOVW, operand1: Some(Direct(EDI)), operand2: Some(Direct(K3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 147, 251], OperandSize::Qword)
}

