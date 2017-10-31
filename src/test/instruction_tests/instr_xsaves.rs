use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn xsaves_1() {
    run_test(&Instruction { mnemonic: Mnemonic::XSAVES, operand1: Some(IndirectScaledIndexed(BP, SI, One, Some(OperandSize::Unsized), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 199, 42], OperandSize::Word)
}

#[test]
fn xsaves_2() {
    run_test(&Instruction { mnemonic: Mnemonic::XSAVES, operand1: Some(IndirectScaledDisplaced(EDX, Four, 1795408089, Some(OperandSize::Unsized), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 199, 44, 149, 217, 192, 3, 107], OperandSize::Dword)
}

#[test]
fn xsaves_3() {
    run_test(&Instruction { mnemonic: Mnemonic::XSAVES, operand1: Some(Indirect(RCX, Some(OperandSize::Unsized), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 199, 41], OperandSize::Qword)
}

