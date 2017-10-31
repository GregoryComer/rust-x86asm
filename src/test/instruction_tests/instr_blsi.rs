use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn blsi_1() {
    run_test(&Instruction { mnemonic: Mnemonic::BLSI, operand1: Some(Direct(ECX)), operand2: Some(Direct(EBP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 112, 243, 221], OperandSize::Dword)
}

#[test]
fn blsi_2() {
    run_test(&Instruction { mnemonic: Mnemonic::BLSI, operand1: Some(Direct(ECX)), operand2: Some(IndirectScaledDisplaced(EBX, Four, 294154846, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 112, 243, 28, 157, 94, 114, 136, 17], OperandSize::Dword)
}

#[test]
fn blsi_3() {
    run_test(&Instruction { mnemonic: Mnemonic::BLSI, operand1: Some(Direct(EBX)), operand2: Some(Direct(EDI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 96, 243, 223], OperandSize::Qword)
}

#[test]
fn blsi_4() {
    run_test(&Instruction { mnemonic: Mnemonic::BLSI, operand1: Some(Direct(EBX)), operand2: Some(IndirectScaledIndexedDisplaced(RBX, RSI, Two, 612659927, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 96, 243, 156, 115, 215, 114, 132, 36], OperandSize::Qword)
}

#[test]
fn blsi_5() {
    run_test(&Instruction { mnemonic: Mnemonic::BLSI, operand1: Some(Direct(RDX)), operand2: Some(Direct(RDX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 232, 243, 218], OperandSize::Qword)
}

#[test]
fn blsi_6() {
    run_test(&Instruction { mnemonic: Mnemonic::BLSI, operand1: Some(Direct(RDI)), operand2: Some(IndirectScaledIndexed(RSI, RAX, Four, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 192, 243, 28, 134], OperandSize::Qword)
}

