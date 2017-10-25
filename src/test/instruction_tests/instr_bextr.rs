use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn bextr_1() {
    run_test(&Instruction { mnemonic: Mnemonic::BEXTR, operand1: Some(Direct(EBP)), operand2: Some(Direct(EBX)), operand3: Some(Direct(EBX)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 96, 247, 235], OperandSize::Dword)
}

#[test]
fn bextr_2() {
    run_test(&Instruction { mnemonic: Mnemonic::BEXTR, operand1: Some(Direct(EBP)), operand2: Some(IndirectDisplaced(ECX, 919990480, Some(OperandSize::Dword), None)), operand3: Some(Direct(ESP)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 88, 247, 169, 208, 240, 213, 54], OperandSize::Dword)
}

#[test]
fn bextr_3() {
    run_test(&Instruction { mnemonic: Mnemonic::BEXTR, operand1: Some(Direct(ECX)), operand2: Some(Direct(ESI)), operand3: Some(Direct(ESI)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 72, 247, 206], OperandSize::Qword)
}

#[test]
fn bextr_4() {
    run_test(&Instruction { mnemonic: Mnemonic::BEXTR, operand1: Some(Direct(ESP)), operand2: Some(IndirectScaledDisplaced(RCX, Four, 528926666, Some(OperandSize::Dword), None)), operand3: Some(Direct(EDI)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 64, 247, 36, 141, 202, 199, 134, 31], OperandSize::Qword)
}

#[test]
fn bextr_5() {
    run_test(&Instruction { mnemonic: Mnemonic::BEXTR, operand1: Some(Direct(RSP)), operand2: Some(Direct(RDX)), operand3: Some(Direct(RBP)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 208, 247, 226], OperandSize::Qword)
}

#[test]
fn bextr_6() {
    run_test(&Instruction { mnemonic: Mnemonic::BEXTR, operand1: Some(Direct(RBX)), operand2: Some(Indirect(RAX, Some(OperandSize::Qword), None)), operand3: Some(Direct(RDI)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 192, 247, 24], OperandSize::Qword)
}

