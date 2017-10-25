use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn fstp_1() {
    run_test(&Instruction { mnemonic: Mnemonic::FSTP, operand1: Some(IndirectScaledIndexed(BX, DI, One, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[217, 25], OperandSize::Word)
}

#[test]
fn fstp_2() {
    run_test(&Instruction { mnemonic: Mnemonic::FSTP, operand1: Some(IndirectScaledIndexedDisplaced(EDX, EDX, Four, 168742134, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[217, 156, 146, 246, 204, 14, 10], OperandSize::Dword)
}

#[test]
fn fstp_3() {
    run_test(&Instruction { mnemonic: Mnemonic::FSTP, operand1: Some(IndirectScaledIndexedDisplaced(RAX, RCX, Two, 839816464, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[217, 156, 72, 16, 149, 14, 50], OperandSize::Qword)
}

#[test]
fn fstp_4() {
    run_test(&Instruction { mnemonic: Mnemonic::FSTP, operand1: Some(IndirectScaledIndexed(BX, DI, One, Some(OperandSize::Tbyte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[219, 57], OperandSize::Word)
}

#[test]
fn fstp_5() {
    run_test(&Instruction { mnemonic: Mnemonic::FSTP, operand1: Some(IndirectDisplaced(EBX, 872989243, Some(OperandSize::Tbyte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[219, 187, 59, 194, 8, 52], OperandSize::Dword)
}

#[test]
fn fstp_6() {
    run_test(&Instruction { mnemonic: Mnemonic::FSTP, operand1: Some(IndirectDisplaced(RDX, 977127858, Some(OperandSize::Tbyte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[219, 186, 178, 201, 61, 58], OperandSize::Qword)
}

#[test]
fn fstp_7() {
    run_test(&Instruction { mnemonic: Mnemonic::FSTP, operand1: Some(IndirectScaledIndexed(BX, SI, One, Some(OperandSize::Qword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[221, 24], OperandSize::Word)
}

#[test]
fn fstp_8() {
    run_test(&Instruction { mnemonic: Mnemonic::FSTP, operand1: Some(IndirectDisplaced(ESI, 2086841047, Some(OperandSize::Qword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[221, 158, 215, 170, 98, 124], OperandSize::Dword)
}

#[test]
fn fstp_9() {
    run_test(&Instruction { mnemonic: Mnemonic::FSTP, operand1: Some(IndirectScaledIndexedDisplaced(RSI, RCX, Two, 1720085614, Some(OperandSize::Qword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[221, 156, 78, 110, 108, 134, 102], OperandSize::Qword)
}

#[test]
fn fstp_10() {
    run_test(&Instruction { mnemonic: Mnemonic::FSTP, operand1: Some(Direct(ST4)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[221, 220], OperandSize::Word)
}

#[test]
fn fstp_11() {
    run_test(&Instruction { mnemonic: Mnemonic::FSTP, operand1: Some(Direct(ST6)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[221, 222], OperandSize::Dword)
}

#[test]
fn fstp_12() {
    run_test(&Instruction { mnemonic: Mnemonic::FSTP, operand1: Some(Direct(ST6)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[221, 222], OperandSize::Qword)
}

