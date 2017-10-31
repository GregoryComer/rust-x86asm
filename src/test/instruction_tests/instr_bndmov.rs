use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn bndmov_1() {
    run_test(&Instruction { mnemonic: Mnemonic::BNDMOV, operand1: Some(Direct(BND1)), operand2: Some(Direct(BND1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 26, 201], OperandSize::Dword)
}

#[test]
fn bndmov_2() {
    run_test(&Instruction { mnemonic: Mnemonic::BNDMOV, operand1: Some(Direct(BND2)), operand2: Some(IndirectScaledIndexedDisplaced(ESI, ECX, Eight, 1707127893, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 26, 148, 206, 85, 180, 192, 101], OperandSize::Dword)
}

#[test]
fn bndmov_3() {
    run_test(&Instruction { mnemonic: Mnemonic::BNDMOV, operand1: Some(Direct(BND2)), operand2: Some(Direct(BND3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 26, 211], OperandSize::Qword)
}

#[test]
fn bndmov_4() {
    run_test(&Instruction { mnemonic: Mnemonic::BNDMOV, operand1: Some(Direct(BND3)), operand2: Some(Indirect(RDX, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 26, 26], OperandSize::Qword)
}

#[test]
fn bndmov_5() {
    run_test(&Instruction { mnemonic: Mnemonic::BNDMOV, operand1: Some(Direct(BND1)), operand2: Some(Direct(BND0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 26, 200], OperandSize::Dword)
}

#[test]
fn bndmov_6() {
    run_test(&Instruction { mnemonic: Mnemonic::BNDMOV, operand1: Some(IndirectScaledIndexedDisplaced(ECX, EBX, Two, 2087225676, Some(OperandSize::Qword), None)), operand2: Some(Direct(BND2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 27, 148, 89, 76, 137, 104, 124], OperandSize::Dword)
}

#[test]
fn bndmov_7() {
    run_test(&Instruction { mnemonic: Mnemonic::BNDMOV, operand1: Some(Direct(BND3)), operand2: Some(Direct(BND0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 26, 216], OperandSize::Qword)
}

#[test]
fn bndmov_8() {
    run_test(&Instruction { mnemonic: Mnemonic::BNDMOV, operand1: Some(IndirectScaledIndexed(RSI, RSI, Two, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(BND1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 27, 12, 118], OperandSize::Qword)
}

