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
    run_test(&Instruction { mnemonic: Mnemonic::PMULUDQ, operand1: Some(Direct(MM1)), operand2: Some(IndirectScaledIndexed(ECX, EBX, Eight, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 244, 12, 217], OperandSize::Dword)
}

#[test]
fn pmuludq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PMULUDQ, operand1: Some(Direct(MM5)), operand2: Some(Direct(MM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 244, 236], OperandSize::Qword)
}

#[test]
fn pmuludq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PMULUDQ, operand1: Some(Direct(MM3)), operand2: Some(IndirectScaledIndexed(RCX, RCX, Four, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 244, 28, 137], OperandSize::Qword)
}

#[test]
fn pmuludq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PMULUDQ, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 244, 222], OperandSize::Dword)
}

#[test]
fn pmuludq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PMULUDQ, operand1: Some(Direct(XMM3)), operand2: Some(IndirectScaledIndexedDisplaced(ECX, EDX, Two, 1441193641, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 244, 156, 81, 169, 222, 230, 85], OperandSize::Dword)
}

#[test]
fn pmuludq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PMULUDQ, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 244, 200], OperandSize::Qword)
}

#[test]
fn pmuludq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PMULUDQ, operand1: Some(Direct(XMM3)), operand2: Some(Indirect(RBX, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 244, 27], OperandSize::Qword)
}

