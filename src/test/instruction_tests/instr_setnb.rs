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
    run_test(&Instruction { mnemonic: Mnemonic::SETNB, operand1: Some(IndirectScaledIndexed(BP, DI, One, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 147, 3], OperandSize::Word)
}

#[test]
fn setnb_3() {
    run_test(&Instruction { mnemonic: Mnemonic::SETNB, operand1: Some(Direct(DL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 147, 194], OperandSize::Dword)
}

#[test]
fn setnb_4() {
    run_test(&Instruction { mnemonic: Mnemonic::SETNB, operand1: Some(Indirect(EBX, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 147, 3], OperandSize::Dword)
}

#[test]
fn setnb_5() {
    run_test(&Instruction { mnemonic: Mnemonic::SETNB, operand1: Some(Direct(BL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 147, 195], OperandSize::Qword)
}

#[test]
fn setnb_6() {
    run_test(&Instruction { mnemonic: Mnemonic::SETNB, operand1: Some(Indirect(RAX, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 147, 0], OperandSize::Qword)
}

#[test]
fn setnb_7() {
    run_test(&Instruction { mnemonic: Mnemonic::SETNB, operand1: Some(Direct(BL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 147, 195], OperandSize::Qword)
}

#[test]
fn setnb_8() {
    run_test(&Instruction { mnemonic: Mnemonic::SETNB, operand1: Some(IndirectDisplaced(RAX, 1821066532, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 147, 128, 36, 69, 139, 108], OperandSize::Qword)
}

