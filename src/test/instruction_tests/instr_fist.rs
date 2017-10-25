use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn fist_1() {
    run_test(&Instruction { mnemonic: Mnemonic::FIST, operand1: Some(IndirectDisplaced(BP, 15598, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[219, 150, 238, 60], OperandSize::Word)
}

#[test]
fn fist_2() {
    run_test(&Instruction { mnemonic: Mnemonic::FIST, operand1: Some(IndirectScaledDisplaced(EDX, Four, 770985611, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[219, 20, 149, 139, 78, 244, 45], OperandSize::Dword)
}

#[test]
fn fist_3() {
    run_test(&Instruction { mnemonic: Mnemonic::FIST, operand1: Some(IndirectScaledDisplaced(RBX, Two, 875286082, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[219, 20, 93, 66, 206, 43, 52], OperandSize::Qword)
}

#[test]
fn fist_4() {
    run_test(&Instruction { mnemonic: Mnemonic::FIST, operand1: Some(IndirectDisplaced(BX, 41, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[223, 87, 41], OperandSize::Word)
}

#[test]
fn fist_5() {
    run_test(&Instruction { mnemonic: Mnemonic::FIST, operand1: Some(IndirectScaledDisplaced(EDX, Four, 1404833680, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[223, 20, 149, 144, 15, 188, 83], OperandSize::Dword)
}

#[test]
fn fist_6() {
    run_test(&Instruction { mnemonic: Mnemonic::FIST, operand1: Some(Indirect(RSI, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[223, 22], OperandSize::Qword)
}

