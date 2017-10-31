use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn setg_1() {
    run_test(&Instruction { mnemonic: Mnemonic::SETG, operand1: Some(Direct(CL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 159, 193], OperandSize::Word)
}

#[test]
fn setg_2() {
    run_test(&Instruction { mnemonic: Mnemonic::SETG, operand1: Some(Indirect(SI, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 159, 4], OperandSize::Word)
}

#[test]
fn setg_3() {
    run_test(&Instruction { mnemonic: Mnemonic::SETG, operand1: Some(Direct(DL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 159, 194], OperandSize::Dword)
}

#[test]
fn setg_4() {
    run_test(&Instruction { mnemonic: Mnemonic::SETG, operand1: Some(IndirectDisplaced(ESI, 1284294884, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 159, 134, 228, 200, 140, 76], OperandSize::Dword)
}

#[test]
fn setg_5() {
    run_test(&Instruction { mnemonic: Mnemonic::SETG, operand1: Some(Direct(CL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 159, 193], OperandSize::Qword)
}

#[test]
fn setg_6() {
    run_test(&Instruction { mnemonic: Mnemonic::SETG, operand1: Some(IndirectScaledDisplaced(RDX, Four, 2113111357, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 159, 4, 149, 61, 133, 243, 125], OperandSize::Qword)
}

#[test]
fn setg_7() {
    run_test(&Instruction { mnemonic: Mnemonic::SETG, operand1: Some(Direct(CL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 159, 193], OperandSize::Qword)
}

#[test]
fn setg_8() {
    run_test(&Instruction { mnemonic: Mnemonic::SETG, operand1: Some(IndirectScaledDisplaced(RAX, Four, 747738490, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 159, 4, 133, 122, 149, 145, 44], OperandSize::Qword)
}

