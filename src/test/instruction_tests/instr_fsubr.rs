use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn fsubr_1() {
    run_test(&Instruction { mnemonic: Mnemonic::FSUBR, operand1: Some(IndirectDisplaced(BX, 195, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[216, 175, 195, 0], OperandSize::Word)
}

#[test]
fn fsubr_2() {
    run_test(&Instruction { mnemonic: Mnemonic::FSUBR, operand1: Some(IndirectDisplaced(ECX, 1283246439, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[216, 169, 103, 201, 124, 76], OperandSize::Dword)
}

#[test]
fn fsubr_3() {
    run_test(&Instruction { mnemonic: Mnemonic::FSUBR, operand1: Some(Indirect(RDI, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[216, 47], OperandSize::Qword)
}

#[test]
fn fsubr_4() {
    run_test(&Instruction { mnemonic: Mnemonic::FSUBR, operand1: Some(Direct(ST)), operand2: Some(Direct(ST1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[216, 233], OperandSize::Word)
}

#[test]
fn fsubr_5() {
    run_test(&Instruction { mnemonic: Mnemonic::FSUBR, operand1: Some(Direct(ST)), operand2: Some(Direct(ST3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[216, 235], OperandSize::Dword)
}

#[test]
fn fsubr_6() {
    run_test(&Instruction { mnemonic: Mnemonic::FSUBR, operand1: Some(Direct(ST)), operand2: Some(Direct(ST2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[216, 234], OperandSize::Qword)
}

#[test]
fn fsubr_7() {
    run_test(&Instruction { mnemonic: Mnemonic::FSUBR, operand1: Some(IndirectScaledIndexedDisplaced(BX, DI, One, 16509, Some(OperandSize::Qword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[220, 169, 125, 64], OperandSize::Word)
}

#[test]
fn fsubr_8() {
    run_test(&Instruction { mnemonic: Mnemonic::FSUBR, operand1: Some(IndirectScaledDisplaced(ESI, Four, 399546404, Some(OperandSize::Qword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[220, 44, 181, 36, 152, 208, 23], OperandSize::Dword)
}

#[test]
fn fsubr_9() {
    run_test(&Instruction { mnemonic: Mnemonic::FSUBR, operand1: Some(IndirectScaledIndexed(RSI, RDX, Two, Some(OperandSize::Qword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[220, 44, 86], OperandSize::Qword)
}

#[test]
fn fsubr_10() {
    run_test(&Instruction { mnemonic: Mnemonic::FSUBR, operand1: Some(Direct(ST3)), operand2: Some(Direct(ST)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[220, 227], OperandSize::Word)
}

#[test]
fn fsubr_11() {
    run_test(&Instruction { mnemonic: Mnemonic::FSUBR, operand1: Some(Direct(ST4)), operand2: Some(Direct(ST)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[220, 228], OperandSize::Dword)
}

#[test]
fn fsubr_12() {
    run_test(&Instruction { mnemonic: Mnemonic::FSUBR, operand1: Some(Direct(ST3)), operand2: Some(Direct(ST)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[220, 227], OperandSize::Qword)
}

