use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn fidivr_1() {
    run_test(&Instruction { mnemonic: Mnemonic::FIDIVR, operand1: Some(IndirectScaledIndexed(BX, SI, One, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[218, 56], OperandSize::Word)
}

#[test]
fn fidivr_2() {
    run_test(&Instruction { mnemonic: Mnemonic::FIDIVR, operand1: Some(IndirectScaledDisplaced(EDI, Two, 564683961, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[218, 60, 125, 185, 100, 168, 33], OperandSize::Dword)
}

#[test]
fn fidivr_3() {
    run_test(&Instruction { mnemonic: Mnemonic::FIDIVR, operand1: Some(IndirectScaledIndexed(RAX, RSI, Eight, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[218, 60, 240], OperandSize::Qword)
}

#[test]
fn fidivr_4() {
    run_test(&Instruction { mnemonic: Mnemonic::FIDIVR, operand1: Some(IndirectDisplaced(SI, 117, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[222, 124, 117], OperandSize::Word)
}

#[test]
fn fidivr_5() {
    run_test(&Instruction { mnemonic: Mnemonic::FIDIVR, operand1: Some(Indirect(EBX, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[222, 59], OperandSize::Dword)
}

#[test]
fn fidivr_6() {
    run_test(&Instruction { mnemonic: Mnemonic::FIDIVR, operand1: Some(IndirectScaledIndexed(RAX, RDX, Four, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[222, 60, 144], OperandSize::Qword)
}

