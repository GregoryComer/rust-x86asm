use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn xsavec_1() {
    run_test(&Instruction { mnemonic: Mnemonic::XSAVEC, operand1: Some(IndirectDisplaced(BP, 14962, Some(OperandSize::Unsized), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 199, 166, 114, 58], OperandSize::Word)
}

#[test]
fn xsavec_2() {
    run_test(&Instruction { mnemonic: Mnemonic::XSAVEC, operand1: Some(IndirectScaledIndexedDisplaced(ESI, ECX, Two, 778059223, Some(OperandSize::Unsized), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 199, 164, 78, 215, 61, 96, 46], OperandSize::Dword)
}

#[test]
fn xsavec_3() {
    run_test(&Instruction { mnemonic: Mnemonic::XSAVEC, operand1: Some(IndirectDisplaced(RBX, 515056682, Some(OperandSize::Unsized), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 199, 163, 42, 36, 179, 30], OperandSize::Qword)
}

