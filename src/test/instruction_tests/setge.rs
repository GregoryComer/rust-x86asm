use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn setge_1() {
    run_test(&Instruction { mnemonic: Mnemonic::SETGE, operand1: Some(Direct(DL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 157, 194], OperandSize::Word)
}

fn setge_2() {
    run_test(&Instruction { mnemonic: Mnemonic::SETGE, operand1: Some(IndirectDisplaced(BX, 13345, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 157, 135, 33, 52], OperandSize::Word)
}

fn setge_3() {
    run_test(&Instruction { mnemonic: Mnemonic::SETGE, operand1: Some(Direct(CL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 157, 193], OperandSize::Dword)
}

fn setge_4() {
    run_test(&Instruction { mnemonic: Mnemonic::SETGE, operand1: Some(IndirectScaledIndexed(EAX, ECX, Two, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 157, 4, 72], OperandSize::Dword)
}

fn setge_5() {
    run_test(&Instruction { mnemonic: Mnemonic::SETGE, operand1: Some(Direct(CL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 157, 193], OperandSize::Qword)
}

fn setge_6() {
    run_test(&Instruction { mnemonic: Mnemonic::SETGE, operand1: Some(IndirectScaledDisplaced(RAX, Eight, 1552557253, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 157, 4, 197, 197, 36, 138, 92], OperandSize::Qword)
}

fn setge_7() {
    run_test(&Instruction { mnemonic: Mnemonic::SETGE, operand1: Some(Direct(DL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 157, 194], OperandSize::Qword)
}

fn setge_8() {
    run_test(&Instruction { mnemonic: Mnemonic::SETGE, operand1: Some(IndirectScaledDisplaced(RSI, Four, 1930086803, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 157, 4, 181, 147, 201, 10, 115], OperandSize::Qword)
}

