use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn xsaveopt_1() {
    run_test(&Instruction { mnemonic: Mnemonic::XSAVEOPT, operand1: Some(IndirectScaledIndexed(EDX, ESI, Eight, Some(OperandSize::Unsized), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 174, 52, 242], OperandSize::Dword)
}

#[test]
fn xsaveopt_2() {
    run_test(&Instruction { mnemonic: Mnemonic::XSAVEOPT, operand1: Some(IndirectScaledIndexedDisplaced(RAX, RDX, Eight, 2016049987, Some(OperandSize::Unsized), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 174, 180, 208, 67, 123, 42, 120], OperandSize::Qword)
}

