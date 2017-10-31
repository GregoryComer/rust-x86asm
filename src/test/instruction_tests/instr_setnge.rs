use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn setnge_1() {
    run_test(&Instruction { mnemonic: Mnemonic::SETNGE, operand1: Some(Direct(BL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 156, 195], OperandSize::Word)
}

#[test]
fn setnge_2() {
    run_test(&Instruction { mnemonic: Mnemonic::SETNGE, operand1: Some(IndirectDisplaced(SI, 3671, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 156, 132, 87, 14], OperandSize::Word)
}

#[test]
fn setnge_3() {
    run_test(&Instruction { mnemonic: Mnemonic::SETNGE, operand1: Some(Direct(DL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 156, 194], OperandSize::Dword)
}

#[test]
fn setnge_4() {
    run_test(&Instruction { mnemonic: Mnemonic::SETNGE, operand1: Some(IndirectScaledDisplaced(ECX, Four, 2106562317, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 156, 4, 141, 13, 151, 143, 125], OperandSize::Dword)
}

#[test]
fn setnge_5() {
    run_test(&Instruction { mnemonic: Mnemonic::SETNGE, operand1: Some(Direct(DL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 156, 194], OperandSize::Qword)
}

#[test]
fn setnge_6() {
    run_test(&Instruction { mnemonic: Mnemonic::SETNGE, operand1: Some(Indirect(RAX, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 156, 0], OperandSize::Qword)
}

#[test]
fn setnge_7() {
    run_test(&Instruction { mnemonic: Mnemonic::SETNGE, operand1: Some(Direct(DL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 156, 194], OperandSize::Qword)
}

#[test]
fn setnge_8() {
    run_test(&Instruction { mnemonic: Mnemonic::SETNGE, operand1: Some(IndirectScaledIndexedDisplaced(RDI, RSI, Two, 183504010, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 156, 132, 119, 138, 12, 240, 10], OperandSize::Qword)
}

