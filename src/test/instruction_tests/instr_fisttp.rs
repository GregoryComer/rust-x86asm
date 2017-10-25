use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn fisttp_1() {
    run_test(&Instruction { mnemonic: Mnemonic::FISTTP, operand1: Some(IndirectScaledIndexedDisplaced(BP, DI, One, 135, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[219, 139, 135, 0], OperandSize::Word)
}

#[test]
fn fisttp_2() {
    run_test(&Instruction { mnemonic: Mnemonic::FISTTP, operand1: Some(IndirectScaledDisplaced(EBX, Four, 137929979, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[219, 12, 157, 251, 164, 56, 8], OperandSize::Dword)
}

#[test]
fn fisttp_3() {
    run_test(&Instruction { mnemonic: Mnemonic::FISTTP, operand1: Some(Indirect(RCX, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[219, 9], OperandSize::Qword)
}

#[test]
fn fisttp_4() {
    run_test(&Instruction { mnemonic: Mnemonic::FISTTP, operand1: Some(IndirectScaledIndexed(BX, DI, One, Some(OperandSize::Qword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[221, 9], OperandSize::Word)
}

#[test]
fn fisttp_5() {
    run_test(&Instruction { mnemonic: Mnemonic::FISTTP, operand1: Some(IndirectScaledIndexedDisplaced(EBX, EDX, Eight, 354586733, Some(OperandSize::Qword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[221, 140, 211, 109, 144, 34, 21], OperandSize::Dword)
}

#[test]
fn fisttp_6() {
    run_test(&Instruction { mnemonic: Mnemonic::FISTTP, operand1: Some(IndirectScaledIndexed(RAX, RAX, Four, Some(OperandSize::Qword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[221, 12, 128], OperandSize::Qword)
}

#[test]
fn fisttp_7() {
    run_test(&Instruction { mnemonic: Mnemonic::FISTTP, operand1: Some(IndirectScaledIndexed(BX, DI, One, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[223, 9], OperandSize::Word)
}

#[test]
fn fisttp_8() {
    run_test(&Instruction { mnemonic: Mnemonic::FISTTP, operand1: Some(IndirectDisplaced(EAX, 947520014, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[223, 136, 14, 2, 122, 56], OperandSize::Dword)
}

#[test]
fn fisttp_9() {
    run_test(&Instruction { mnemonic: Mnemonic::FISTTP, operand1: Some(IndirectScaledIndexedDisplaced(RAX, RDI, Four, 1371574572, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[223, 140, 184, 44, 145, 192, 81], OperandSize::Qword)
}

