use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn xrstor_1() {
    run_test(&Instruction { mnemonic: Mnemonic::XRSTOR, operand1: Some(IndirectScaledIndexedDisplaced(BP, SI, One, 61, Some(OperandSize::Unsized), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 174, 106, 61], OperandSize::Word)
}

#[test]
fn xrstor_2() {
    run_test(&Instruction { mnemonic: Mnemonic::XRSTOR, operand1: Some(IndirectScaledDisplaced(EBX, Four, 445977385, Some(OperandSize::Unsized), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 174, 44, 157, 41, 19, 149, 26], OperandSize::Dword)
}

#[test]
fn xrstor_3() {
    run_test(&Instruction { mnemonic: Mnemonic::XRSTOR, operand1: Some(IndirectScaledDisplaced(RDI, Four, 948338655, Some(OperandSize::Unsized), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 174, 44, 189, 223, 127, 134, 56], OperandSize::Qword)
}

