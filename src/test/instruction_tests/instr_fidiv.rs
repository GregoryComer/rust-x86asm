use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn fidiv_1() {
    run_test(&Instruction { mnemonic: Mnemonic::FIDIV, operand1: Some(Indirect(DI, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[218, 53], OperandSize::Word)
}

#[test]
fn fidiv_2() {
    run_test(&Instruction { mnemonic: Mnemonic::FIDIV, operand1: Some(IndirectScaledIndexedDisplaced(EBX, ESI, Four, 1172625045, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[218, 180, 179, 149, 214, 228, 69], OperandSize::Dword)
}

#[test]
fn fidiv_3() {
    run_test(&Instruction { mnemonic: Mnemonic::FIDIV, operand1: Some(IndirectDisplaced(RCX, 875419400, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[218, 177, 8, 215, 45, 52], OperandSize::Qword)
}

#[test]
fn fidiv_4() {
    run_test(&Instruction { mnemonic: Mnemonic::FIDIV, operand1: Some(Memory(29057, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[222, 54, 129, 113], OperandSize::Word)
}

#[test]
fn fidiv_5() {
    run_test(&Instruction { mnemonic: Mnemonic::FIDIV, operand1: Some(IndirectScaledIndexed(EDI, EDI, Eight, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[222, 52, 255], OperandSize::Dword)
}

#[test]
fn fidiv_6() {
    run_test(&Instruction { mnemonic: Mnemonic::FIDIV, operand1: Some(IndirectScaledIndexed(RSI, RDX, Four, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[222, 52, 150], OperandSize::Qword)
}

