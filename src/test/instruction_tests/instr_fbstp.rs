use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn fbstp_1() {
    run_test(&Instruction { mnemonic: Mnemonic::FBSTP, operand1: Some(IndirectScaledIndexedDisplaced(BX, SI, One, 26115, Some(OperandSize::Tbyte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[223, 176, 3, 102], OperandSize::Word)
}

#[test]
fn fbstp_2() {
    run_test(&Instruction { mnemonic: Mnemonic::FBSTP, operand1: Some(Indirect(EDI, Some(OperandSize::Tbyte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[223, 55], OperandSize::Dword)
}

#[test]
fn fbstp_3() {
    run_test(&Instruction { mnemonic: Mnemonic::FBSTP, operand1: Some(Indirect(RCX, Some(OperandSize::Tbyte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[223, 49], OperandSize::Qword)
}

