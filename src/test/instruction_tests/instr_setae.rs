use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn setae_1() {
    run_test(&Instruction { mnemonic: Mnemonic::SETAE, operand1: Some(Direct(CL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 147, 193], OperandSize::Word)
}

#[test]
fn setae_2() {
    run_test(&Instruction { mnemonic: Mnemonic::SETAE, operand1: Some(IndirectDisplaced(BP, 136, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 147, 134, 136, 0], OperandSize::Word)
}

#[test]
fn setae_3() {
    run_test(&Instruction { mnemonic: Mnemonic::SETAE, operand1: Some(Direct(DL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 147, 194], OperandSize::Dword)
}

#[test]
fn setae_4() {
    run_test(&Instruction { mnemonic: Mnemonic::SETAE, operand1: Some(IndirectScaledIndexedDisplaced(EDI, ESI, Four, 1532067978, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 147, 132, 183, 138, 128, 81, 91], OperandSize::Dword)
}

#[test]
fn setae_5() {
    run_test(&Instruction { mnemonic: Mnemonic::SETAE, operand1: Some(Direct(BL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 147, 195], OperandSize::Qword)
}

#[test]
fn setae_6() {
    run_test(&Instruction { mnemonic: Mnemonic::SETAE, operand1: Some(IndirectScaledDisplaced(RSI, Eight, 561382146, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 147, 4, 245, 2, 3, 118, 33], OperandSize::Qword)
}

#[test]
fn setae_7() {
    run_test(&Instruction { mnemonic: Mnemonic::SETAE, operand1: Some(Direct(DL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 147, 194], OperandSize::Qword)
}

#[test]
fn setae_8() {
    run_test(&Instruction { mnemonic: Mnemonic::SETAE, operand1: Some(IndirectScaledDisplaced(RCX, Four, 39623737, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 147, 4, 141, 57, 156, 92, 2], OperandSize::Qword)
}

