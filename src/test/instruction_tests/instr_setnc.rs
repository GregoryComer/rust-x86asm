use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn setnc_1() {
    run_test(&Instruction { mnemonic: Mnemonic::SETNC, operand1: Some(Direct(BL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 147, 195], OperandSize::Word)
}

#[test]
fn setnc_2() {
    run_test(&Instruction { mnemonic: Mnemonic::SETNC, operand1: Some(IndirectScaledIndexedDisplaced(BP, SI, One, 29196, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 147, 130, 12, 114], OperandSize::Word)
}

#[test]
fn setnc_3() {
    run_test(&Instruction { mnemonic: Mnemonic::SETNC, operand1: Some(Direct(CL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 147, 193], OperandSize::Dword)
}

#[test]
fn setnc_4() {
    run_test(&Instruction { mnemonic: Mnemonic::SETNC, operand1: Some(Indirect(EDX, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 147, 2], OperandSize::Dword)
}

#[test]
fn setnc_5() {
    run_test(&Instruction { mnemonic: Mnemonic::SETNC, operand1: Some(Direct(BL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 147, 195], OperandSize::Qword)
}

#[test]
fn setnc_6() {
    run_test(&Instruction { mnemonic: Mnemonic::SETNC, operand1: Some(IndirectScaledIndexed(RAX, RDX, Four, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 147, 4, 144], OperandSize::Qword)
}

#[test]
fn setnc_7() {
    run_test(&Instruction { mnemonic: Mnemonic::SETNC, operand1: Some(Direct(DL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 147, 194], OperandSize::Qword)
}

#[test]
fn setnc_8() {
    run_test(&Instruction { mnemonic: Mnemonic::SETNC, operand1: Some(Indirect(RBX, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 147, 3], OperandSize::Qword)
}

