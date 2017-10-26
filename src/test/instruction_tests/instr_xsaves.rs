use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn xsaves_1() {
    run_test(&Instruction { mnemonic: Mnemonic::XSAVES, operand1: Some(IndirectScaledIndexed(BP, DI, One, Some(OperandSize::Unsized), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 199, 43], OperandSize::Word)
}

#[test]
fn xsaves_2() {
    run_test(&Instruction { mnemonic: Mnemonic::XSAVES, operand1: Some(IndirectDisplaced(EBX, 1877890596, Some(OperandSize::Unsized), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 199, 171, 36, 86, 238, 111], OperandSize::Dword)
}

#[test]
fn xsaves_3() {
    run_test(&Instruction { mnemonic: Mnemonic::XSAVES, operand1: Some(IndirectScaledDisplaced(RAX, Two, 360079518, Some(OperandSize::Unsized), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 199, 44, 69, 158, 96, 118, 21], OperandSize::Qword)
}

