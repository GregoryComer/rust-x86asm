use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn setng_1() {
    run_test(&Instruction { mnemonic: Mnemonic::SETNG, operand1: Some(Direct(DL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 158, 194], OperandSize::Word)
}

fn setng_2() {
    run_test(&Instruction { mnemonic: Mnemonic::SETNG, operand1: Some(IndirectScaledIndexedDisplaced(BP, SI, One, 211, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 158, 130, 211, 0], OperandSize::Word)
}

fn setng_3() {
    run_test(&Instruction { mnemonic: Mnemonic::SETNG, operand1: Some(Direct(BL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 158, 195], OperandSize::Dword)
}

fn setng_4() {
    run_test(&Instruction { mnemonic: Mnemonic::SETNG, operand1: Some(IndirectScaledIndexed(EDI, EDI, Four, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 158, 4, 191], OperandSize::Dword)
}

fn setng_5() {
    run_test(&Instruction { mnemonic: Mnemonic::SETNG, operand1: Some(Direct(BL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 158, 195], OperandSize::Qword)
}

fn setng_6() {
    run_test(&Instruction { mnemonic: Mnemonic::SETNG, operand1: Some(IndirectScaledIndexedDisplaced(RAX, RBX, Two, 1638655751, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 158, 132, 88, 7, 231, 171, 97], OperandSize::Qword)
}

fn setng_7() {
    run_test(&Instruction { mnemonic: Mnemonic::SETNG, operand1: Some(Direct(DL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 158, 194], OperandSize::Qword)
}

fn setng_8() {
    run_test(&Instruction { mnemonic: Mnemonic::SETNG, operand1: Some(Indirect(RAX, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 158, 0], OperandSize::Qword)
}

