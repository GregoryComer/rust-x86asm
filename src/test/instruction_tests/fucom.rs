use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn fucom_1() {
    run_test(&Instruction { mnemonic: Mnemonic::FUCOM, operand1: Some(Direct(ST7)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[221, 231], OperandSize::Word)
}

fn fucom_2() {
    run_test(&Instruction { mnemonic: Mnemonic::FUCOM, operand1: Some(Direct(ST2)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[221, 226], OperandSize::Dword)
}

fn fucom_3() {
    run_test(&Instruction { mnemonic: Mnemonic::FUCOM, operand1: Some(Direct(ST3)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[221, 227], OperandSize::Qword)
}

