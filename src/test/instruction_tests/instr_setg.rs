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
    run_test(&Instruction { mnemonic: Mnemonic::SETG, operand1: Some(IndirectScaledIndexedDisplaced(BP, SI, One, 91, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 159, 66, 91], OperandSize::Word)
}

#[test]
fn setg_3() {
    run_test(&Instruction { mnemonic: Mnemonic::SETG, operand1: Some(Direct(BL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 159, 195], OperandSize::Dword)
}

#[test]
fn setg_4() {
    run_test(&Instruction { mnemonic: Mnemonic::SETG, operand1: Some(IndirectScaledDisplaced(EAX, Eight, 1488015844, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 159, 4, 197, 228, 81, 177, 88], OperandSize::Dword)
}

#[test]
fn setg_5() {
    run_test(&Instruction { mnemonic: Mnemonic::SETG, operand1: Some(Direct(CL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 159, 193], OperandSize::Qword)
}

#[test]
fn setg_6() {
    run_test(&Instruction { mnemonic: Mnemonic::SETG, operand1: Some(Indirect(RAX, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 159, 0], OperandSize::Qword)
}

#[test]
fn setg_7() {
    run_test(&Instruction { mnemonic: Mnemonic::SETG, operand1: Some(Direct(BL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 159, 195], OperandSize::Qword)
}

#[test]
fn setg_8() {
    run_test(&Instruction { mnemonic: Mnemonic::SETG, operand1: Some(Indirect(RCX, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 159, 1], OperandSize::Qword)
}

