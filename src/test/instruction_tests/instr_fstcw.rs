use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn fstcw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::FSTCW, operand1: Some(Indirect(DI, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[155, 217, 61], OperandSize::Word)
}

#[test]
fn fstcw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::FSTCW, operand1: Some(Indirect(ESI, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[155, 217, 62], OperandSize::Dword)
}

#[test]
fn fstcw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::FSTCW, operand1: Some(IndirectScaledIndexed(RDX, RDX, Two, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[155, 217, 60, 82], OperandSize::Qword)
}

