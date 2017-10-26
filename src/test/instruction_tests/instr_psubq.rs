use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn psubq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PSUBQ, operand1: Some(Direct(MM5)), operand2: Some(Direct(MM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 251, 235], OperandSize::Dword)
}

#[test]
fn psubq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PSUBQ, operand1: Some(Direct(MM0)), operand2: Some(IndirectDisplaced(EAX, 1615043623, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 251, 128, 39, 156, 67, 96], OperandSize::Dword)
}

#[test]
fn psubq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PSUBQ, operand1: Some(Direct(MM4)), operand2: Some(Direct(MM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 251, 224], OperandSize::Qword)
}

#[test]
fn psubq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PSUBQ, operand1: Some(Direct(MM4)), operand2: Some(Indirect(RSI, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 251, 38], OperandSize::Qword)
}

#[test]
fn psubq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PSUBQ, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 251, 201], OperandSize::Dword)
}

#[test]
fn psubq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PSUBQ, operand1: Some(Direct(XMM5)), operand2: Some(IndirectScaledIndexedDisplaced(ECX, EAX, Eight, 1492932117, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 251, 172, 193, 21, 86, 252, 88], OperandSize::Dword)
}

#[test]
fn psubq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PSUBQ, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 251, 204], OperandSize::Qword)
}

#[test]
fn psubq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PSUBQ, operand1: Some(Direct(XMM5)), operand2: Some(IndirectScaledIndexed(RBX, RDX, Two, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 251, 44, 83], OperandSize::Qword)
}

