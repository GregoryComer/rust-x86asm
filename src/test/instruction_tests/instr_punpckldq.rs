use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn punpckldq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PUNPCKLDQ, operand1: Some(Direct(MM4)), operand2: Some(Direct(MM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 98, 224], OperandSize::Dword)
}

#[test]
fn punpckldq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PUNPCKLDQ, operand1: Some(Direct(MM4)), operand2: Some(IndirectDisplaced(EDX, 171617407, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 98, 162, 127, 172, 58, 10], OperandSize::Dword)
}

#[test]
fn punpckldq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PUNPCKLDQ, operand1: Some(Direct(MM6)), operand2: Some(Direct(MM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 98, 245], OperandSize::Qword)
}

#[test]
fn punpckldq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PUNPCKLDQ, operand1: Some(Direct(MM1)), operand2: Some(IndirectScaledIndexedDisplaced(RDX, RCX, Four, 359279211, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 98, 140, 138, 107, 42, 106, 21], OperandSize::Qword)
}

#[test]
fn punpckldq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PUNPCKLDQ, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 98, 252], OperandSize::Dword)
}

#[test]
fn punpckldq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PUNPCKLDQ, operand1: Some(Direct(XMM4)), operand2: Some(IndirectScaledDisplaced(ECX, Two, 1213039952, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 98, 36, 77, 80, 133, 77, 72], OperandSize::Dword)
}

#[test]
fn punpckldq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PUNPCKLDQ, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 98, 241], OperandSize::Qword)
}

#[test]
fn punpckldq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PUNPCKLDQ, operand1: Some(Direct(XMM2)), operand2: Some(IndirectScaledIndexed(RAX, RDX, Four, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 98, 20, 144], OperandSize::Qword)
}

