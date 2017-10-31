use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn bndmk_1() {
    run_test(&Instruction { mnemonic: Mnemonic::BNDMK, operand1: Some(Direct(BND3)), operand2: Some(IndirectDisplaced(EAX, 1177366412, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 27, 152, 140, 47, 45, 70], OperandSize::Dword)
}

#[test]
fn bndmk_2() {
    run_test(&Instruction { mnemonic: Mnemonic::BNDMK, operand1: Some(Direct(BND1)), operand2: Some(IndirectScaledDisplaced(RDI, Two, 1544037568, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 27, 12, 125, 192, 36, 8, 92], OperandSize::Qword)
}

