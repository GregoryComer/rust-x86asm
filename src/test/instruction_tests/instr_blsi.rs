use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn blsi_1() {
    run_test(&Instruction { mnemonic: Mnemonic::BLSI, operand1: Some(Direct(ESI)), operand2: Some(Direct(ESI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 72, 243, 222], OperandSize::Dword)
}

#[test]
fn blsi_2() {
    run_test(&Instruction { mnemonic: Mnemonic::BLSI, operand1: Some(Direct(EBX)), operand2: Some(Indirect(EDX, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 96, 243, 26], OperandSize::Dword)
}

#[test]
fn blsi_3() {
    run_test(&Instruction { mnemonic: Mnemonic::BLSI, operand1: Some(Direct(EBP)), operand2: Some(Direct(ESI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 80, 243, 222], OperandSize::Qword)
}

#[test]
fn blsi_4() {
    run_test(&Instruction { mnemonic: Mnemonic::BLSI, operand1: Some(Direct(ESI)), operand2: Some(Indirect(RDX, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 72, 243, 26], OperandSize::Qword)
}

#[test]
fn blsi_5() {
    run_test(&Instruction { mnemonic: Mnemonic::BLSI, operand1: Some(Direct(RSP)), operand2: Some(Direct(RBP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 216, 243, 221], OperandSize::Qword)
}

#[test]
fn blsi_6() {
    run_test(&Instruction { mnemonic: Mnemonic::BLSI, operand1: Some(Direct(RDX)), operand2: Some(IndirectScaledIndexedDisplaced(RAX, RCX, Eight, 292890079, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 232, 243, 156, 200, 223, 37, 117, 17], OperandSize::Qword)
}

