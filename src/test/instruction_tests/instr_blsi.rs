use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn blsi_1() {
    run_test(&Instruction { mnemonic: Mnemonic::BLSI, operand1: Some(Direct(ESI)), operand2: Some(Direct(EDX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 72, 243, 218], OperandSize::Dword)
}

#[test]
fn blsi_2() {
    run_test(&Instruction { mnemonic: Mnemonic::BLSI, operand1: Some(Direct(EDX)), operand2: Some(Indirect(EAX, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 104, 243, 24], OperandSize::Dword)
}

#[test]
fn blsi_3() {
    run_test(&Instruction { mnemonic: Mnemonic::BLSI, operand1: Some(Direct(EBP)), operand2: Some(Direct(EDI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 80, 243, 223], OperandSize::Qword)
}

#[test]
fn blsi_4() {
    run_test(&Instruction { mnemonic: Mnemonic::BLSI, operand1: Some(Direct(EBX)), operand2: Some(IndirectScaledIndexedDisplaced(RDI, RSI, Four, 1498404587, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 96, 243, 156, 183, 235, 214, 79, 89], OperandSize::Qword)
}

#[test]
fn blsi_5() {
    run_test(&Instruction { mnemonic: Mnemonic::BLSI, operand1: Some(Direct(RSP)), operand2: Some(Direct(RDI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 216, 243, 223], OperandSize::Qword)
}

#[test]
fn blsi_6() {
    run_test(&Instruction { mnemonic: Mnemonic::BLSI, operand1: Some(Direct(RCX)), operand2: Some(Indirect(RCX, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 240, 243, 25], OperandSize::Qword)
}

