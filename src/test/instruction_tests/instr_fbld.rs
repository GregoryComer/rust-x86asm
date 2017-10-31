use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn fbld_1() {
    run_test(&Instruction { mnemonic: Mnemonic::FBLD, operand1: Some(IndirectScaledIndexed(BP, SI, One, Some(OperandSize::Tbyte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[223, 34], OperandSize::Word)
}

#[test]
fn fbld_2() {
    run_test(&Instruction { mnemonic: Mnemonic::FBLD, operand1: Some(IndirectScaledIndexedDisplaced(EDX, ESI, Four, 778625900, Some(OperandSize::Tbyte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[223, 164, 178, 108, 227, 104, 46], OperandSize::Dword)
}

#[test]
fn fbld_3() {
    run_test(&Instruction { mnemonic: Mnemonic::FBLD, operand1: Some(Indirect(RSI, Some(OperandSize::Tbyte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[223, 38], OperandSize::Qword)
}

