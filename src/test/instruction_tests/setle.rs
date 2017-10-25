use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn setle_1() {
    run_test(&Instruction { mnemonic: Mnemonic::SETLE, operand1: Some(Direct(BL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 158, 195], OperandSize::Word)
}

fn setle_2() {
    run_test(&Instruction { mnemonic: Mnemonic::SETLE, operand1: Some(IndirectScaledIndexedDisplaced(BX, SI, One, 70, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 158, 64, 70], OperandSize::Word)
}

fn setle_3() {
    run_test(&Instruction { mnemonic: Mnemonic::SETLE, operand1: Some(Direct(BL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 158, 195], OperandSize::Dword)
}

fn setle_4() {
    run_test(&Instruction { mnemonic: Mnemonic::SETLE, operand1: Some(IndirectDisplaced(EDX, 2105724766, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 158, 130, 94, 207, 130, 125], OperandSize::Dword)
}

fn setle_5() {
    run_test(&Instruction { mnemonic: Mnemonic::SETLE, operand1: Some(Direct(CL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 158, 193], OperandSize::Qword)
}

fn setle_6() {
    run_test(&Instruction { mnemonic: Mnemonic::SETLE, operand1: Some(IndirectScaledIndexedDisplaced(RCX, RDI, Eight, 2079955112, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 158, 132, 249, 168, 152, 249, 123], OperandSize::Qword)
}

fn setle_7() {
    run_test(&Instruction { mnemonic: Mnemonic::SETLE, operand1: Some(Direct(DL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 158, 194], OperandSize::Qword)
}

fn setle_8() {
    run_test(&Instruction { mnemonic: Mnemonic::SETLE, operand1: Some(IndirectScaledIndexed(RAX, RAX, Four, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 158, 4, 128], OperandSize::Qword)
}

