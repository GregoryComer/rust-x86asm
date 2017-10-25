use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn blsmsk_1() {
    run_test(&Instruction { mnemonic: Mnemonic::BLSMSK, operand1: Some(Direct(ECX)), operand2: Some(Direct(ECX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 112, 243, 209], OperandSize::Dword)
}

#[test]
fn blsmsk_2() {
    run_test(&Instruction { mnemonic: Mnemonic::BLSMSK, operand1: Some(Direct(ECX)), operand2: Some(Indirect(EAX, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 112, 243, 16], OperandSize::Dword)
}

#[test]
fn blsmsk_3() {
    run_test(&Instruction { mnemonic: Mnemonic::BLSMSK, operand1: Some(Direct(EDI)), operand2: Some(Direct(EDI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 64, 243, 215], OperandSize::Qword)
}

#[test]
fn blsmsk_4() {
    run_test(&Instruction { mnemonic: Mnemonic::BLSMSK, operand1: Some(Direct(ESI)), operand2: Some(Indirect(RCX, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 72, 243, 17], OperandSize::Qword)
}

#[test]
fn blsmsk_5() {
    run_test(&Instruction { mnemonic: Mnemonic::BLSMSK, operand1: Some(Direct(RBP)), operand2: Some(Direct(RBP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 208, 243, 213], OperandSize::Qword)
}

#[test]
fn blsmsk_6() {
    run_test(&Instruction { mnemonic: Mnemonic::BLSMSK, operand1: Some(Direct(RBX)), operand2: Some(IndirectScaledIndexedDisplaced(RAX, RCX, Four, 1569684702, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 224, 243, 148, 136, 222, 124, 143, 93], OperandSize::Qword)
}

