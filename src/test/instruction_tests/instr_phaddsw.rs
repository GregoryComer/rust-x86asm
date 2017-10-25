use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn phaddsw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PHADDSW, operand1: Some(Direct(MM6)), operand2: Some(Direct(MM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 3, 245], OperandSize::Dword)
}

#[test]
fn phaddsw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PHADDSW, operand1: Some(Direct(MM2)), operand2: Some(IndirectScaledIndexedDisplaced(EAX, ECX, Two, 437947688, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 3, 148, 72, 40, 141, 26, 26], OperandSize::Dword)
}

#[test]
fn phaddsw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PHADDSW, operand1: Some(Direct(MM1)), operand2: Some(Direct(MM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 3, 202], OperandSize::Qword)
}

#[test]
fn phaddsw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PHADDSW, operand1: Some(Direct(MM0)), operand2: Some(IndirectScaledIndexed(RCX, RSI, Eight, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 3, 4, 241], OperandSize::Qword)
}

#[test]
fn phaddsw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PHADDSW, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 3, 214], OperandSize::Dword)
}

#[test]
fn phaddsw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PHADDSW, operand1: Some(Direct(XMM6)), operand2: Some(Indirect(EDX, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 3, 50], OperandSize::Dword)
}

#[test]
fn phaddsw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PHADDSW, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 3, 234], OperandSize::Qword)
}

#[test]
fn phaddsw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PHADDSW, operand1: Some(Direct(XMM0)), operand2: Some(IndirectDisplaced(RAX, 808306296, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 3, 128, 120, 198, 45, 48], OperandSize::Qword)
}

