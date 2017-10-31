use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn phaddd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PHADDD, operand1: Some(Direct(MM7)), operand2: Some(Direct(MM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 2, 248], OperandSize::Dword)
}

#[test]
fn phaddd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PHADDD, operand1: Some(Direct(MM5)), operand2: Some(IndirectScaledDisplaced(EDX, Eight, 1162440715, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 2, 44, 213, 11, 112, 73, 69], OperandSize::Dword)
}

#[test]
fn phaddd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PHADDD, operand1: Some(Direct(MM5)), operand2: Some(Direct(MM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 2, 233], OperandSize::Qword)
}

#[test]
fn phaddd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PHADDD, operand1: Some(Direct(MM3)), operand2: Some(IndirectScaledIndexed(RSI, RSI, Eight, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 2, 28, 246], OperandSize::Qword)
}

#[test]
fn phaddd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PHADDD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 2, 239], OperandSize::Dword)
}

#[test]
fn phaddd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PHADDD, operand1: Some(Direct(XMM4)), operand2: Some(Indirect(EDI, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 2, 39], OperandSize::Dword)
}

#[test]
fn phaddd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PHADDD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 2, 255], OperandSize::Qword)
}

#[test]
fn phaddd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PHADDD, operand1: Some(Direct(XMM5)), operand2: Some(Indirect(RDX, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 2, 42], OperandSize::Qword)
}

