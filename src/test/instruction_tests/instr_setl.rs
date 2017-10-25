use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn setl_1() {
    run_test(&Instruction { mnemonic: Mnemonic::SETL, operand1: Some(Direct(CL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 156, 193], OperandSize::Word)
}

#[test]
fn setl_2() {
    run_test(&Instruction { mnemonic: Mnemonic::SETL, operand1: Some(IndirectDisplaced(SI, 30739, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 156, 132, 19, 120], OperandSize::Word)
}

#[test]
fn setl_3() {
    run_test(&Instruction { mnemonic: Mnemonic::SETL, operand1: Some(Direct(BL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 156, 195], OperandSize::Dword)
}

#[test]
fn setl_4() {
    run_test(&Instruction { mnemonic: Mnemonic::SETL, operand1: Some(IndirectScaledDisplaced(EDX, Two, 689068600, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 156, 4, 85, 56, 90, 18, 41], OperandSize::Dword)
}

#[test]
fn setl_5() {
    run_test(&Instruction { mnemonic: Mnemonic::SETL, operand1: Some(Direct(DL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 156, 194], OperandSize::Qword)
}

#[test]
fn setl_6() {
    run_test(&Instruction { mnemonic: Mnemonic::SETL, operand1: Some(IndirectDisplaced(RBX, 765293548, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 156, 131, 236, 115, 157, 45], OperandSize::Qword)
}

#[test]
fn setl_7() {
    run_test(&Instruction { mnemonic: Mnemonic::SETL, operand1: Some(Direct(CL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 156, 193], OperandSize::Qword)
}

#[test]
fn setl_8() {
    run_test(&Instruction { mnemonic: Mnemonic::SETL, operand1: Some(IndirectScaledIndexed(RDI, RAX, Two, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 156, 4, 71], OperandSize::Qword)
}

