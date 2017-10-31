use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn fisttp_1() {
    run_test(&Instruction { mnemonic: Mnemonic::FISTTP, operand1: Some(IndirectScaledIndexedDisplaced(BP, SI, One, 18, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[219, 74, 18], OperandSize::Word)
}

#[test]
fn fisttp_2() {
    run_test(&Instruction { mnemonic: Mnemonic::FISTTP, operand1: Some(Indirect(EBX, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[219, 11], OperandSize::Dword)
}

#[test]
fn fisttp_3() {
    run_test(&Instruction { mnemonic: Mnemonic::FISTTP, operand1: Some(IndirectScaledIndexedDisplaced(RDX, RAX, Two, 1238008023, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[219, 140, 66, 215, 128, 202, 73], OperandSize::Qword)
}

#[test]
fn fisttp_4() {
    run_test(&Instruction { mnemonic: Mnemonic::FISTTP, operand1: Some(IndirectScaledIndexedDisplaced(BP, DI, One, 153, Some(OperandSize::Qword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[221, 139, 153, 0], OperandSize::Word)
}

#[test]
fn fisttp_5() {
    run_test(&Instruction { mnemonic: Mnemonic::FISTTP, operand1: Some(Indirect(EAX, Some(OperandSize::Qword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[221, 8], OperandSize::Dword)
}

#[test]
fn fisttp_6() {
    run_test(&Instruction { mnemonic: Mnemonic::FISTTP, operand1: Some(IndirectScaledDisplaced(RBX, Two, 1052181335, Some(OperandSize::Qword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[221, 12, 93, 87, 3, 183, 62], OperandSize::Qword)
}

#[test]
fn fisttp_7() {
    run_test(&Instruction { mnemonic: Mnemonic::FISTTP, operand1: Some(IndirectScaledIndexedDisplaced(BX, SI, One, 2357, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[223, 136, 53, 9], OperandSize::Word)
}

#[test]
fn fisttp_8() {
    run_test(&Instruction { mnemonic: Mnemonic::FISTTP, operand1: Some(IndirectScaledIndexedDisplaced(EAX, EDX, Four, 1281354160, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[223, 140, 144, 176, 233, 95, 76], OperandSize::Dword)
}

#[test]
fn fisttp_9() {
    run_test(&Instruction { mnemonic: Mnemonic::FISTTP, operand1: Some(Indirect(RCX, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[223, 9], OperandSize::Qword)
}

