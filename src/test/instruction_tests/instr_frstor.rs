use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn frstor_1() {
    run_test(&Instruction { mnemonic: Mnemonic::FRSTOR, operand1: Some(IndirectScaledIndexedDisplaced(BP, DI, One, 23153, Some(OperandSize::Unsized), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[221, 163, 113, 90], OperandSize::Word)
}

#[test]
fn frstor_2() {
    run_test(&Instruction { mnemonic: Mnemonic::FRSTOR, operand1: Some(IndirectScaledIndexed(ECX, EBX, Four, Some(OperandSize::Unsized), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[221, 36, 153], OperandSize::Dword)
}

#[test]
fn frstor_3() {
    run_test(&Instruction { mnemonic: Mnemonic::FRSTOR, operand1: Some(IndirectScaledIndexedDisplaced(RSI, RDX, Eight, 1410147367, Some(OperandSize::Unsized), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[221, 164, 214, 39, 36, 13, 84], OperandSize::Qword)
}

