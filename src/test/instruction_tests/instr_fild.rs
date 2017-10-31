use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn fild_1() {
    run_test(&Instruction { mnemonic: Mnemonic::FILD, operand1: Some(IndirectScaledIndexedDisplaced(BX, DI, One, 204, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[219, 129, 204, 0], OperandSize::Word)
}

#[test]
fn fild_2() {
    run_test(&Instruction { mnemonic: Mnemonic::FILD, operand1: Some(IndirectDisplaced(EAX, 890069507, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[219, 128, 3, 98, 13, 53], OperandSize::Dword)
}

#[test]
fn fild_3() {
    run_test(&Instruction { mnemonic: Mnemonic::FILD, operand1: Some(IndirectScaledDisplaced(RSI, Four, 1472008465, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[219, 4, 181, 17, 17, 189, 87], OperandSize::Qword)
}

#[test]
fn fild_4() {
    run_test(&Instruction { mnemonic: Mnemonic::FILD, operand1: Some(IndirectScaledIndexedDisplaced(BX, DI, One, 24832, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[223, 129, 0, 97], OperandSize::Word)
}

#[test]
fn fild_5() {
    run_test(&Instruction { mnemonic: Mnemonic::FILD, operand1: Some(IndirectScaledIndexed(ECX, EAX, Eight, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[223, 4, 193], OperandSize::Dword)
}

#[test]
fn fild_6() {
    run_test(&Instruction { mnemonic: Mnemonic::FILD, operand1: Some(IndirectScaledIndexedDisplaced(RBX, RDX, Eight, 661725118, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[223, 132, 211, 190, 31, 113, 39], OperandSize::Qword)
}

#[test]
fn fild_7() {
    run_test(&Instruction { mnemonic: Mnemonic::FILD, operand1: Some(IndirectScaledIndexedDisplaced(BP, DI, One, 70, Some(OperandSize::Qword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[223, 107, 70], OperandSize::Word)
}

#[test]
fn fild_8() {
    run_test(&Instruction { mnemonic: Mnemonic::FILD, operand1: Some(IndirectScaledIndexedDisplaced(EBX, EBX, Two, 1574749451, Some(OperandSize::Qword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[223, 172, 91, 11, 197, 220, 93], OperandSize::Dword)
}

#[test]
fn fild_9() {
    run_test(&Instruction { mnemonic: Mnemonic::FILD, operand1: Some(Indirect(RDI, Some(OperandSize::Qword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[223, 47], OperandSize::Qword)
}

