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
    run_test(&Instruction { mnemonic: Mnemonic::FIADD, operand1: Some(IndirectDisplaced(EBX, 1684295499, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[218, 131, 75, 79, 100, 100], OperandSize::Dword)
}

#[test]
fn fiadd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::FIADD, operand1: Some(IndirectScaledIndexedDisplaced(RCX, RDI, Four, 409052741, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[218, 132, 185, 69, 166, 97, 24], OperandSize::Qword)
}

#[test]
fn fiadd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::FIADD, operand1: Some(Indirect(DI, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[222, 5], OperandSize::Word)
}

#[test]
fn fiadd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::FIADD, operand1: Some(IndirectScaledIndexedDisplaced(EDI, ESI, Four, 1260485731, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[222, 132, 183, 99, 124, 33, 75], OperandSize::Dword)
}

#[test]
fn fiadd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::FIADD, operand1: Some(Indirect(RSI, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[222, 6], OperandSize::Qword)
}

