use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn setna_1() {
    run_test(&Instruction { mnemonic: Mnemonic::SETNA, operand1: Some(Direct(DL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 150, 194], OperandSize::Word)
}

#[test]
fn setna_2() {
    run_test(&Instruction { mnemonic: Mnemonic::SETNA, operand1: Some(IndirectDisplaced(BX, 1663, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 150, 135, 127, 6], OperandSize::Word)
}

#[test]
fn setna_3() {
    run_test(&Instruction { mnemonic: Mnemonic::SETNA, operand1: Some(Direct(CL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 150, 193], OperandSize::Dword)
}

#[test]
fn setna_4() {
    run_test(&Instruction { mnemonic: Mnemonic::SETNA, operand1: Some(IndirectScaledDisplaced(ECX, Four, 1693224574, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 150, 4, 141, 126, 142, 236, 100], OperandSize::Dword)
}

#[test]
fn setna_5() {
    run_test(&Instruction { mnemonic: Mnemonic::SETNA, operand1: Some(Direct(DL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 150, 194], OperandSize::Qword)
}

#[test]
fn setna_6() {
    run_test(&Instruction { mnemonic: Mnemonic::SETNA, operand1: Some(IndirectScaledIndexed(RBX, RCX, Four, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 150, 4, 139], OperandSize::Qword)
}

#[test]
fn setna_7() {
    run_test(&Instruction { mnemonic: Mnemonic::SETNA, operand1: Some(Direct(CL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 150, 193], OperandSize::Qword)
}

#[test]
fn setna_8() {
    run_test(&Instruction { mnemonic: Mnemonic::SETNA, operand1: Some(IndirectScaledIndexed(RAX, RSI, Four, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 150, 4, 176], OperandSize::Qword)
}

