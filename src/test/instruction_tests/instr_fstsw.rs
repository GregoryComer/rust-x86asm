use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn fstsw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::FSTSW, operand1: Some(Indirect(DI, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[155, 221, 61], OperandSize::Word)
}

#[test]
fn fstsw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::FSTSW, operand1: Some(Indirect(EBX, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[155, 221, 59], OperandSize::Dword)
}

#[test]
fn fstsw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::FSTSW, operand1: Some(IndirectScaledIndexedDisplaced(RCX, RBX, Eight, 1323658754, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[155, 221, 188, 217, 2, 110, 229, 78], OperandSize::Qword)
}

