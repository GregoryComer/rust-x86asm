use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn pmuludq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PMULUDQ, operand1: Some(Direct(MM1)), operand2: Some(Direct(MM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 244, 202], OperandSize::Dword)
}

#[test]
fn pmuludq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PMULUDQ, operand1: Some(Direct(MM0)), operand2: Some(IndirectScaledDisplaced(EAX, Eight, 135056426, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 244, 4, 197, 42, 204, 12, 8], OperandSize::Dword)
}

#[test]
fn pmuludq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PMULUDQ, operand1: Some(Direct(MM1)), operand2: Some(Direct(MM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 244, 201], OperandSize::Qword)
}

#[test]
fn pmuludq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PMULUDQ, operand1: Some(Direct(MM1)), operand2: Some(Indirect(RSI, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 244, 14], OperandSize::Qword)
}

#[test]
fn pmuludq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PMULUDQ, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 244, 218], OperandSize::Dword)
}

#[test]
fn pmuludq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PMULUDQ, operand1: Some(Direct(XMM2)), operand2: Some(IndirectScaledDisplaced(EDX, Two, 1027045644, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 244, 20, 85, 12, 121, 55, 61], OperandSize::Dword)
}

#[test]
fn pmuludq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PMULUDQ, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 244, 210], OperandSize::Qword)
}

#[test]
fn pmuludq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PMULUDQ, operand1: Some(Direct(XMM3)), operand2: Some(IndirectScaledDisplaced(RCX, Two, 236269905, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 244, 28, 77, 81, 49, 21, 14], OperandSize::Qword)
}

