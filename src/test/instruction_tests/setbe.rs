use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn setbe_1() {
    run_test(&Instruction { mnemonic: Mnemonic::SETBE, operand1: Some(Direct(CL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 150, 193], OperandSize::Word)
}

fn setbe_2() {
    run_test(&Instruction { mnemonic: Mnemonic::SETBE, operand1: Some(IndirectScaledIndexed(BP, DI, One, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 150, 3], OperandSize::Word)
}

fn setbe_3() {
    run_test(&Instruction { mnemonic: Mnemonic::SETBE, operand1: Some(Direct(DL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 150, 194], OperandSize::Dword)
}

fn setbe_4() {
    run_test(&Instruction { mnemonic: Mnemonic::SETBE, operand1: Some(IndirectScaledIndexedDisplaced(EDX, ECX, Four, 1013535583, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 150, 132, 138, 95, 83, 105, 60], OperandSize::Dword)
}

fn setbe_5() {
    run_test(&Instruction { mnemonic: Mnemonic::SETBE, operand1: Some(Direct(BL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 150, 195], OperandSize::Qword)
}

fn setbe_6() {
    run_test(&Instruction { mnemonic: Mnemonic::SETBE, operand1: Some(IndirectDisplaced(RBX, 216512418, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 150, 131, 162, 183, 231, 12], OperandSize::Qword)
}

fn setbe_7() {
    run_test(&Instruction { mnemonic: Mnemonic::SETBE, operand1: Some(Direct(BL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 150, 195], OperandSize::Qword)
}

fn setbe_8() {
    run_test(&Instruction { mnemonic: Mnemonic::SETBE, operand1: Some(IndirectDisplaced(RAX, 1609732884, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 150, 128, 20, 147, 242, 95], OperandSize::Qword)
}

