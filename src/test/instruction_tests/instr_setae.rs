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
    run_test(&Instruction { mnemonic: Mnemonic::SETAE, operand1: Some(IndirectScaledIndexedDisplaced(BP, DI, One, 89, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 147, 67, 89], OperandSize::Word)
}

#[test]
fn setae_3() {
    run_test(&Instruction { mnemonic: Mnemonic::SETAE, operand1: Some(Direct(CL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 147, 193], OperandSize::Dword)
}

#[test]
fn setae_4() {
    run_test(&Instruction { mnemonic: Mnemonic::SETAE, operand1: Some(IndirectScaledDisplaced(ESI, Two, 1703376625, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 147, 4, 117, 241, 118, 135, 101], OperandSize::Dword)
}

#[test]
fn setae_5() {
    run_test(&Instruction { mnemonic: Mnemonic::SETAE, operand1: Some(Direct(DL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 147, 194], OperandSize::Qword)
}

#[test]
fn setae_6() {
    run_test(&Instruction { mnemonic: Mnemonic::SETAE, operand1: Some(IndirectScaledIndexed(RCX, RCX, Four, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 147, 4, 137], OperandSize::Qword)
}

#[test]
fn setae_7() {
    run_test(&Instruction { mnemonic: Mnemonic::SETAE, operand1: Some(Direct(CL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 147, 193], OperandSize::Qword)
}

#[test]
fn setae_8() {
    run_test(&Instruction { mnemonic: Mnemonic::SETAE, operand1: Some(IndirectScaledIndexedDisplaced(RDI, RDI, Four, 371671579, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 147, 132, 191, 27, 66, 39, 22], OperandSize::Qword)
}

