use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn fmul_1() {
    run_test(&Instruction { mnemonic: Mnemonic::FMUL, operand1: Some(IndirectDisplaced(DI, 32245, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[216, 141, 245, 125], OperandSize::Word)
}

#[test]
fn fmul_2() {
    run_test(&Instruction { mnemonic: Mnemonic::FMUL, operand1: Some(Indirect(EDI, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[216, 15], OperandSize::Dword)
}

#[test]
fn fmul_3() {
    run_test(&Instruction { mnemonic: Mnemonic::FMUL, operand1: Some(IndirectScaledIndexedDisplaced(RDI, RSI, Four, 1388368305, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[216, 140, 183, 177, 209, 192, 82], OperandSize::Qword)
}

#[test]
fn fmul_4() {
    run_test(&Instruction { mnemonic: Mnemonic::FMUL, operand1: Some(Direct(ST)), operand2: Some(Direct(ST2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[216, 202], OperandSize::Word)
}

#[test]
fn fmul_5() {
    run_test(&Instruction { mnemonic: Mnemonic::FMUL, operand1: Some(Direct(ST)), operand2: Some(Direct(ST4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[216, 204], OperandSize::Dword)
}

#[test]
fn fmul_6() {
    run_test(&Instruction { mnemonic: Mnemonic::FMUL, operand1: Some(Direct(ST)), operand2: Some(Direct(ST5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[216, 205], OperandSize::Qword)
}

#[test]
fn fmul_7() {
    run_test(&Instruction { mnemonic: Mnemonic::FMUL, operand1: Some(IndirectScaledIndexedDisplaced(BX, SI, One, 15507, Some(OperandSize::Qword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[220, 136, 147, 60], OperandSize::Word)
}

#[test]
fn fmul_8() {
    run_test(&Instruction { mnemonic: Mnemonic::FMUL, operand1: Some(IndirectScaledIndexed(EBX, EAX, Four, Some(OperandSize::Qword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[220, 12, 131], OperandSize::Dword)
}

#[test]
fn fmul_9() {
    run_test(&Instruction { mnemonic: Mnemonic::FMUL, operand1: Some(IndirectScaledDisplaced(RSI, Two, 196066436, Some(OperandSize::Qword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[220, 12, 117, 132, 188, 175, 11], OperandSize::Qword)
}

#[test]
fn fmul_10() {
    run_test(&Instruction { mnemonic: Mnemonic::FMUL, operand1: Some(Direct(ST4)), operand2: Some(Direct(ST)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[220, 204], OperandSize::Word)
}

#[test]
fn fmul_11() {
    run_test(&Instruction { mnemonic: Mnemonic::FMUL, operand1: Some(Direct(ST2)), operand2: Some(Direct(ST)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[220, 202], OperandSize::Dword)
}

#[test]
fn fmul_12() {
    run_test(&Instruction { mnemonic: Mnemonic::FMUL, operand1: Some(Direct(ST4)), operand2: Some(Direct(ST)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[220, 204], OperandSize::Qword)
}

