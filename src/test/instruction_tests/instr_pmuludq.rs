use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn pmuludq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PMULUDQ, operand1: Some(Direct(MM3)), operand2: Some(Direct(MM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 244, 216], OperandSize::Dword)
}

#[test]
fn pmuludq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PMULUDQ, operand1: Some(Direct(MM7)), operand2: Some(IndirectScaledIndexed(EBX, EDI, Two, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 244, 60, 123], OperandSize::Dword)
}

#[test]
fn pmuludq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PMULUDQ, operand1: Some(Direct(MM7)), operand2: Some(Direct(MM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 244, 252], OperandSize::Qword)
}

#[test]
fn pmuludq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PMULUDQ, operand1: Some(Direct(MM2)), operand2: Some(IndirectScaledDisplaced(RCX, Four, 2087636627, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 244, 20, 141, 147, 206, 110, 124], OperandSize::Qword)
}

#[test]
fn pmuludq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PMULUDQ, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 244, 200], OperandSize::Dword)
}

#[test]
fn pmuludq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PMULUDQ, operand1: Some(Direct(XMM3)), operand2: Some(IndirectScaledIndexedDisplaced(EBX, EDX, Four, 382724529, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 244, 156, 147, 177, 233, 207, 22], OperandSize::Dword)
}

#[test]
fn pmuludq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PMULUDQ, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 244, 209], OperandSize::Qword)
}

#[test]
fn pmuludq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PMULUDQ, operand1: Some(Direct(XMM4)), operand2: Some(IndirectScaledIndexed(RBX, RDI, Two, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 244, 36, 123], OperandSize::Qword)
}

