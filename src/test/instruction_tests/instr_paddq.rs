use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn paddq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PADDQ, operand1: Some(Direct(MM2)), operand2: Some(Direct(MM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 212, 213], OperandSize::Dword)
}

#[test]
fn paddq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PADDQ, operand1: Some(Direct(MM5)), operand2: Some(IndirectScaledIndexedDisplaced(EAX, EBX, Eight, 188960648, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 212, 172, 216, 136, 79, 67, 11], OperandSize::Dword)
}

#[test]
fn paddq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PADDQ, operand1: Some(Direct(MM5)), operand2: Some(Direct(MM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 212, 232], OperandSize::Qword)
}

#[test]
fn paddq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PADDQ, operand1: Some(Direct(MM2)), operand2: Some(IndirectScaledIndexed(RBX, RSI, Eight, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 212, 20, 243], OperandSize::Qword)
}

#[test]
fn paddq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PADDQ, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 212, 248], OperandSize::Dword)
}

#[test]
fn paddq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PADDQ, operand1: Some(Direct(XMM7)), operand2: Some(IndirectScaledDisplaced(EAX, Two, 201929700, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 212, 60, 69, 228, 51, 9, 12], OperandSize::Dword)
}

#[test]
fn paddq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PADDQ, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 212, 246], OperandSize::Qword)
}

#[test]
fn paddq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PADDQ, operand1: Some(Direct(XMM5)), operand2: Some(IndirectDisplaced(RAX, 274623053, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 212, 168, 77, 106, 94, 16], OperandSize::Qword)
}

