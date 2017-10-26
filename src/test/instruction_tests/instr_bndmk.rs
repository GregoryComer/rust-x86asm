use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn bndmk_1() {
    run_test(&Instruction { mnemonic: Mnemonic::BNDMK, operand1: Some(Direct(BND2)), operand2: Some(Indirect(ECX, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 27, 17], OperandSize::Dword)
}

#[test]
fn bndmk_2() {
    run_test(&Instruction { mnemonic: Mnemonic::BNDMK, operand1: Some(Direct(BND1)), operand2: Some(IndirectDisplaced(RCX, 1057070530, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 27, 137, 194, 157, 1, 63], OperandSize::Qword)
}

