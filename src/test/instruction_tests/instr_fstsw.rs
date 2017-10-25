use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn fstsw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::FSTSW, operand1: Some(IndirectScaledIndexedDisplaced(BP, SI, One, 20419, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[155, 221, 186, 195, 79], OperandSize::Word)
}

#[test]
fn fstsw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::FSTSW, operand1: Some(IndirectScaledIndexedDisplaced(EDX, EAX, Two, 814601851, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[155, 221, 188, 66, 123, 214, 141, 48], OperandSize::Dword)
}

#[test]
fn fstsw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::FSTSW, operand1: Some(Indirect(RAX, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[155, 221, 56], OperandSize::Qword)
}

