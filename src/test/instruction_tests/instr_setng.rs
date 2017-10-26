use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn setng_1() {
    run_test(&Instruction { mnemonic: Mnemonic::SETNG, operand1: Some(Direct(CL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 158, 193], OperandSize::Word)
}

#[test]
fn setng_2() {
    run_test(&Instruction { mnemonic: Mnemonic::SETNG, operand1: Some(IndirectScaledIndexed(BP, DI, One, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 158, 3], OperandSize::Word)
}

#[test]
fn setng_3() {
    run_test(&Instruction { mnemonic: Mnemonic::SETNG, operand1: Some(Direct(BL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 158, 195], OperandSize::Dword)
}

#[test]
fn setng_4() {
    run_test(&Instruction { mnemonic: Mnemonic::SETNG, operand1: Some(IndirectScaledIndexed(EBX, EDI, Four, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 158, 4, 187], OperandSize::Dword)
}

#[test]
fn setng_5() {
    run_test(&Instruction { mnemonic: Mnemonic::SETNG, operand1: Some(Direct(DL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 158, 194], OperandSize::Qword)
}

#[test]
fn setng_6() {
    run_test(&Instruction { mnemonic: Mnemonic::SETNG, operand1: Some(IndirectDisplaced(RCX, 258581215, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 158, 129, 223, 162, 105, 15], OperandSize::Qword)
}

#[test]
fn setng_7() {
    run_test(&Instruction { mnemonic: Mnemonic::SETNG, operand1: Some(Direct(CL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 158, 193], OperandSize::Qword)
}

#[test]
fn setng_8() {
    run_test(&Instruction { mnemonic: Mnemonic::SETNG, operand1: Some(IndirectScaledIndexed(RDI, RBX, Four, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 158, 4, 159], OperandSize::Qword)
}

