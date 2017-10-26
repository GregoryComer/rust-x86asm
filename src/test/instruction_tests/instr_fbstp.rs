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
    run_test(&Instruction { mnemonic: Mnemonic::FBSTP, operand1: Some(Indirect(ECX, Some(OperandSize::Tbyte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[223, 49], OperandSize::Dword)
}

#[test]
fn fbstp_3() {
    run_test(&Instruction { mnemonic: Mnemonic::FBSTP, operand1: Some(IndirectScaledIndexed(RCX, RBX, Eight, Some(OperandSize::Tbyte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[223, 52, 217], OperandSize::Qword)
}

