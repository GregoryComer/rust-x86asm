use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn blsmsk_1() {
    run_test(&Instruction { mnemonic: Mnemonic::BLSMSK, operand1: Some(Direct(ECX)), operand2: Some(Direct(EDI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 112, 243, 215], OperandSize::Dword)
}

#[test]
fn blsmsk_2() {
    run_test(&Instruction { mnemonic: Mnemonic::BLSMSK, operand1: Some(Direct(ECX)), operand2: Some(IndirectScaledIndexedDisplaced(ECX, EDX, Four, 847363692, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 112, 243, 148, 145, 108, 190, 129, 50], OperandSize::Dword)
}

#[test]
fn blsmsk_3() {
    run_test(&Instruction { mnemonic: Mnemonic::BLSMSK, operand1: Some(Direct(ESI)), operand2: Some(Direct(EDI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 72, 243, 215], OperandSize::Qword)
}

#[test]
fn blsmsk_4() {
    run_test(&Instruction { mnemonic: Mnemonic::BLSMSK, operand1: Some(Direct(ESP)), operand2: Some(Indirect(RSI, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 88, 243, 22], OperandSize::Qword)
}

#[test]
fn blsmsk_5() {
    run_test(&Instruction { mnemonic: Mnemonic::BLSMSK, operand1: Some(Direct(RDX)), operand2: Some(Direct(RDI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 232, 243, 215], OperandSize::Qword)
}

#[test]
fn blsmsk_6() {
    run_test(&Instruction { mnemonic: Mnemonic::BLSMSK, operand1: Some(Direct(RSI)), operand2: Some(IndirectScaledIndexed(RDI, RAX, Eight, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 200, 243, 20, 199], OperandSize::Qword)
}

