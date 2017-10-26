use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn fisttp_1() {
    run_test(&Instruction { mnemonic: Mnemonic::FISTTP, operand1: Some(IndirectScaledIndexed(BP, DI, One, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[219, 11], OperandSize::Word)
}

#[test]
fn fisttp_2() {
    run_test(&Instruction { mnemonic: Mnemonic::FISTTP, operand1: Some(IndirectScaledIndexed(ESI, ESI, Two, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[219, 12, 118], OperandSize::Dword)
}

#[test]
fn fisttp_3() {
    run_test(&Instruction { mnemonic: Mnemonic::FISTTP, operand1: Some(IndirectDisplaced(RDI, 1799837783, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[219, 143, 87, 88, 71, 107], OperandSize::Qword)
}

#[test]
fn fisttp_4() {
    run_test(&Instruction { mnemonic: Mnemonic::FISTTP, operand1: Some(IndirectScaledIndexed(BP, DI, One, Some(OperandSize::Qword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[221, 11], OperandSize::Word)
}

#[test]
fn fisttp_5() {
    run_test(&Instruction { mnemonic: Mnemonic::FISTTP, operand1: Some(IndirectDisplaced(EDX, 372719124, Some(OperandSize::Qword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[221, 138, 20, 62, 55, 22], OperandSize::Dword)
}

#[test]
fn fisttp_6() {
    run_test(&Instruction { mnemonic: Mnemonic::FISTTP, operand1: Some(IndirectScaledIndexedDisplaced(RAX, RDX, Eight, 10362421, Some(OperandSize::Qword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[221, 140, 208, 53, 30, 158, 0], OperandSize::Qword)
}

#[test]
fn fisttp_7() {
    run_test(&Instruction { mnemonic: Mnemonic::FISTTP, operand1: Some(IndirectDisplaced(BP, 155, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[223, 142, 155, 0], OperandSize::Word)
}

#[test]
fn fisttp_8() {
    run_test(&Instruction { mnemonic: Mnemonic::FISTTP, operand1: Some(IndirectScaledDisplaced(EBX, Eight, 1697323345, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[223, 12, 221, 81, 25, 43, 101], OperandSize::Dword)
}

#[test]
fn fisttp_9() {
    run_test(&Instruction { mnemonic: Mnemonic::FISTTP, operand1: Some(IndirectScaledIndexedDisplaced(RSI, RAX, Two, 271113087, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[223, 140, 70, 127, 219, 40, 16], OperandSize::Qword)
}

