use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn bndmov_1() {
    run_test(&Instruction { mnemonic: Mnemonic::BNDMOV, operand1: Some(Direct(BND0)), operand2: Some(Direct(BND1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 26, 193], OperandSize::Dword)
}

#[test]
fn bndmov_2() {
    run_test(&Instruction { mnemonic: Mnemonic::BNDMOV, operand1: Some(Direct(BND2)), operand2: Some(IndirectDisplaced(EBX, 33911286, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 26, 147, 246, 113, 5, 2], OperandSize::Dword)
}

#[test]
fn bndmov_3() {
    run_test(&Instruction { mnemonic: Mnemonic::BNDMOV, operand1: Some(Direct(BND0)), operand2: Some(Direct(BND2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 26, 194], OperandSize::Qword)
}

#[test]
fn bndmov_4() {
    run_test(&Instruction { mnemonic: Mnemonic::BNDMOV, operand1: Some(Direct(BND0)), operand2: Some(IndirectScaledIndexedDisplaced(RBX, RCX, Four, 1511004038, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 26, 132, 139, 134, 23, 16, 90], OperandSize::Qword)
}

#[test]
fn bndmov_5() {
    run_test(&Instruction { mnemonic: Mnemonic::BNDMOV, operand1: Some(Direct(BND2)), operand2: Some(Direct(BND3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 26, 211], OperandSize::Dword)
}

#[test]
fn bndmov_6() {
    run_test(&Instruction { mnemonic: Mnemonic::BNDMOV, operand1: Some(Indirect(EAX, Some(OperandSize::Qword), None)), operand2: Some(Direct(BND2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 27, 16], OperandSize::Dword)
}

#[test]
fn bndmov_7() {
    run_test(&Instruction { mnemonic: Mnemonic::BNDMOV, operand1: Some(Direct(BND1)), operand2: Some(Direct(BND0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 26, 200], OperandSize::Qword)
}

#[test]
fn bndmov_8() {
    run_test(&Instruction { mnemonic: Mnemonic::BNDMOV, operand1: Some(IndirectScaledIndexed(RSI, RDI, Eight, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(BND1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 27, 12, 254], OperandSize::Qword)
}

