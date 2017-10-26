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
    run_test(&Instruction { mnemonic: Mnemonic::SETBE, operand1: Some(IndirectDisplaced(BX, 229, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 150, 135, 229, 0], OperandSize::Word)
}

#[test]
fn setbe_3() {
    run_test(&Instruction { mnemonic: Mnemonic::SETBE, operand1: Some(Direct(BL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 150, 195], OperandSize::Dword)
}

#[test]
fn setbe_4() {
    run_test(&Instruction { mnemonic: Mnemonic::SETBE, operand1: Some(IndirectDisplaced(ECX, 366163427, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 150, 129, 227, 53, 211, 21], OperandSize::Dword)
}

#[test]
fn setbe_5() {
    run_test(&Instruction { mnemonic: Mnemonic::SETBE, operand1: Some(Direct(CL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 150, 193], OperandSize::Qword)
}

#[test]
fn setbe_6() {
    run_test(&Instruction { mnemonic: Mnemonic::SETBE, operand1: Some(IndirectScaledDisplaced(RAX, Four, 2081899246, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 150, 4, 133, 238, 66, 23, 124], OperandSize::Qword)
}

#[test]
fn setbe_7() {
    run_test(&Instruction { mnemonic: Mnemonic::SETBE, operand1: Some(Direct(CL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 150, 193], OperandSize::Qword)
}

#[test]
fn setbe_8() {
    run_test(&Instruction { mnemonic: Mnemonic::SETBE, operand1: Some(IndirectScaledDisplaced(RBX, Four, 2141204301, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 150, 4, 157, 77, 47, 160, 127], OperandSize::Qword)
}

