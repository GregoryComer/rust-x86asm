use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn setbe_1() {
    run_test(&Instruction { mnemonic: Mnemonic::SETBE, operand1: Some(Direct(BL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 150, 195], OperandSize::Word)
}

#[test]
fn setbe_2() {
    run_test(&Instruction { mnemonic: Mnemonic::SETBE, operand1: Some(IndirectScaledIndexed(BX, DI, One, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 150, 1], OperandSize::Word)
}

#[test]
fn setbe_3() {
    run_test(&Instruction { mnemonic: Mnemonic::SETBE, operand1: Some(Direct(DL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 150, 194], OperandSize::Dword)
}

#[test]
fn setbe_4() {
    run_test(&Instruction { mnemonic: Mnemonic::SETBE, operand1: Some(IndirectScaledIndexed(ESI, EDX, Eight, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 150, 4, 214], OperandSize::Dword)
}

#[test]
fn setbe_5() {
    run_test(&Instruction { mnemonic: Mnemonic::SETBE, operand1: Some(Direct(BL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 150, 195], OperandSize::Qword)
}

#[test]
fn setbe_6() {
    run_test(&Instruction { mnemonic: Mnemonic::SETBE, operand1: Some(IndirectScaledIndexedDisplaced(RDI, RCX, Two, 592400139, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 150, 132, 79, 11, 79, 79, 35], OperandSize::Qword)
}

#[test]
fn setbe_7() {
    run_test(&Instruction { mnemonic: Mnemonic::SETBE, operand1: Some(Direct(BL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 150, 195], OperandSize::Qword)
}

#[test]
fn setbe_8() {
    run_test(&Instruction { mnemonic: Mnemonic::SETBE, operand1: Some(IndirectScaledDisplaced(RBX, Four, 1324405918, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 150, 4, 157, 158, 212, 240, 78], OperandSize::Qword)
}

