use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn cvtsd2si_1() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTSD2SI, operand1: Some(Direct(ECX)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 45, 201], OperandSize::Dword)
}

#[test]
fn cvtsd2si_2() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTSD2SI, operand1: Some(Direct(EDX)), operand2: Some(IndirectDisplaced(EAX, 1975836944, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 45, 144, 16, 225, 196, 117], OperandSize::Dword)
}

#[test]
fn cvtsd2si_3() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTSD2SI, operand1: Some(Direct(EBX)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 45, 218], OperandSize::Qword)
}

#[test]
fn cvtsd2si_4() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTSD2SI, operand1: Some(Direct(ESI)), operand2: Some(IndirectScaledIndexedDisplaced(RBX, RDI, Four, 397721023, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 45, 180, 187, 191, 189, 180, 23], OperandSize::Qword)
}

#[test]
fn cvtsd2si_5() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTSD2SI, operand1: Some(Direct(RSI)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 72, 15, 45, 240], OperandSize::Qword)
}

#[test]
fn cvtsd2si_6() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTSD2SI, operand1: Some(Direct(RDI)), operand2: Some(IndirectScaledIndexedDisplaced(RBX, RAX, Eight, 492397815, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 72, 15, 45, 188, 195, 247, 100, 89, 29], OperandSize::Qword)
}

