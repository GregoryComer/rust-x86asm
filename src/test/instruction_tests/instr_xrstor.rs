use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn xrstor_1() {
    run_test(&Instruction { mnemonic: Mnemonic::XRSTOR, operand1: Some(Indirect(DI, Some(OperandSize::Unsized), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 174, 45], OperandSize::Word)
}

#[test]
fn xrstor_2() {
    run_test(&Instruction { mnemonic: Mnemonic::XRSTOR, operand1: Some(IndirectScaledIndexedDisplaced(EAX, EAX, Two, 1278228434, Some(OperandSize::Unsized), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 174, 172, 64, 210, 55, 48, 76], OperandSize::Dword)
}

#[test]
fn xrstor_3() {
    run_test(&Instruction { mnemonic: Mnemonic::XRSTOR, operand1: Some(IndirectScaledIndexed(RDX, RAX, Two, Some(OperandSize::Unsized), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 174, 44, 66], OperandSize::Qword)
}

