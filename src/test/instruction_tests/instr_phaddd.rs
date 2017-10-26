use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn phaddd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PHADDD, operand1: Some(Direct(MM0)), operand2: Some(Direct(MM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 2, 193], OperandSize::Dword)
}

#[test]
fn phaddd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PHADDD, operand1: Some(Direct(MM2)), operand2: Some(IndirectScaledIndexed(ECX, ECX, Four, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 2, 20, 137], OperandSize::Dword)
}

#[test]
fn phaddd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PHADDD, operand1: Some(Direct(MM1)), operand2: Some(Direct(MM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 2, 205], OperandSize::Qword)
}

#[test]
fn phaddd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PHADDD, operand1: Some(Direct(MM7)), operand2: Some(IndirectScaledIndexedDisplaced(RAX, RSI, Two, 1206482477, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 2, 188, 112, 45, 118, 233, 71], OperandSize::Qword)
}

#[test]
fn phaddd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PHADDD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 2, 205], OperandSize::Dword)
}

#[test]
fn phaddd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PHADDD, operand1: Some(Direct(XMM4)), operand2: Some(Indirect(ECX, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 2, 33], OperandSize::Dword)
}

#[test]
fn phaddd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PHADDD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 2, 225], OperandSize::Qword)
}

#[test]
fn phaddd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PHADDD, operand1: Some(Direct(XMM2)), operand2: Some(Indirect(RAX, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 2, 16], OperandSize::Qword)
}

