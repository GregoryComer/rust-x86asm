use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn clwb_1() {
    run_test(&Instruction { mnemonic: Mnemonic::CLWB, operand1: Some(IndirectDisplaced(EDI, 846815526, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 174, 183, 38, 97, 121, 50], OperandSize::Dword)
}

#[test]
fn clwb_2() {
    run_test(&Instruction { mnemonic: Mnemonic::CLWB, operand1: Some(IndirectScaledIndexed(RCX, RSI, Four, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 174, 52, 177], OperandSize::Qword)
}

