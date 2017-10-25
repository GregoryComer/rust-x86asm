use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn phaddd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PHADDD, operand1: Some(Direct(MM5)), operand2: Some(Direct(MM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 2, 235], OperandSize::Dword)
}

#[test]
fn phaddd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PHADDD, operand1: Some(Direct(MM5)), operand2: Some(Indirect(EAX, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 2, 40], OperandSize::Dword)
}

#[test]
fn phaddd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PHADDD, operand1: Some(Direct(MM5)), operand2: Some(Direct(MM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 2, 238], OperandSize::Qword)
}

#[test]
fn phaddd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PHADDD, operand1: Some(Direct(MM6)), operand2: Some(IndirectScaledIndexed(RAX, RAX, Two, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 2, 52, 64], OperandSize::Qword)
}

#[test]
fn phaddd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PHADDD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 2, 231], OperandSize::Dword)
}

#[test]
fn phaddd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PHADDD, operand1: Some(Direct(XMM4)), operand2: Some(IndirectScaledIndexed(EDX, EBX, Four, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 2, 36, 154], OperandSize::Dword)
}

#[test]
fn phaddd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PHADDD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 2, 228], OperandSize::Qword)
}

#[test]
fn phaddd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PHADDD, operand1: Some(Direct(XMM6)), operand2: Some(Indirect(RBX, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 2, 51], OperandSize::Qword)
}

