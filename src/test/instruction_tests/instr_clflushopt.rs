use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn clflushopt_1() {
    run_test(&Instruction { mnemonic: Mnemonic::CLFLUSHOPT, operand1: Some(IndirectScaledIndexedDisplaced(BX, SI, One, 240, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 174, 184, 240, 0], OperandSize::Word)
}

#[test]
fn clflushopt_2() {
    run_test(&Instruction { mnemonic: Mnemonic::CLFLUSHOPT, operand1: Some(IndirectScaledDisplaced(ECX, Four, 937726867, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 174, 60, 141, 147, 147, 228, 55], OperandSize::Dword)
}

#[test]
fn clflushopt_3() {
    run_test(&Instruction { mnemonic: Mnemonic::CLFLUSHOPT, operand1: Some(Indirect(RSI, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 174, 62], OperandSize::Qword)
}

