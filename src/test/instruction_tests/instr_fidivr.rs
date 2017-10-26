use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn fidivr_1() {
    run_test(&Instruction { mnemonic: Mnemonic::FIDIVR, operand1: Some(IndirectDisplaced(DI, 27343, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[218, 189, 207, 106], OperandSize::Word)
}

#[test]
fn fidivr_2() {
    run_test(&Instruction { mnemonic: Mnemonic::FIDIVR, operand1: Some(IndirectScaledDisplaced(ECX, Two, 816483781, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[218, 60, 77, 197, 141, 170, 48], OperandSize::Dword)
}

#[test]
fn fidivr_3() {
    run_test(&Instruction { mnemonic: Mnemonic::FIDIVR, operand1: Some(IndirectDisplaced(RAX, 498529301, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[218, 184, 21, 244, 182, 29], OperandSize::Qword)
}

#[test]
fn fidivr_4() {
    run_test(&Instruction { mnemonic: Mnemonic::FIDIVR, operand1: Some(IndirectDisplaced(SI, 27840, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[222, 188, 192, 108], OperandSize::Word)
}

#[test]
fn fidivr_5() {
    run_test(&Instruction { mnemonic: Mnemonic::FIDIVR, operand1: Some(IndirectScaledIndexed(EDI, EDI, Eight, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[222, 60, 255], OperandSize::Dword)
}

#[test]
fn fidivr_6() {
    run_test(&Instruction { mnemonic: Mnemonic::FIDIVR, operand1: Some(IndirectDisplaced(RAX, 130868118, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[222, 184, 150, 227, 204, 7], OperandSize::Qword)
}

