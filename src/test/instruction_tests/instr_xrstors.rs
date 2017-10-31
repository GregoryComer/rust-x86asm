use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn xrstors_1() {
    run_test(&Instruction { mnemonic: Mnemonic::XRSTORS, operand1: Some(IndirectScaledIndexedDisplaced(BX, DI, One, 9, Some(OperandSize::Unsized), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 199, 89, 9], OperandSize::Word)
}

#[test]
fn xrstors_2() {
    run_test(&Instruction { mnemonic: Mnemonic::XRSTORS, operand1: Some(IndirectDisplaced(EDX, 991574451, Some(OperandSize::Unsized), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 199, 154, 179, 57, 26, 59], OperandSize::Dword)
}

#[test]
fn xrstors_3() {
    run_test(&Instruction { mnemonic: Mnemonic::XRSTORS, operand1: Some(IndirectDisplaced(RDX, 1368307711, Some(OperandSize::Unsized), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 199, 154, 255, 183, 142, 81], OperandSize::Qword)
}

