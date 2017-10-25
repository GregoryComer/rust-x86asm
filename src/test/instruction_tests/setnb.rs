use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn setnb_1() {
    run_test(&Instruction { mnemonic: Mnemonic::SETNB, operand1: Some(Direct(DL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 147, 194], OperandSize::Word)
}

fn setnb_2() {
    run_test(&Instruction { mnemonic: Mnemonic::SETNB, operand1: Some(IndirectDisplaced(SI, 24463, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 147, 132, 143, 95], OperandSize::Word)
}

fn setnb_3() {
    run_test(&Instruction { mnemonic: Mnemonic::SETNB, operand1: Some(Direct(CL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 147, 193], OperandSize::Dword)
}

fn setnb_4() {
    run_test(&Instruction { mnemonic: Mnemonic::SETNB, operand1: Some(IndirectScaledIndexed(EDX, ESI, Eight, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 147, 4, 242], OperandSize::Dword)
}

fn setnb_5() {
    run_test(&Instruction { mnemonic: Mnemonic::SETNB, operand1: Some(Direct(DL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 147, 194], OperandSize::Qword)
}

fn setnb_6() {
    run_test(&Instruction { mnemonic: Mnemonic::SETNB, operand1: Some(IndirectScaledDisplaced(RBX, Eight, 42100087, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 147, 4, 221, 119, 101, 130, 2], OperandSize::Qword)
}

fn setnb_7() {
    run_test(&Instruction { mnemonic: Mnemonic::SETNB, operand1: Some(Direct(CL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 147, 193], OperandSize::Qword)
}

fn setnb_8() {
    run_test(&Instruction { mnemonic: Mnemonic::SETNB, operand1: Some(IndirectScaledIndexedDisplaced(RBX, RAX, Four, 1235756855, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 147, 132, 131, 55, 39, 168, 73], OperandSize::Qword)
}

