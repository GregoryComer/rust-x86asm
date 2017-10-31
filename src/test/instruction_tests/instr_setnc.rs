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
    run_test(&Instruction { mnemonic: Mnemonic::SETNC, operand1: Some(Indirect(DI, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 147, 5], OperandSize::Word)
}

#[test]
fn setnc_3() {
    run_test(&Instruction { mnemonic: Mnemonic::SETNC, operand1: Some(Direct(CL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 147, 193], OperandSize::Dword)
}

#[test]
fn setnc_4() {
    run_test(&Instruction { mnemonic: Mnemonic::SETNC, operand1: Some(IndirectScaledIndexed(EDI, EDX, Eight, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 147, 4, 215], OperandSize::Dword)
}

#[test]
fn setnc_5() {
    run_test(&Instruction { mnemonic: Mnemonic::SETNC, operand1: Some(Direct(DL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 147, 194], OperandSize::Qword)
}

#[test]
fn setnc_6() {
    run_test(&Instruction { mnemonic: Mnemonic::SETNC, operand1: Some(IndirectScaledDisplaced(RDI, Four, 2026211947, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 147, 4, 189, 107, 138, 197, 120], OperandSize::Qword)
}

#[test]
fn setnc_7() {
    run_test(&Instruction { mnemonic: Mnemonic::SETNC, operand1: Some(Direct(BL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 147, 195], OperandSize::Qword)
}

#[test]
fn setnc_8() {
    run_test(&Instruction { mnemonic: Mnemonic::SETNC, operand1: Some(IndirectDisplaced(RDX, 665786378, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 147, 130, 10, 24, 175, 39], OperandSize::Qword)
}

