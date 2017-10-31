use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn xlat_1() {
    run_test(&Instruction { mnemonic: Mnemonic::XLAT, operand1: Some(IndirectScaledIndexed(BP, SI, One, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[215], OperandSize::Word)
}

#[test]
fn xlat_2() {
    run_test(&Instruction { mnemonic: Mnemonic::XLAT, operand1: Some(IndirectDisplaced(EAX, 1382872497, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[215], OperandSize::Dword)
}

#[test]
fn xlat_3() {
    run_test(&Instruction { mnemonic: Mnemonic::XLAT, operand1: Some(IndirectDisplaced(RAX, 512755582, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[215], OperandSize::Qword)
}

