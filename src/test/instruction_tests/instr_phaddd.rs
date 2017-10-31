use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn phaddd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PHADDD, operand1: Some(Direct(MM6)), operand2: Some(Direct(MM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 2, 243], OperandSize::Dword)
}

#[test]
fn phaddd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PHADDD, operand1: Some(Direct(MM7)), operand2: Some(IndirectScaledIndexed(EAX, EDX, Two, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 2, 60, 80], OperandSize::Dword)
}

#[test]
fn phaddd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PHADDD, operand1: Some(Direct(MM1)), operand2: Some(Direct(MM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 2, 203], OperandSize::Qword)
}

#[test]
fn phaddd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PHADDD, operand1: Some(Direct(MM2)), operand2: Some(IndirectScaledDisplaced(RDX, Eight, 443638471, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 2, 20, 213, 199, 98, 113, 26], OperandSize::Qword)
}

#[test]
fn phaddd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PHADDD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 2, 212], OperandSize::Dword)
}

#[test]
fn phaddd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PHADDD, operand1: Some(Direct(XMM4)), operand2: Some(IndirectDisplaced(EDX, 1088618852, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 2, 162, 100, 1, 227, 64], OperandSize::Dword)
}

#[test]
fn phaddd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PHADDD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 2, 211], OperandSize::Qword)
}

#[test]
fn phaddd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PHADDD, operand1: Some(Direct(XMM1)), operand2: Some(IndirectScaledDisplaced(RCX, Four, 827951073, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 2, 12, 141, 225, 135, 89, 49], OperandSize::Qword)
}

