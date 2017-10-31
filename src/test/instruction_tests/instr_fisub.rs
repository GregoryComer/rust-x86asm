use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn fisub_1() {
    run_test(&Instruction { mnemonic: Mnemonic::FISUB, operand1: Some(IndirectScaledIndexedDisplaced(BP, SI, One, 31627, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[218, 162, 139, 123], OperandSize::Word)
}

#[test]
fn fisub_2() {
    run_test(&Instruction { mnemonic: Mnemonic::FISUB, operand1: Some(IndirectScaledDisplaced(ESI, Eight, 1399542760, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[218, 36, 245, 232, 83, 107, 83], OperandSize::Dword)
}

#[test]
fn fisub_3() {
    run_test(&Instruction { mnemonic: Mnemonic::FISUB, operand1: Some(IndirectDisplaced(RSI, 234463911, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[218, 166, 167, 162, 249, 13], OperandSize::Qword)
}

#[test]
fn fisub_4() {
    run_test(&Instruction { mnemonic: Mnemonic::FISUB, operand1: Some(IndirectScaledIndexed(BX, SI, One, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[222, 32], OperandSize::Word)
}

#[test]
fn fisub_5() {
    run_test(&Instruction { mnemonic: Mnemonic::FISUB, operand1: Some(IndirectDisplaced(EBX, 968160959, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[222, 163, 191, 246, 180, 57], OperandSize::Dword)
}

#[test]
fn fisub_6() {
    run_test(&Instruction { mnemonic: Mnemonic::FISUB, operand1: Some(IndirectScaledDisplaced(RAX, Eight, 986821570, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[222, 36, 197, 194, 179, 209, 58], OperandSize::Qword)
}

