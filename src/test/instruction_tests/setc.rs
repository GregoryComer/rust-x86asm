use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn setc_1() {
    run_test(&Instruction { mnemonic: Mnemonic::SETC, operand1: Some(Direct(BL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 146, 195], OperandSize::Word)
}

fn setc_2() {
    run_test(&Instruction { mnemonic: Mnemonic::SETC, operand1: Some(Memory(17816, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 146, 6, 152, 69], OperandSize::Word)
}

fn setc_3() {
    run_test(&Instruction { mnemonic: Mnemonic::SETC, operand1: Some(Direct(BL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 146, 195], OperandSize::Dword)
}

fn setc_4() {
    run_test(&Instruction { mnemonic: Mnemonic::SETC, operand1: Some(Indirect(EDI, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 146, 7], OperandSize::Dword)
}

fn setc_5() {
    run_test(&Instruction { mnemonic: Mnemonic::SETC, operand1: Some(Direct(DL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 146, 194], OperandSize::Qword)
}

fn setc_6() {
    run_test(&Instruction { mnemonic: Mnemonic::SETC, operand1: Some(IndirectScaledIndexed(RCX, RCX, Eight, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 146, 4, 201], OperandSize::Qword)
}

fn setc_7() {
    run_test(&Instruction { mnemonic: Mnemonic::SETC, operand1: Some(Direct(BL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 146, 195], OperandSize::Qword)
}

fn setc_8() {
    run_test(&Instruction { mnemonic: Mnemonic::SETC, operand1: Some(Indirect(RCX, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 146, 1], OperandSize::Qword)
}

