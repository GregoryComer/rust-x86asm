use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn xsave_1() {
    run_test(&Instruction { mnemonic: Mnemonic::XSAVE, operand1: Some(IndirectDisplaced(BP, 70, Some(OperandSize::Unsized), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 174, 102, 70], OperandSize::Word)
}

#[test]
fn xsave_2() {
    run_test(&Instruction { mnemonic: Mnemonic::XSAVE, operand1: Some(IndirectScaledIndexedDisplaced(ESI, EAX, Four, 1462695868, Some(OperandSize::Unsized), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 174, 164, 134, 188, 247, 46, 87], OperandSize::Dword)
}

#[test]
fn xsave_3() {
    run_test(&Instruction { mnemonic: Mnemonic::XSAVE, operand1: Some(IndirectScaledIndexed(RAX, RDI, Four, Some(OperandSize::Unsized), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 174, 36, 184], OperandSize::Qword)
}

