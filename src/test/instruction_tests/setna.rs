use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn setna_1() {
    run_test(&Instruction { mnemonic: Mnemonic::SETNA, operand1: Some(Direct(BL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 150, 195], OperandSize::Word)
}

fn setna_2() {
    run_test(&Instruction { mnemonic: Mnemonic::SETNA, operand1: Some(IndirectScaledIndexedDisplaced(BX, DI, One, 8809, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 150, 129, 105, 34], OperandSize::Word)
}

fn setna_3() {
    run_test(&Instruction { mnemonic: Mnemonic::SETNA, operand1: Some(Direct(BL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 150, 195], OperandSize::Dword)
}

fn setna_4() {
    run_test(&Instruction { mnemonic: Mnemonic::SETNA, operand1: Some(IndirectScaledDisplaced(EDX, Two, 13155049, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 150, 4, 85, 233, 186, 200, 0], OperandSize::Dword)
}

fn setna_5() {
    run_test(&Instruction { mnemonic: Mnemonic::SETNA, operand1: Some(Direct(CL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 150, 193], OperandSize::Qword)
}

fn setna_6() {
    run_test(&Instruction { mnemonic: Mnemonic::SETNA, operand1: Some(Indirect(RAX, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 150, 0], OperandSize::Qword)
}

fn setna_7() {
    run_test(&Instruction { mnemonic: Mnemonic::SETNA, operand1: Some(Direct(DL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 150, 194], OperandSize::Qword)
}

fn setna_8() {
    run_test(&Instruction { mnemonic: Mnemonic::SETNA, operand1: Some(IndirectDisplaced(RCX, 1784775150, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 150, 129, 238, 129, 97, 106], OperandSize::Qword)
}

