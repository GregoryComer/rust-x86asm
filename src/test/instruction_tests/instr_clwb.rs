use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn clwb_1() {
    run_test(&Instruction { mnemonic: Mnemonic::CLWB, operand1: Some(IndirectScaledIndexedDisplaced(EDI, EDX, Two, 555629817, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 174, 180, 87, 249, 60, 30, 33], OperandSize::Dword)
}

#[test]
fn clwb_2() {
    run_test(&Instruction { mnemonic: Mnemonic::CLWB, operand1: Some(Indirect(RDX, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 174, 50], OperandSize::Qword)
}

