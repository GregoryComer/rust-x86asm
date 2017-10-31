use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn fisub_1() {
    run_test(&Instruction { mnemonic: Mnemonic::FISUB, operand1: Some(IndirectScaledIndexedDisplaced(BP, SI, One, 240, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[218, 162, 240, 0], OperandSize::Word)
}

#[test]
fn fisub_2() {
    run_test(&Instruction { mnemonic: Mnemonic::FISUB, operand1: Some(IndirectScaledIndexed(EDX, ESI, Four, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[218, 36, 178], OperandSize::Dword)
}

#[test]
fn fisub_3() {
    run_test(&Instruction { mnemonic: Mnemonic::FISUB, operand1: Some(Indirect(RCX, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[218, 33], OperandSize::Qword)
}

#[test]
fn fisub_4() {
    run_test(&Instruction { mnemonic: Mnemonic::FISUB, operand1: Some(IndirectScaledIndexedDisplaced(BP, SI, One, 162, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[222, 162, 162, 0], OperandSize::Word)
}

#[test]
fn fisub_5() {
    run_test(&Instruction { mnemonic: Mnemonic::FISUB, operand1: Some(IndirectScaledIndexed(ESI, EDX, Eight, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[222, 36, 214], OperandSize::Dword)
}

#[test]
fn fisub_6() {
    run_test(&Instruction { mnemonic: Mnemonic::FISUB, operand1: Some(IndirectScaledIndexed(RDX, RAX, Four, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[222, 36, 130], OperandSize::Qword)
}

