use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn fisubr_1() {
    run_test(&Instruction { mnemonic: Mnemonic::FISUBR, operand1: Some(IndirectDisplaced(SI, 216, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[218, 172, 216, 0], OperandSize::Word)
}

#[test]
fn fisubr_2() {
    run_test(&Instruction { mnemonic: Mnemonic::FISUBR, operand1: Some(IndirectDisplaced(EDI, 2102396329, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[218, 175, 169, 5, 80, 125], OperandSize::Dword)
}

#[test]
fn fisubr_3() {
    run_test(&Instruction { mnemonic: Mnemonic::FISUBR, operand1: Some(IndirectScaledIndexedDisplaced(RDX, RAX, Eight, 1978807657, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[218, 172, 194, 105, 53, 242, 117], OperandSize::Qword)
}

#[test]
fn fisubr_4() {
    run_test(&Instruction { mnemonic: Mnemonic::FISUBR, operand1: Some(IndirectDisplaced(SI, 25676, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[222, 172, 76, 100], OperandSize::Word)
}

#[test]
fn fisubr_5() {
    run_test(&Instruction { mnemonic: Mnemonic::FISUBR, operand1: Some(IndirectDisplaced(ECX, 805682048, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[222, 169, 128, 187, 5, 48], OperandSize::Dword)
}

#[test]
fn fisubr_6() {
    run_test(&Instruction { mnemonic: Mnemonic::FISUBR, operand1: Some(IndirectScaledIndexed(RSI, RDI, Four, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[222, 44, 190], OperandSize::Qword)
}

