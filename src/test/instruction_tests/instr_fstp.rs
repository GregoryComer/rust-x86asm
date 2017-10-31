use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn fstp_1() {
    run_test(&Instruction { mnemonic: Mnemonic::FSTP, operand1: Some(IndirectScaledIndexed(BP, DI, One, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[217, 27], OperandSize::Word)
}

#[test]
fn fstp_2() {
    run_test(&Instruction { mnemonic: Mnemonic::FSTP, operand1: Some(IndirectDisplaced(EDX, 1527225350, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[217, 154, 6, 156, 7, 91], OperandSize::Dword)
}

#[test]
fn fstp_3() {
    run_test(&Instruction { mnemonic: Mnemonic::FSTP, operand1: Some(IndirectScaledIndexedDisplaced(RDX, RBX, Four, 1191880455, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[217, 156, 154, 7, 167, 10, 71], OperandSize::Qword)
}

#[test]
fn fstp_4() {
    run_test(&Instruction { mnemonic: Mnemonic::FSTP, operand1: Some(IndirectDisplaced(DI, 61, Some(OperandSize::Tbyte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[219, 125, 61], OperandSize::Word)
}

#[test]
fn fstp_5() {
    run_test(&Instruction { mnemonic: Mnemonic::FSTP, operand1: Some(IndirectScaledDisplaced(EAX, Two, 1826263834, Some(OperandSize::Tbyte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[219, 60, 69, 26, 147, 218, 108], OperandSize::Dword)
}

#[test]
fn fstp_6() {
    run_test(&Instruction { mnemonic: Mnemonic::FSTP, operand1: Some(IndirectScaledDisplaced(RDX, Two, 1411746871, Some(OperandSize::Tbyte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[219, 60, 85, 55, 140, 37, 84], OperandSize::Qword)
}

#[test]
fn fstp_7() {
    run_test(&Instruction { mnemonic: Mnemonic::FSTP, operand1: Some(Indirect(BX, Some(OperandSize::Qword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[221, 31], OperandSize::Word)
}

#[test]
fn fstp_8() {
    run_test(&Instruction { mnemonic: Mnemonic::FSTP, operand1: Some(IndirectScaledIndexedDisplaced(EBX, EBX, Two, 1111488362, Some(OperandSize::Qword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[221, 156, 91, 106, 247, 63, 66], OperandSize::Dword)
}

#[test]
fn fstp_9() {
    run_test(&Instruction { mnemonic: Mnemonic::FSTP, operand1: Some(IndirectScaledIndexed(RSI, RDI, Two, Some(OperandSize::Qword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[221, 28, 126], OperandSize::Qword)
}

#[test]
fn fstp_10() {
    run_test(&Instruction { mnemonic: Mnemonic::FSTP, operand1: Some(Direct(ST7)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[221, 223], OperandSize::Word)
}

#[test]
fn fstp_11() {
    run_test(&Instruction { mnemonic: Mnemonic::FSTP, operand1: Some(Direct(ST1)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[221, 217], OperandSize::Dword)
}

#[test]
fn fstp_12() {
    run_test(&Instruction { mnemonic: Mnemonic::FSTP, operand1: Some(Direct(ST3)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[221, 219], OperandSize::Qword)
}

