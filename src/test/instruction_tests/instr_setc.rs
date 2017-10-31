use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn setc_1() {
    run_test(&Instruction { mnemonic: Mnemonic::SETC, operand1: Some(Direct(DL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 146, 194], OperandSize::Word)
}

#[test]
fn setc_2() {
    run_test(&Instruction { mnemonic: Mnemonic::SETC, operand1: Some(IndirectDisplaced(BX, 4395, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 146, 135, 43, 17], OperandSize::Word)
}

#[test]
fn setc_3() {
    run_test(&Instruction { mnemonic: Mnemonic::SETC, operand1: Some(Direct(DL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 146, 194], OperandSize::Dword)
}

#[test]
fn setc_4() {
    run_test(&Instruction { mnemonic: Mnemonic::SETC, operand1: Some(IndirectScaledDisplaced(ESI, Four, 884053162, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 146, 4, 181, 170, 148, 177, 52], OperandSize::Dword)
}

#[test]
fn setc_5() {
    run_test(&Instruction { mnemonic: Mnemonic::SETC, operand1: Some(Direct(CL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 146, 193], OperandSize::Qword)
}

#[test]
fn setc_6() {
    run_test(&Instruction { mnemonic: Mnemonic::SETC, operand1: Some(IndirectDisplaced(RBX, 215182080, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 146, 131, 0, 107, 211, 12], OperandSize::Qword)
}

#[test]
fn setc_7() {
    run_test(&Instruction { mnemonic: Mnemonic::SETC, operand1: Some(Direct(CL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 146, 193], OperandSize::Qword)
}

#[test]
fn setc_8() {
    run_test(&Instruction { mnemonic: Mnemonic::SETC, operand1: Some(Indirect(RAX, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 146, 0], OperandSize::Qword)
}

