use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn fmul_1() {
    run_test(&Instruction { mnemonic: Mnemonic::FMUL, operand1: Some(IndirectScaledIndexedDisplaced(BP, DI, One, 12506, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[216, 139, 218, 48], OperandSize::Word)
}

#[test]
fn fmul_2() {
    run_test(&Instruction { mnemonic: Mnemonic::FMUL, operand1: Some(IndirectScaledIndexedDisplaced(ECX, EBX, Eight, 1946163245, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[216, 140, 217, 45, 24, 0, 116], OperandSize::Dword)
}

#[test]
fn fmul_3() {
    run_test(&Instruction { mnemonic: Mnemonic::FMUL, operand1: Some(IndirectScaledIndexedDisplaced(RDI, RDX, Eight, 1358002082, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[216, 140, 215, 162, 119, 241, 80], OperandSize::Qword)
}

#[test]
fn fmul_4() {
    run_test(&Instruction { mnemonic: Mnemonic::FMUL, operand1: Some(Direct(ST)), operand2: Some(Direct(ST4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[216, 204], OperandSize::Word)
}

#[test]
fn fmul_5() {
    run_test(&Instruction { mnemonic: Mnemonic::FMUL, operand1: Some(Direct(ST)), operand2: Some(Direct(ST4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[216, 204], OperandSize::Dword)
}

#[test]
fn fmul_6() {
    run_test(&Instruction { mnemonic: Mnemonic::FMUL, operand1: Some(Direct(ST)), operand2: Some(Direct(ST4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[216, 204], OperandSize::Qword)
}

#[test]
fn fmul_7() {
    run_test(&Instruction { mnemonic: Mnemonic::FMUL, operand1: Some(IndirectScaledIndexedDisplaced(BP, SI, One, 8025, Some(OperandSize::Qword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[220, 138, 89, 31], OperandSize::Word)
}

#[test]
fn fmul_8() {
    run_test(&Instruction { mnemonic: Mnemonic::FMUL, operand1: Some(IndirectScaledIndexed(ECX, ECX, Two, Some(OperandSize::Qword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[220, 12, 73], OperandSize::Dword)
}

#[test]
fn fmul_9() {
    run_test(&Instruction { mnemonic: Mnemonic::FMUL, operand1: Some(IndirectDisplaced(RAX, 1783883255, Some(OperandSize::Qword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[220, 136, 247, 229, 83, 106], OperandSize::Qword)
}

#[test]
fn fmul_10() {
    run_test(&Instruction { mnemonic: Mnemonic::FMUL, operand1: Some(Direct(ST1)), operand2: Some(Direct(ST)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[220, 201], OperandSize::Word)
}

#[test]
fn fmul_11() {
    run_test(&Instruction { mnemonic: Mnemonic::FMUL, operand1: Some(Direct(ST7)), operand2: Some(Direct(ST)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[220, 207], OperandSize::Dword)
}

#[test]
fn fmul_12() {
    run_test(&Instruction { mnemonic: Mnemonic::FMUL, operand1: Some(Direct(ST1)), operand2: Some(Direct(ST)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[220, 201], OperandSize::Qword)
}

