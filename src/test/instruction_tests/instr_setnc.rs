use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn setnc_1() {
    run_test(&Instruction { mnemonic: Mnemonic::SETNC, operand1: Some(Direct(CL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 147, 193], OperandSize::Word)
}

#[test]
fn setnc_2() {
    run_test(&Instruction { mnemonic: Mnemonic::SETNC, operand1: Some(IndirectScaledIndexed(BX, SI, One, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 147, 0], OperandSize::Word)
}

#[test]
fn setnc_3() {
    run_test(&Instruction { mnemonic: Mnemonic::SETNC, operand1: Some(Direct(BL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 147, 195], OperandSize::Dword)
}

#[test]
fn setnc_4() {
    run_test(&Instruction { mnemonic: Mnemonic::SETNC, operand1: Some(IndirectScaledIndexedDisplaced(EDX, EDX, Two, 991605394, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 147, 132, 82, 146, 178, 26, 59], OperandSize::Dword)
}

#[test]
fn setnc_5() {
    run_test(&Instruction { mnemonic: Mnemonic::SETNC, operand1: Some(Direct(BL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 147, 195], OperandSize::Qword)
}

#[test]
fn setnc_6() {
    run_test(&Instruction { mnemonic: Mnemonic::SETNC, operand1: Some(IndirectScaledIndexedDisplaced(RDI, RCX, Two, 211018918, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 147, 132, 79, 166, 228, 147, 12], OperandSize::Qword)
}

#[test]
fn setnc_7() {
    run_test(&Instruction { mnemonic: Mnemonic::SETNC, operand1: Some(Direct(BL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 147, 195], OperandSize::Qword)
}

#[test]
fn setnc_8() {
    run_test(&Instruction { mnemonic: Mnemonic::SETNC, operand1: Some(IndirectDisplaced(RSI, 646626105, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 147, 134, 57, 187, 138, 38], OperandSize::Qword)
}

