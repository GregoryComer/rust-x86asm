use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn setnb_1() {
    run_test(&Instruction { mnemonic: Mnemonic::SETNB, operand1: Some(Direct(BL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 147, 195], OperandSize::Word)
}

#[test]
fn setnb_2() {
    run_test(&Instruction { mnemonic: Mnemonic::SETNB, operand1: Some(Memory(5596, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 147, 6, 220, 21], OperandSize::Word)
}

#[test]
fn setnb_3() {
    run_test(&Instruction { mnemonic: Mnemonic::SETNB, operand1: Some(Direct(BL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 147, 195], OperandSize::Dword)
}

#[test]
fn setnb_4() {
    run_test(&Instruction { mnemonic: Mnemonic::SETNB, operand1: Some(IndirectDisplaced(EAX, 284819900, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 147, 128, 188, 1, 250, 16], OperandSize::Dword)
}

#[test]
fn setnb_5() {
    run_test(&Instruction { mnemonic: Mnemonic::SETNB, operand1: Some(Direct(DL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 147, 194], OperandSize::Qword)
}

#[test]
fn setnb_6() {
    run_test(&Instruction { mnemonic: Mnemonic::SETNB, operand1: Some(IndirectScaledIndexedDisplaced(RAX, RAX, Four, 268301335, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 147, 132, 128, 23, 244, 253, 15], OperandSize::Qword)
}

#[test]
fn setnb_7() {
    run_test(&Instruction { mnemonic: Mnemonic::SETNB, operand1: Some(Direct(CL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 147, 193], OperandSize::Qword)
}

#[test]
fn setnb_8() {
    run_test(&Instruction { mnemonic: Mnemonic::SETNB, operand1: Some(IndirectScaledIndexed(RCX, RDX, Two, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 147, 4, 81], OperandSize::Qword)
}

