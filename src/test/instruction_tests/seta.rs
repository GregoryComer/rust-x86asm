use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn seta_1() {
    run_test(&Instruction { mnemonic: Mnemonic::SETA, operand1: Some(Direct(BL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 151, 195], OperandSize::Word)
}

fn seta_2() {
    run_test(&Instruction { mnemonic: Mnemonic::SETA, operand1: Some(IndirectDisplaced(SI, 29056, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 151, 132, 128, 113], OperandSize::Word)
}

fn seta_3() {
    run_test(&Instruction { mnemonic: Mnemonic::SETA, operand1: Some(Direct(DL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 151, 194], OperandSize::Dword)
}

fn seta_4() {
    run_test(&Instruction { mnemonic: Mnemonic::SETA, operand1: Some(IndirectScaledIndexedDisplaced(EAX, ESI, Two, 1343171086, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 151, 132, 112, 14, 42, 15, 80], OperandSize::Dword)
}

fn seta_5() {
    run_test(&Instruction { mnemonic: Mnemonic::SETA, operand1: Some(Direct(BL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 151, 195], OperandSize::Qword)
}

fn seta_6() {
    run_test(&Instruction { mnemonic: Mnemonic::SETA, operand1: Some(IndirectScaledDisplaced(RBX, Eight, 1814961543, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 151, 4, 221, 135, 29, 46, 108], OperandSize::Qword)
}

fn seta_7() {
    run_test(&Instruction { mnemonic: Mnemonic::SETA, operand1: Some(Direct(CL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 151, 193], OperandSize::Qword)
}

fn seta_8() {
    run_test(&Instruction { mnemonic: Mnemonic::SETA, operand1: Some(IndirectScaledDisplaced(RDX, Eight, 1025826675, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 151, 4, 213, 115, 223, 36, 61], OperandSize::Qword)
}

