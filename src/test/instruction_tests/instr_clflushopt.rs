use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn clflushopt_1() {
    run_test(&Instruction { mnemonic: Mnemonic::CLFLUSHOPT, operand1: Some(Indirect(SI, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 174, 60], OperandSize::Word)
}

#[test]
fn clflushopt_2() {
    run_test(&Instruction { mnemonic: Mnemonic::CLFLUSHOPT, operand1: Some(IndirectScaledDisplaced(EDI, Eight, 1259903661, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 174, 60, 253, 173, 154, 24, 75], OperandSize::Dword)
}

#[test]
fn clflushopt_3() {
    run_test(&Instruction { mnemonic: Mnemonic::CLFLUSHOPT, operand1: Some(IndirectScaledIndexed(RCX, RDX, Two, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 174, 60, 81], OperandSize::Qword)
}

