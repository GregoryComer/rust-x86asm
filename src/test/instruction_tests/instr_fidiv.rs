use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn fidiv_1() {
    run_test(&Instruction { mnemonic: Mnemonic::FIDIV, operand1: Some(IndirectDisplaced(SI, 13717, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[218, 180, 149, 53], OperandSize::Word)
}

#[test]
fn fidiv_2() {
    run_test(&Instruction { mnemonic: Mnemonic::FIDIV, operand1: Some(IndirectScaledIndexed(EDX, ESI, Eight, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[218, 52, 242], OperandSize::Dword)
}

#[test]
fn fidiv_3() {
    run_test(&Instruction { mnemonic: Mnemonic::FIDIV, operand1: Some(IndirectScaledIndexed(RBX, RSI, Two, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[218, 52, 115], OperandSize::Qword)
}

#[test]
fn fidiv_4() {
    run_test(&Instruction { mnemonic: Mnemonic::FIDIV, operand1: Some(IndirectScaledIndexedDisplaced(BP, DI, One, 118, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[222, 115, 118], OperandSize::Word)
}

#[test]
fn fidiv_5() {
    run_test(&Instruction { mnemonic: Mnemonic::FIDIV, operand1: Some(IndirectScaledIndexed(EDI, ESI, Four, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[222, 52, 183], OperandSize::Dword)
}

#[test]
fn fidiv_6() {
    run_test(&Instruction { mnemonic: Mnemonic::FIDIV, operand1: Some(IndirectScaledIndexed(RDI, RBX, Eight, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[222, 52, 223], OperandSize::Qword)
}

