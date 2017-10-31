use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn fimul_1() {
    run_test(&Instruction { mnemonic: Mnemonic::FIMUL, operand1: Some(Indirect(DI, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[218, 13], OperandSize::Word)
}

#[test]
fn fimul_2() {
    run_test(&Instruction { mnemonic: Mnemonic::FIMUL, operand1: Some(IndirectScaledDisplaced(EDI, Four, 2146040278, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[218, 12, 189, 214, 249, 233, 127], OperandSize::Dword)
}

#[test]
fn fimul_3() {
    run_test(&Instruction { mnemonic: Mnemonic::FIMUL, operand1: Some(IndirectScaledIndexed(RSI, RSI, Four, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[218, 12, 182], OperandSize::Qword)
}

#[test]
fn fimul_4() {
    run_test(&Instruction { mnemonic: Mnemonic::FIMUL, operand1: Some(IndirectDisplaced(DI, 14344, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[222, 141, 8, 56], OperandSize::Word)
}

#[test]
fn fimul_5() {
    run_test(&Instruction { mnemonic: Mnemonic::FIMUL, operand1: Some(IndirectScaledDisplaced(EAX, Eight, 683337662, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[222, 12, 197, 190, 231, 186, 40], OperandSize::Dword)
}

#[test]
fn fimul_6() {
    run_test(&Instruction { mnemonic: Mnemonic::FIMUL, operand1: Some(IndirectDisplaced(RCX, 2086613587, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[222, 137, 83, 50, 95, 124], OperandSize::Qword)
}

