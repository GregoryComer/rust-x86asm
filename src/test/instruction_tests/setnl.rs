use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn setnl_1() {
    run_test(&Instruction { mnemonic: Mnemonic::SETNL, operand1: Some(Direct(BL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 157, 195], OperandSize::Word)
}

fn setnl_2() {
    run_test(&Instruction { mnemonic: Mnemonic::SETNL, operand1: Some(IndirectScaledIndexed(BP, DI, One, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 157, 3], OperandSize::Word)
}

fn setnl_3() {
    run_test(&Instruction { mnemonic: Mnemonic::SETNL, operand1: Some(Direct(CL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 157, 193], OperandSize::Dword)
}

fn setnl_4() {
    run_test(&Instruction { mnemonic: Mnemonic::SETNL, operand1: Some(IndirectScaledIndexed(EBX, EAX, Two, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 157, 4, 67], OperandSize::Dword)
}

fn setnl_5() {
    run_test(&Instruction { mnemonic: Mnemonic::SETNL, operand1: Some(Direct(DL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 157, 194], OperandSize::Qword)
}

fn setnl_6() {
    run_test(&Instruction { mnemonic: Mnemonic::SETNL, operand1: Some(IndirectDisplaced(RDX, 1089065937, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 157, 130, 209, 211, 233, 64], OperandSize::Qword)
}

fn setnl_7() {
    run_test(&Instruction { mnemonic: Mnemonic::SETNL, operand1: Some(Direct(DL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 157, 194], OperandSize::Qword)
}

fn setnl_8() {
    run_test(&Instruction { mnemonic: Mnemonic::SETNL, operand1: Some(IndirectDisplaced(RSI, 1840525847, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 157, 134, 23, 50, 180, 109], OperandSize::Qword)
}

