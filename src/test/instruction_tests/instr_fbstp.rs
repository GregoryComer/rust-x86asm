use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn fbstp_1() {
    run_test(&Instruction { mnemonic: Mnemonic::FBSTP, operand1: Some(IndirectScaledIndexed(BX, SI, One, Some(OperandSize::Tbyte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[223, 48], OperandSize::Word)
}

#[test]
fn fbstp_2() {
    run_test(&Instruction { mnemonic: Mnemonic::FBSTP, operand1: Some(IndirectDisplaced(EDI, 2091886860, Some(OperandSize::Tbyte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[223, 183, 12, 169, 175, 124], OperandSize::Dword)
}

#[test]
fn fbstp_3() {
    run_test(&Instruction { mnemonic: Mnemonic::FBSTP, operand1: Some(IndirectScaledIndexedDisplaced(RSI, RDI, Two, 1184633389, Some(OperandSize::Tbyte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[223, 180, 126, 45, 18, 156, 70], OperandSize::Qword)
}

