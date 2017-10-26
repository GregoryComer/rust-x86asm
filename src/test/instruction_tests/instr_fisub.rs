use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn fisub_1() {
    run_test(&Instruction { mnemonic: Mnemonic::FISUB, operand1: Some(IndirectScaledIndexed(BP, SI, One, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[218, 34], OperandSize::Word)
}

#[test]
fn fisub_2() {
    run_test(&Instruction { mnemonic: Mnemonic::FISUB, operand1: Some(IndirectScaledDisplaced(EDI, Eight, 148716385, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[218, 36, 253, 97, 59, 221, 8], OperandSize::Dword)
}

#[test]
fn fisub_3() {
    run_test(&Instruction { mnemonic: Mnemonic::FISUB, operand1: Some(IndirectDisplaced(RDI, 1506156539, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[218, 167, 251, 31, 198, 89], OperandSize::Qword)
}

#[test]
fn fisub_4() {
    run_test(&Instruction { mnemonic: Mnemonic::FISUB, operand1: Some(IndirectDisplaced(BP, 30057, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[222, 166, 105, 117], OperandSize::Word)
}

#[test]
fn fisub_5() {
    run_test(&Instruction { mnemonic: Mnemonic::FISUB, operand1: Some(IndirectScaledDisplaced(EBX, Four, 282775065, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[222, 36, 157, 25, 206, 218, 16], OperandSize::Dword)
}

#[test]
fn fisub_6() {
    run_test(&Instruction { mnemonic: Mnemonic::FISUB, operand1: Some(IndirectScaledDisplaced(RDX, Two, 1743357961, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[222, 36, 85, 9, 136, 233, 103], OperandSize::Qword)
}

