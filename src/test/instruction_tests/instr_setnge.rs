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
    run_test(&Instruction { mnemonic: Mnemonic::SETNGE, operand1: Some(IndirectScaledIndexedDisplaced(BP, SI, One, 164, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 156, 130, 164, 0], OperandSize::Word)
}

#[test]
fn setnge_3() {
    run_test(&Instruction { mnemonic: Mnemonic::SETNGE, operand1: Some(Direct(DL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 156, 194], OperandSize::Dword)
}

#[test]
fn setnge_4() {
    run_test(&Instruction { mnemonic: Mnemonic::SETNGE, operand1: Some(IndirectDisplaced(EDI, 1207762010, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 156, 135, 90, 252, 252, 71], OperandSize::Dword)
}

#[test]
fn setnge_5() {
    run_test(&Instruction { mnemonic: Mnemonic::SETNGE, operand1: Some(Direct(BL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 156, 195], OperandSize::Qword)
}

#[test]
fn setnge_6() {
    run_test(&Instruction { mnemonic: Mnemonic::SETNGE, operand1: Some(Indirect(RCX, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 156, 1], OperandSize::Qword)
}

#[test]
fn setnge_7() {
    run_test(&Instruction { mnemonic: Mnemonic::SETNGE, operand1: Some(Direct(BL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 156, 195], OperandSize::Qword)
}

#[test]
fn setnge_8() {
    run_test(&Instruction { mnemonic: Mnemonic::SETNGE, operand1: Some(IndirectDisplaced(RSI, 2097703705, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 156, 134, 25, 107, 8, 125], OperandSize::Qword)
}

