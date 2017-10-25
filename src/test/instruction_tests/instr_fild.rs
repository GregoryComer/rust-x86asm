use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn fild_1() {
    run_test(&Instruction { mnemonic: Mnemonic::FILD, operand1: Some(IndirectScaledIndexedDisplaced(BP, DI, One, 2336, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[219, 131, 32, 9], OperandSize::Word)
}

#[test]
fn fild_2() {
    run_test(&Instruction { mnemonic: Mnemonic::FILD, operand1: Some(IndirectScaledIndexedDisplaced(ECX, ECX, Eight, 1153213638, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[219, 132, 201, 198, 164, 188, 68], OperandSize::Dword)
}

#[test]
fn fild_3() {
    run_test(&Instruction { mnemonic: Mnemonic::FILD, operand1: Some(IndirectScaledIndexed(RDX, RDX, Two, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[219, 4, 82], OperandSize::Qword)
}

#[test]
fn fild_4() {
    run_test(&Instruction { mnemonic: Mnemonic::FILD, operand1: Some(IndirectScaledIndexedDisplaced(BX, DI, One, 40, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[223, 65, 40], OperandSize::Word)
}

#[test]
fn fild_5() {
    run_test(&Instruction { mnemonic: Mnemonic::FILD, operand1: Some(IndirectScaledIndexedDisplaced(EDI, ECX, Four, 1903142962, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[223, 132, 143, 50, 168, 111, 113], OperandSize::Dword)
}

#[test]
fn fild_6() {
    run_test(&Instruction { mnemonic: Mnemonic::FILD, operand1: Some(IndirectDisplaced(RCX, 316597235, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[223, 129, 243, 227, 222, 18], OperandSize::Qword)
}

#[test]
fn fild_7() {
    run_test(&Instruction { mnemonic: Mnemonic::FILD, operand1: Some(IndirectScaledIndexedDisplaced(BX, SI, One, 218, Some(OperandSize::Qword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[223, 168, 218, 0], OperandSize::Word)
}

#[test]
fn fild_8() {
    run_test(&Instruction { mnemonic: Mnemonic::FILD, operand1: Some(Indirect(EDI, Some(OperandSize::Qword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[223, 47], OperandSize::Dword)
}

#[test]
fn fild_9() {
    run_test(&Instruction { mnemonic: Mnemonic::FILD, operand1: Some(IndirectDisplaced(RBX, 2048698734, Some(OperandSize::Qword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[223, 171, 110, 169, 28, 122], OperandSize::Qword)
}

