use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn bndmov_1() {
    run_test(&Instruction { mnemonic: Mnemonic::BNDMOV, operand1: Some(Direct(BND0)), operand2: Some(Direct(BND3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 26, 195], OperandSize::Dword)
}

#[test]
fn bndmov_2() {
    run_test(&Instruction { mnemonic: Mnemonic::BNDMOV, operand1: Some(Direct(BND0)), operand2: Some(IndirectScaledIndexedDisplaced(ECX, EDX, Two, 770660997, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 26, 132, 81, 133, 90, 239, 45], OperandSize::Dword)
}

#[test]
fn bndmov_3() {
    run_test(&Instruction { mnemonic: Mnemonic::BNDMOV, operand1: Some(Direct(BND3)), operand2: Some(Direct(BND3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 26, 219], OperandSize::Qword)
}

#[test]
fn bndmov_4() {
    run_test(&Instruction { mnemonic: Mnemonic::BNDMOV, operand1: Some(Direct(BND2)), operand2: Some(IndirectDisplaced(RDX, 1827991535, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 26, 146, 239, 239, 244, 108], OperandSize::Qword)
}

#[test]
fn bndmov_5() {
    run_test(&Instruction { mnemonic: Mnemonic::BNDMOV, operand1: Some(Direct(BND1)), operand2: Some(Direct(BND1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 26, 201], OperandSize::Dword)
}

#[test]
fn bndmov_6() {
    run_test(&Instruction { mnemonic: Mnemonic::BNDMOV, operand1: Some(IndirectScaledIndexedDisplaced(EDX, EDI, Four, 1684282186, Some(OperandSize::Qword), None)), operand2: Some(Direct(BND1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 27, 140, 186, 74, 27, 100, 100], OperandSize::Dword)
}

#[test]
fn bndmov_7() {
    run_test(&Instruction { mnemonic: Mnemonic::BNDMOV, operand1: Some(Direct(BND2)), operand2: Some(Direct(BND1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 26, 209], OperandSize::Qword)
}

#[test]
fn bndmov_8() {
    run_test(&Instruction { mnemonic: Mnemonic::BNDMOV, operand1: Some(IndirectDisplaced(RDX, 1735362070, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(BND0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 27, 130, 22, 134, 111, 103], OperandSize::Qword)
}

