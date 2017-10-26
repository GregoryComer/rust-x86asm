use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn fild_1() {
    run_test(&Instruction { mnemonic: Mnemonic::FILD, operand1: Some(IndirectScaledIndexedDisplaced(BP, SI, One, 28166, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[219, 130, 6, 110], OperandSize::Word)
}

#[test]
fn fild_2() {
    run_test(&Instruction { mnemonic: Mnemonic::FILD, operand1: Some(Indirect(EDI, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[219, 7], OperandSize::Dword)
}

#[test]
fn fild_3() {
    run_test(&Instruction { mnemonic: Mnemonic::FILD, operand1: Some(IndirectScaledIndexedDisplaced(RAX, RSI, Eight, 462880065, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[219, 132, 240, 65, 253, 150, 27], OperandSize::Qword)
}

#[test]
fn fild_4() {
    run_test(&Instruction { mnemonic: Mnemonic::FILD, operand1: Some(IndirectDisplaced(BX, 23422, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[223, 135, 126, 91], OperandSize::Word)
}

#[test]
fn fild_5() {
    run_test(&Instruction { mnemonic: Mnemonic::FILD, operand1: Some(IndirectDisplaced(ECX, 2145898431, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[223, 129, 191, 207, 231, 127], OperandSize::Dword)
}

#[test]
fn fild_6() {
    run_test(&Instruction { mnemonic: Mnemonic::FILD, operand1: Some(IndirectScaledIndexedDisplaced(RDX, RSI, Two, 583519992, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[223, 132, 114, 248, 206, 199, 34], OperandSize::Qword)
}

#[test]
fn fild_7() {
    run_test(&Instruction { mnemonic: Mnemonic::FILD, operand1: Some(IndirectDisplaced(BX, 0, Some(OperandSize::Qword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[223, 47], OperandSize::Word)
}

#[test]
fn fild_8() {
    run_test(&Instruction { mnemonic: Mnemonic::FILD, operand1: Some(IndirectScaledIndexedDisplaced(EDX, EAX, Two, 1980144383, Some(OperandSize::Qword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[223, 172, 66, 255, 154, 6, 118], OperandSize::Dword)
}

#[test]
fn fild_9() {
    run_test(&Instruction { mnemonic: Mnemonic::FILD, operand1: Some(Indirect(RDI, Some(OperandSize::Qword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[223, 47], OperandSize::Qword)
}

