use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn fsubr_1() {
    run_test(&Instruction { mnemonic: Mnemonic::FSUBR, operand1: Some(IndirectDisplaced(SI, 11899, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[216, 172, 123, 46], OperandSize::Word)
}

#[test]
fn fsubr_2() {
    run_test(&Instruction { mnemonic: Mnemonic::FSUBR, operand1: Some(IndirectDisplaced(EDI, 652356203, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[216, 175, 107, 42, 226, 38], OperandSize::Dword)
}

#[test]
fn fsubr_3() {
    run_test(&Instruction { mnemonic: Mnemonic::FSUBR, operand1: Some(IndirectScaledIndexedDisplaced(RBX, RBX, Four, 300258521, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[216, 172, 155, 217, 148, 229, 17], OperandSize::Qword)
}

#[test]
fn fsubr_4() {
    run_test(&Instruction { mnemonic: Mnemonic::FSUBR, operand1: Some(Direct(ST)), operand2: Some(Direct(ST7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[216, 239], OperandSize::Word)
}

#[test]
fn fsubr_5() {
    run_test(&Instruction { mnemonic: Mnemonic::FSUBR, operand1: Some(Direct(ST)), operand2: Some(Direct(ST5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[216, 237], OperandSize::Dword)
}

#[test]
fn fsubr_6() {
    run_test(&Instruction { mnemonic: Mnemonic::FSUBR, operand1: Some(Direct(ST)), operand2: Some(Direct(ST4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[216, 236], OperandSize::Qword)
}

#[test]
fn fsubr_7() {
    run_test(&Instruction { mnemonic: Mnemonic::FSUBR, operand1: Some(IndirectDisplaced(BX, 14, Some(OperandSize::Qword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[220, 111, 14], OperandSize::Word)
}

#[test]
fn fsubr_8() {
    run_test(&Instruction { mnemonic: Mnemonic::FSUBR, operand1: Some(IndirectScaledDisplaced(ECX, Four, 1772204062, Some(OperandSize::Qword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[220, 44, 141, 30, 176, 161, 105], OperandSize::Dword)
}

#[test]
fn fsubr_9() {
    run_test(&Instruction { mnemonic: Mnemonic::FSUBR, operand1: Some(IndirectDisplaced(RDI, 420533644, Some(OperandSize::Qword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[220, 175, 140, 213, 16, 25], OperandSize::Qword)
}

#[test]
fn fsubr_10() {
    run_test(&Instruction { mnemonic: Mnemonic::FSUBR, operand1: Some(Direct(ST7)), operand2: Some(Direct(ST)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[220, 231], OperandSize::Word)
}

#[test]
fn fsubr_11() {
    run_test(&Instruction { mnemonic: Mnemonic::FSUBR, operand1: Some(Direct(ST3)), operand2: Some(Direct(ST)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[220, 227], OperandSize::Dword)
}

#[test]
fn fsubr_12() {
    run_test(&Instruction { mnemonic: Mnemonic::FSUBR, operand1: Some(Direct(ST3)), operand2: Some(Direct(ST)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[220, 227], OperandSize::Qword)
}

