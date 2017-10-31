use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn fsubr_1() {
    run_test(&Instruction { mnemonic: Mnemonic::FSUBR, operand1: Some(IndirectScaledIndexedDisplaced(BX, DI, One, 246, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[216, 169, 246, 0], OperandSize::Word)
}

#[test]
fn fsubr_2() {
    run_test(&Instruction { mnemonic: Mnemonic::FSUBR, operand1: Some(IndirectDisplaced(EAX, 1483639556, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[216, 168, 4, 139, 110, 88], OperandSize::Dword)
}

#[test]
fn fsubr_3() {
    run_test(&Instruction { mnemonic: Mnemonic::FSUBR, operand1: Some(IndirectScaledIndexed(RCX, RCX, Four, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[216, 44, 137], OperandSize::Qword)
}

#[test]
fn fsubr_4() {
    run_test(&Instruction { mnemonic: Mnemonic::FSUBR, operand1: Some(Direct(ST)), operand2: Some(Direct(ST1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[216, 233], OperandSize::Word)
}

#[test]
fn fsubr_5() {
    run_test(&Instruction { mnemonic: Mnemonic::FSUBR, operand1: Some(Direct(ST)), operand2: Some(Direct(ST1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[216, 233], OperandSize::Dword)
}

#[test]
fn fsubr_6() {
    run_test(&Instruction { mnemonic: Mnemonic::FSUBR, operand1: Some(Direct(ST)), operand2: Some(Direct(ST7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[216, 239], OperandSize::Qword)
}

#[test]
fn fsubr_7() {
    run_test(&Instruction { mnemonic: Mnemonic::FSUBR, operand1: Some(IndirectScaledIndexedDisplaced(BP, SI, One, 11149, Some(OperandSize::Qword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[220, 170, 141, 43], OperandSize::Word)
}

#[test]
fn fsubr_8() {
    run_test(&Instruction { mnemonic: Mnemonic::FSUBR, operand1: Some(IndirectScaledIndexedDisplaced(EBX, EBX, Four, 2134363001, Some(OperandSize::Qword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[220, 172, 155, 121, 203, 55, 127], OperandSize::Dword)
}

#[test]
fn fsubr_9() {
    run_test(&Instruction { mnemonic: Mnemonic::FSUBR, operand1: Some(IndirectScaledIndexed(RDX, RBX, Two, Some(OperandSize::Qword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[220, 44, 90], OperandSize::Qword)
}

#[test]
fn fsubr_10() {
    run_test(&Instruction { mnemonic: Mnemonic::FSUBR, operand1: Some(Direct(ST3)), operand2: Some(Direct(ST)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[220, 227], OperandSize::Word)
}

#[test]
fn fsubr_11() {
    run_test(&Instruction { mnemonic: Mnemonic::FSUBR, operand1: Some(Direct(ST3)), operand2: Some(Direct(ST)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[220, 227], OperandSize::Dword)
}

#[test]
fn fsubr_12() {
    run_test(&Instruction { mnemonic: Mnemonic::FSUBR, operand1: Some(Direct(ST4)), operand2: Some(Direct(ST)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[220, 228], OperandSize::Qword)
}

