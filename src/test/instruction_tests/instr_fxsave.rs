use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn fxsave_1() {
    run_test(&Instruction { mnemonic: Mnemonic::FXSAVE, operand1: Some(IndirectScaledIndexed(BX, DI, One, Some(OperandSize::Unsized), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 174, 1], OperandSize::Word)
}

#[test]
fn fxsave_2() {
    run_test(&Instruction { mnemonic: Mnemonic::FXSAVE, operand1: Some(IndirectDisplaced(EDX, 391598113, Some(OperandSize::Unsized), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 174, 130, 33, 80, 87, 23], OperandSize::Dword)
}

#[test]
fn fxsave_3() {
    run_test(&Instruction { mnemonic: Mnemonic::FXSAVE, operand1: Some(IndirectScaledIndexedDisplaced(RAX, RDI, Four, 1101177861, Some(OperandSize::Unsized), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 174, 132, 184, 5, 164, 162, 65], OperandSize::Qword)
}

