use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn fxrstor_1() {
    run_test(&Instruction { mnemonic: Mnemonic::FXRSTOR, operand1: Some(IndirectDisplaced(BP, 9823, Some(OperandSize::Unsized), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 174, 142, 95, 38], OperandSize::Word)
}

#[test]
fn fxrstor_2() {
    run_test(&Instruction { mnemonic: Mnemonic::FXRSTOR, operand1: Some(IndirectScaledIndexedDisplaced(ECX, ESI, Four, 1784727459, Some(OperandSize::Unsized), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 174, 140, 177, 163, 199, 96, 106], OperandSize::Dword)
}

#[test]
fn fxrstor_3() {
    run_test(&Instruction { mnemonic: Mnemonic::FXRSTOR, operand1: Some(IndirectScaledIndexedDisplaced(RSI, RSI, Two, 466817617, Some(OperandSize::Unsized), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 174, 140, 118, 81, 18, 211, 27], OperandSize::Qword)
}

