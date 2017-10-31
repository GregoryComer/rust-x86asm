use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn fmul_1() {
    run_test(&Instruction { mnemonic: Mnemonic::FMUL, operand1: Some(IndirectDisplaced(BX, 133, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[216, 143, 133, 0], OperandSize::Word)
}

#[test]
fn fmul_2() {
    run_test(&Instruction { mnemonic: Mnemonic::FMUL, operand1: Some(IndirectScaledIndexedDisplaced(ESI, EDX, Four, 57497739, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[216, 140, 150, 139, 88, 109, 3], OperandSize::Dword)
}

#[test]
fn fmul_3() {
    run_test(&Instruction { mnemonic: Mnemonic::FMUL, operand1: Some(IndirectDisplaced(RDX, 1919007390, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[216, 138, 158, 186, 97, 114], OperandSize::Qword)
}

#[test]
fn fmul_4() {
    run_test(&Instruction { mnemonic: Mnemonic::FMUL, operand1: Some(Direct(ST)), operand2: Some(Direct(ST4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[216, 204], OperandSize::Word)
}

#[test]
fn fmul_5() {
    run_test(&Instruction { mnemonic: Mnemonic::FMUL, operand1: Some(Direct(ST)), operand2: Some(Direct(ST2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[216, 202], OperandSize::Dword)
}

#[test]
fn fmul_6() {
    run_test(&Instruction { mnemonic: Mnemonic::FMUL, operand1: Some(Direct(ST)), operand2: Some(Direct(ST6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[216, 206], OperandSize::Qword)
}

#[test]
fn fmul_7() {
    run_test(&Instruction { mnemonic: Mnemonic::FMUL, operand1: Some(IndirectDisplaced(DI, 532, Some(OperandSize::Qword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[220, 141, 20, 2], OperandSize::Word)
}

#[test]
fn fmul_8() {
    run_test(&Instruction { mnemonic: Mnemonic::FMUL, operand1: Some(IndirectScaledIndexedDisplaced(EAX, EDX, Two, 1990506617, Some(OperandSize::Qword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[220, 140, 80, 121, 184, 164, 118], OperandSize::Dword)
}

#[test]
fn fmul_9() {
    run_test(&Instruction { mnemonic: Mnemonic::FMUL, operand1: Some(Indirect(RAX, Some(OperandSize::Qword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[220, 8], OperandSize::Qword)
}

#[test]
fn fmul_10() {
    run_test(&Instruction { mnemonic: Mnemonic::FMUL, operand1: Some(Direct(ST1)), operand2: Some(Direct(ST)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[220, 201], OperandSize::Word)
}

#[test]
fn fmul_11() {
    run_test(&Instruction { mnemonic: Mnemonic::FMUL, operand1: Some(Direct(ST3)), operand2: Some(Direct(ST)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[220, 203], OperandSize::Dword)
}

#[test]
fn fmul_12() {
    run_test(&Instruction { mnemonic: Mnemonic::FMUL, operand1: Some(Direct(ST1)), operand2: Some(Direct(ST)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[220, 201], OperandSize::Qword)
}

