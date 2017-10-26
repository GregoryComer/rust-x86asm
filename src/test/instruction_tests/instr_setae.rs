use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn setae_1() {
    run_test(&Instruction { mnemonic: Mnemonic::SETAE, operand1: Some(Direct(DL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 147, 194], OperandSize::Word)
}

#[test]
fn setae_2() {
    run_test(&Instruction { mnemonic: Mnemonic::SETAE, operand1: Some(IndirectScaledIndexedDisplaced(BP, DI, One, 20, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 147, 67, 20], OperandSize::Word)
}

#[test]
fn setae_3() {
    run_test(&Instruction { mnemonic: Mnemonic::SETAE, operand1: Some(Direct(BL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 147, 195], OperandSize::Dword)
}

#[test]
fn setae_4() {
    run_test(&Instruction { mnemonic: Mnemonic::SETAE, operand1: Some(IndirectScaledIndexed(EDI, EDX, Four, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 147, 4, 151], OperandSize::Dword)
}

#[test]
fn setae_5() {
    run_test(&Instruction { mnemonic: Mnemonic::SETAE, operand1: Some(Direct(DL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 147, 194], OperandSize::Qword)
}

#[test]
fn setae_6() {
    run_test(&Instruction { mnemonic: Mnemonic::SETAE, operand1: Some(IndirectScaledIndexedDisplaced(RDI, RDI, Four, 731962257, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 147, 132, 191, 145, 219, 160, 43], OperandSize::Qword)
}

#[test]
fn setae_7() {
    run_test(&Instruction { mnemonic: Mnemonic::SETAE, operand1: Some(Direct(DL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 147, 194], OperandSize::Qword)
}

#[test]
fn setae_8() {
    run_test(&Instruction { mnemonic: Mnemonic::SETAE, operand1: Some(Indirect(RCX, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 147, 1], OperandSize::Qword)
}

