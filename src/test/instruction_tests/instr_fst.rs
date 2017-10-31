use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn fst_1() {
    run_test(&Instruction { mnemonic: Mnemonic::FST, operand1: Some(IndirectScaledIndexedDisplaced(BP, SI, One, 176, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[217, 146, 176, 0], OperandSize::Word)
}

#[test]
fn fst_2() {
    run_test(&Instruction { mnemonic: Mnemonic::FST, operand1: Some(IndirectScaledDisplaced(EBX, Eight, 1422910850, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[217, 20, 221, 130, 229, 207, 84], OperandSize::Dword)
}

#[test]
fn fst_3() {
    run_test(&Instruction { mnemonic: Mnemonic::FST, operand1: Some(IndirectScaledDisplaced(RDI, Two, 926747465, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[217, 20, 125, 73, 11, 61, 55], OperandSize::Qword)
}

#[test]
fn fst_4() {
    run_test(&Instruction { mnemonic: Mnemonic::FST, operand1: Some(IndirectScaledIndexedDisplaced(BP, DI, One, 12260, Some(OperandSize::Qword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[221, 147, 228, 47], OperandSize::Word)
}

#[test]
fn fst_5() {
    run_test(&Instruction { mnemonic: Mnemonic::FST, operand1: Some(IndirectDisplaced(ESI, 261016151, Some(OperandSize::Qword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[221, 150, 87, 202, 142, 15], OperandSize::Dword)
}

#[test]
fn fst_6() {
    run_test(&Instruction { mnemonic: Mnemonic::FST, operand1: Some(IndirectScaledIndexedDisplaced(RCX, RSI, Four, 2074710398, Some(OperandSize::Qword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[221, 148, 177, 126, 145, 169, 123], OperandSize::Qword)
}

#[test]
fn fst_7() {
    run_test(&Instruction { mnemonic: Mnemonic::FST, operand1: Some(Direct(ST4)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[221, 212], OperandSize::Word)
}

#[test]
fn fst_8() {
    run_test(&Instruction { mnemonic: Mnemonic::FST, operand1: Some(Direct(ST7)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[221, 215], OperandSize::Dword)
}

#[test]
fn fst_9() {
    run_test(&Instruction { mnemonic: Mnemonic::FST, operand1: Some(Direct(ST5)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[221, 213], OperandSize::Qword)
}

