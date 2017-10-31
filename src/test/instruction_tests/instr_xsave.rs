use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn xsave_1() {
    run_test(&Instruction { mnemonic: Mnemonic::XSAVE, operand1: Some(Indirect(BX, Some(OperandSize::Unsized), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 174, 39], OperandSize::Word)
}

#[test]
fn xsave_2() {
    run_test(&Instruction { mnemonic: Mnemonic::XSAVE, operand1: Some(IndirectScaledIndexed(EDI, EBX, Eight, Some(OperandSize::Unsized), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 174, 36, 223], OperandSize::Dword)
}

#[test]
fn xsave_3() {
    run_test(&Instruction { mnemonic: Mnemonic::XSAVE, operand1: Some(Indirect(RDI, Some(OperandSize::Unsized), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 174, 39], OperandSize::Qword)
}

