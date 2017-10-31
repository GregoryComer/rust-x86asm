use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn fist_1() {
    run_test(&Instruction { mnemonic: Mnemonic::FIST, operand1: Some(IndirectDisplaced(BP, 103, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[219, 86, 103], OperandSize::Word)
}

#[test]
fn fist_2() {
    run_test(&Instruction { mnemonic: Mnemonic::FIST, operand1: Some(Indirect(EDX, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[219, 18], OperandSize::Dword)
}

#[test]
fn fist_3() {
    run_test(&Instruction { mnemonic: Mnemonic::FIST, operand1: Some(IndirectScaledIndexedDisplaced(RAX, RCX, Four, 752937605, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[219, 148, 136, 133, 234, 224, 44], OperandSize::Qword)
}

#[test]
fn fist_4() {
    run_test(&Instruction { mnemonic: Mnemonic::FIST, operand1: Some(IndirectDisplaced(BP, 147, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[223, 150, 147, 0], OperandSize::Word)
}

#[test]
fn fist_5() {
    run_test(&Instruction { mnemonic: Mnemonic::FIST, operand1: Some(IndirectScaledIndexedDisplaced(EDX, EDX, Four, 736566502, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[223, 148, 146, 230, 28, 231, 43], OperandSize::Dword)
}

#[test]
fn fist_6() {
    run_test(&Instruction { mnemonic: Mnemonic::FIST, operand1: Some(Indirect(RDI, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[223, 23], OperandSize::Qword)
}

