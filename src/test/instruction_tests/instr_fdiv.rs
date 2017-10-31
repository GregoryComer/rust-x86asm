use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn fdiv_1() {
    run_test(&Instruction { mnemonic: Mnemonic::FDIV, operand1: Some(IndirectScaledIndexedDisplaced(BP, SI, One, 141, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[216, 178, 141, 0], OperandSize::Word)
}

#[test]
fn fdiv_2() {
    run_test(&Instruction { mnemonic: Mnemonic::FDIV, operand1: Some(IndirectScaledIndexed(EDI, EDX, Four, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[216, 52, 151], OperandSize::Dword)
}

#[test]
fn fdiv_3() {
    run_test(&Instruction { mnemonic: Mnemonic::FDIV, operand1: Some(Indirect(RSI, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[216, 54], OperandSize::Qword)
}

#[test]
fn fdiv_4() {
    run_test(&Instruction { mnemonic: Mnemonic::FDIV, operand1: Some(Direct(ST)), operand2: Some(Direct(ST6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[216, 246], OperandSize::Word)
}

#[test]
fn fdiv_5() {
    run_test(&Instruction { mnemonic: Mnemonic::FDIV, operand1: Some(Direct(ST)), operand2: Some(Direct(ST4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[216, 244], OperandSize::Dword)
}

#[test]
fn fdiv_6() {
    run_test(&Instruction { mnemonic: Mnemonic::FDIV, operand1: Some(Direct(ST)), operand2: Some(Direct(ST1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[216, 241], OperandSize::Qword)
}

#[test]
fn fdiv_7() {
    run_test(&Instruction { mnemonic: Mnemonic::FDIV, operand1: Some(IndirectScaledIndexedDisplaced(BP, SI, One, 32555, Some(OperandSize::Qword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[220, 178, 43, 127], OperandSize::Word)
}

#[test]
fn fdiv_8() {
    run_test(&Instruction { mnemonic: Mnemonic::FDIV, operand1: Some(Indirect(EDI, Some(OperandSize::Qword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[220, 55], OperandSize::Dword)
}

#[test]
fn fdiv_9() {
    run_test(&Instruction { mnemonic: Mnemonic::FDIV, operand1: Some(Indirect(RCX, Some(OperandSize::Qword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[220, 49], OperandSize::Qword)
}

#[test]
fn fdiv_10() {
    run_test(&Instruction { mnemonic: Mnemonic::FDIV, operand1: Some(Direct(ST2)), operand2: Some(Direct(ST)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[220, 250], OperandSize::Word)
}

#[test]
fn fdiv_11() {
    run_test(&Instruction { mnemonic: Mnemonic::FDIV, operand1: Some(Direct(ST2)), operand2: Some(Direct(ST)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[220, 250], OperandSize::Dword)
}

#[test]
fn fdiv_12() {
    run_test(&Instruction { mnemonic: Mnemonic::FDIV, operand1: Some(Direct(ST5)), operand2: Some(Direct(ST)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[220, 253], OperandSize::Qword)
}

