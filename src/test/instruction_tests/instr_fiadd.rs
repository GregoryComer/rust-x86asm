use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn fiadd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::FIADD, operand1: Some(IndirectScaledIndexed(BP, SI, One, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[218, 2], OperandSize::Word)
}

#[test]
fn fiadd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::FIADD, operand1: Some(IndirectScaledDisplaced(EDI, Two, 852787895, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[218, 4, 125, 183, 130, 212, 50], OperandSize::Dword)
}

#[test]
fn fiadd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::FIADD, operand1: Some(IndirectScaledIndexedDisplaced(RSI, RCX, Four, 1696987690, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[218, 132, 142, 42, 250, 37, 101], OperandSize::Qword)
}

#[test]
fn fiadd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::FIADD, operand1: Some(IndirectScaledIndexed(BP, DI, One, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[222, 3], OperandSize::Word)
}

#[test]
fn fiadd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::FIADD, operand1: Some(IndirectDisplaced(EAX, 29195080, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[222, 128, 72, 123, 189, 1], OperandSize::Dword)
}

#[test]
fn fiadd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::FIADD, operand1: Some(IndirectScaledIndexedDisplaced(RDX, RBX, Eight, 151183186, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[222, 132, 218, 82, 223, 2, 9], OperandSize::Qword)
}

