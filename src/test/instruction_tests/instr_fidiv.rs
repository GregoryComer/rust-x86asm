use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn fidiv_1() {
    run_test(&Instruction { mnemonic: Mnemonic::FIDIV, operand1: Some(IndirectScaledIndexedDisplaced(BP, DI, One, 8, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[218, 115, 8], OperandSize::Word)
}

#[test]
fn fidiv_2() {
    run_test(&Instruction { mnemonic: Mnemonic::FIDIV, operand1: Some(Indirect(EDI, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[218, 55], OperandSize::Dword)
}

#[test]
fn fidiv_3() {
    run_test(&Instruction { mnemonic: Mnemonic::FIDIV, operand1: Some(IndirectScaledIndexedDisplaced(RBX, RSI, Four, 1412949745, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[218, 180, 179, 241, 230, 55, 84], OperandSize::Qword)
}

#[test]
fn fidiv_4() {
    run_test(&Instruction { mnemonic: Mnemonic::FIDIV, operand1: Some(IndirectScaledIndexedDisplaced(BP, SI, One, 235, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[222, 178, 235, 0], OperandSize::Word)
}

#[test]
fn fidiv_5() {
    run_test(&Instruction { mnemonic: Mnemonic::FIDIV, operand1: Some(IndirectScaledIndexed(EDI, ESI, Two, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[222, 52, 119], OperandSize::Dword)
}

#[test]
fn fidiv_6() {
    run_test(&Instruction { mnemonic: Mnemonic::FIDIV, operand1: Some(IndirectScaledDisplaced(RDI, Two, 134727651, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[222, 52, 125, 227, 199, 7, 8], OperandSize::Qword)
}

