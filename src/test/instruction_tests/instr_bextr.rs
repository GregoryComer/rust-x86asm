use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn bextr_1() {
    run_test(&Instruction { mnemonic: Mnemonic::BEXTR, operand1: Some(Direct(EDX)), operand2: Some(Direct(EBP)), operand3: Some(Direct(EDI)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 64, 247, 213], OperandSize::Dword)
}

#[test]
fn bextr_2() {
    run_test(&Instruction { mnemonic: Mnemonic::BEXTR, operand1: Some(Direct(EDX)), operand2: Some(Indirect(ESI, Some(OperandSize::Dword), None)), operand3: Some(Direct(EDX)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 104, 247, 22], OperandSize::Dword)
}

#[test]
fn bextr_3() {
    run_test(&Instruction { mnemonic: Mnemonic::BEXTR, operand1: Some(Direct(ESI)), operand2: Some(Direct(EBP)), operand3: Some(Direct(ESP)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 88, 247, 245], OperandSize::Qword)
}

#[test]
fn bextr_4() {
    run_test(&Instruction { mnemonic: Mnemonic::BEXTR, operand1: Some(Direct(EDX)), operand2: Some(IndirectScaledIndexed(RDX, RDX, Eight, Some(OperandSize::Dword), None)), operand3: Some(Direct(ESP)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 88, 247, 20, 210], OperandSize::Qword)
}

#[test]
fn bextr_5() {
    run_test(&Instruction { mnemonic: Mnemonic::BEXTR, operand1: Some(Direct(RSI)), operand2: Some(Direct(RSP)), operand3: Some(Direct(RSI)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 200, 247, 244], OperandSize::Qword)
}

#[test]
fn bextr_6() {
    run_test(&Instruction { mnemonic: Mnemonic::BEXTR, operand1: Some(Direct(RBX)), operand2: Some(IndirectDisplaced(RCX, 10949294, Some(OperandSize::Qword), None)), operand3: Some(Direct(RDI)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 192, 247, 153, 174, 18, 167, 0], OperandSize::Qword)
}

