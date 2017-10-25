use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn blsi_1() {
    run_test(&Instruction { mnemonic: Mnemonic::BLSI, operand1: Some(Direct(ESP)), operand2: Some(Direct(ESP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 88, 243, 220], OperandSize::Dword)
}

#[test]
fn blsi_2() {
    run_test(&Instruction { mnemonic: Mnemonic::BLSI, operand1: Some(Direct(ESI)), operand2: Some(IndirectScaledIndexedDisplaced(EBX, EAX, Eight, 1187226501, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 72, 243, 156, 195, 133, 163, 195, 70], OperandSize::Dword)
}

#[test]
fn blsi_3() {
    run_test(&Instruction { mnemonic: Mnemonic::BLSI, operand1: Some(Direct(EBX)), operand2: Some(Direct(EDX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 96, 243, 218], OperandSize::Qword)
}

#[test]
fn blsi_4() {
    run_test(&Instruction { mnemonic: Mnemonic::BLSI, operand1: Some(Direct(ESP)), operand2: Some(IndirectDisplaced(RCX, 1816691468, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 88, 243, 153, 12, 131, 72, 108], OperandSize::Qword)
}

#[test]
fn blsi_5() {
    run_test(&Instruction { mnemonic: Mnemonic::BLSI, operand1: Some(Direct(RSI)), operand2: Some(Direct(RBX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 200, 243, 219], OperandSize::Qword)
}

#[test]
fn blsi_6() {
    run_test(&Instruction { mnemonic: Mnemonic::BLSI, operand1: Some(Direct(RBX)), operand2: Some(IndirectScaledDisplaced(RAX, Two, 75840422, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 224, 243, 28, 69, 166, 59, 133, 4], OperandSize::Qword)
}

