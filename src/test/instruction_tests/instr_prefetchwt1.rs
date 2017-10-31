use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn prefetchwt1_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PREFETCHWT1, operand1: Some(IndirectScaledIndexedDisplaced(ECX, EDX, Eight, 783772546, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 13, 148, 209, 130, 107, 183, 46], OperandSize::Dword)
}

#[test]
fn prefetchwt1_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PREFETCHWT1, operand1: Some(Indirect(RBX, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 13, 19], OperandSize::Qword)
}

