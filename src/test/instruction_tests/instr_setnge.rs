use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn setnge_1() {
    run_test(&Instruction { mnemonic: Mnemonic::SETNGE, operand1: Some(Direct(CL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 156, 193], OperandSize::Word)
}

#[test]
fn setnge_2() {
    run_test(&Instruction { mnemonic: Mnemonic::SETNGE, operand1: Some(IndirectScaledIndexedDisplaced(BP, DI, One, 99, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 156, 67, 99], OperandSize::Word)
}

#[test]
fn setnge_3() {
    run_test(&Instruction { mnemonic: Mnemonic::SETNGE, operand1: Some(Direct(CL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 156, 193], OperandSize::Dword)
}

#[test]
fn setnge_4() {
    run_test(&Instruction { mnemonic: Mnemonic::SETNGE, operand1: Some(IndirectScaledIndexed(EDX, EDX, Four, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 156, 4, 146], OperandSize::Dword)
}

#[test]
fn setnge_5() {
    run_test(&Instruction { mnemonic: Mnemonic::SETNGE, operand1: Some(Direct(DL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 156, 194], OperandSize::Qword)
}

#[test]
fn setnge_6() {
    run_test(&Instruction { mnemonic: Mnemonic::SETNGE, operand1: Some(IndirectScaledIndexed(RAX, RBX, Two, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 156, 4, 88], OperandSize::Qword)
}

#[test]
fn setnge_7() {
    run_test(&Instruction { mnemonic: Mnemonic::SETNGE, operand1: Some(Direct(BL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 156, 195], OperandSize::Qword)
}

#[test]
fn setnge_8() {
    run_test(&Instruction { mnemonic: Mnemonic::SETNGE, operand1: Some(IndirectScaledIndexedDisplaced(RDX, RDX, Four, 1861078919, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 156, 132, 146, 135, 207, 237, 110], OperandSize::Qword)
}

