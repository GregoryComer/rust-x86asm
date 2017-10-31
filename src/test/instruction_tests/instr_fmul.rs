use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn fmul_1() {
    run_test(&Instruction { mnemonic: Mnemonic::FMUL, operand1: Some(IndirectDisplaced(DI, 22361, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[216, 141, 89, 87], OperandSize::Word)
}

#[test]
fn fmul_2() {
    run_test(&Instruction { mnemonic: Mnemonic::FMUL, operand1: Some(IndirectScaledIndexed(EBX, EBX, Two, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[216, 12, 91], OperandSize::Dword)
}

#[test]
fn fmul_3() {
    run_test(&Instruction { mnemonic: Mnemonic::FMUL, operand1: Some(IndirectScaledIndexedDisplaced(RCX, RSI, Four, 1384967910, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[216, 140, 177, 230, 238, 140, 82], OperandSize::Qword)
}

#[test]
fn fmul_4() {
    run_test(&Instruction { mnemonic: Mnemonic::FMUL, operand1: Some(Direct(ST)), operand2: Some(Direct(ST5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[216, 205], OperandSize::Word)
}

#[test]
fn fmul_5() {
    run_test(&Instruction { mnemonic: Mnemonic::FMUL, operand1: Some(Direct(ST)), operand2: Some(Direct(ST2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[216, 202], OperandSize::Dword)
}

#[test]
fn fmul_6() {
    run_test(&Instruction { mnemonic: Mnemonic::FMUL, operand1: Some(Direct(ST)), operand2: Some(Direct(ST5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[216, 205], OperandSize::Qword)
}

#[test]
fn fmul_7() {
    run_test(&Instruction { mnemonic: Mnemonic::FMUL, operand1: Some(IndirectScaledIndexedDisplaced(BP, DI, One, 14919, Some(OperandSize::Qword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[220, 139, 71, 58], OperandSize::Word)
}

#[test]
fn fmul_8() {
    run_test(&Instruction { mnemonic: Mnemonic::FMUL, operand1: Some(Indirect(EAX, Some(OperandSize::Qword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[220, 8], OperandSize::Dword)
}

#[test]
fn fmul_9() {
    run_test(&Instruction { mnemonic: Mnemonic::FMUL, operand1: Some(IndirectScaledIndexed(RAX, RSI, Four, Some(OperandSize::Qword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[220, 12, 176], OperandSize::Qword)
}

#[test]
fn fmul_10() {
    run_test(&Instruction { mnemonic: Mnemonic::FMUL, operand1: Some(Direct(ST5)), operand2: Some(Direct(ST)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[220, 205], OperandSize::Word)
}

#[test]
fn fmul_11() {
    run_test(&Instruction { mnemonic: Mnemonic::FMUL, operand1: Some(Direct(ST3)), operand2: Some(Direct(ST)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[220, 203], OperandSize::Dword)
}

#[test]
fn fmul_12() {
    run_test(&Instruction { mnemonic: Mnemonic::FMUL, operand1: Some(Direct(ST2)), operand2: Some(Direct(ST)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[220, 202], OperandSize::Qword)
}

