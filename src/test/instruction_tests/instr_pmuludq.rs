use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn pmuludq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PMULUDQ, operand1: Some(Direct(MM3)), operand2: Some(Direct(MM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 244, 222], OperandSize::Dword)
}

#[test]
fn pmuludq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PMULUDQ, operand1: Some(Direct(MM3)), operand2: Some(IndirectScaledDisplaced(ESI, Two, 1666482490, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 244, 28, 117, 58, 129, 84, 99], OperandSize::Dword)
}

#[test]
fn pmuludq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PMULUDQ, operand1: Some(Direct(MM6)), operand2: Some(Direct(MM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 244, 247], OperandSize::Qword)
}

#[test]
fn pmuludq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PMULUDQ, operand1: Some(Direct(MM4)), operand2: Some(IndirectScaledIndexed(RDI, RBX, Eight, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 244, 36, 223], OperandSize::Qword)
}

#[test]
fn pmuludq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PMULUDQ, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 244, 206], OperandSize::Dword)
}

#[test]
fn pmuludq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PMULUDQ, operand1: Some(Direct(XMM6)), operand2: Some(IndirectDisplaced(EBX, 849751838, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 244, 179, 30, 47, 166, 50], OperandSize::Dword)
}

#[test]
fn pmuludq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PMULUDQ, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 244, 200], OperandSize::Qword)
}

#[test]
fn pmuludq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PMULUDQ, operand1: Some(Direct(XMM0)), operand2: Some(IndirectDisplaced(RBX, 1334313890, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 244, 131, 162, 3, 136, 79], OperandSize::Qword)
}

