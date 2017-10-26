use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn xrstor_1() {
    run_test(&Instruction { mnemonic: Mnemonic::XRSTOR, operand1: Some(IndirectScaledIndexed(BX, DI, One, Some(OperandSize::Unsized), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 174, 41], OperandSize::Word)
}

#[test]
fn xrstor_2() {
    run_test(&Instruction { mnemonic: Mnemonic::XRSTOR, operand1: Some(IndirectScaledDisplaced(EDX, Two, 1466632938, Some(OperandSize::Unsized), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 174, 44, 85, 234, 10, 107, 87], OperandSize::Dword)
}

#[test]
fn xrstor_3() {
    run_test(&Instruction { mnemonic: Mnemonic::XRSTOR, operand1: Some(Indirect(RDI, Some(OperandSize::Unsized), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 174, 47], OperandSize::Qword)
}

