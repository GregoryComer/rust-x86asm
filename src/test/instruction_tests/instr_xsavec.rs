use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn xsavec_1() {
    run_test(&Instruction { mnemonic: Mnemonic::XSAVEC, operand1: Some(IndirectDisplaced(SI, 25265, Some(OperandSize::Unsized), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 199, 164, 177, 98], OperandSize::Word)
}

#[test]
fn xsavec_2() {
    run_test(&Instruction { mnemonic: Mnemonic::XSAVEC, operand1: Some(IndirectScaledIndexed(EDX, EBX, Four, Some(OperandSize::Unsized), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 199, 36, 154], OperandSize::Dword)
}

#[test]
fn xsavec_3() {
    run_test(&Instruction { mnemonic: Mnemonic::XSAVEC, operand1: Some(Indirect(RDI, Some(OperandSize::Unsized), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 199, 39], OperandSize::Qword)
}

