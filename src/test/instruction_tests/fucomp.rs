use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn fucomp_1() {
    run_test(&Instruction { mnemonic: Mnemonic::FUCOMP, operand1: Some(Direct(ST1)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[221, 233], OperandSize::Word)
}

fn fucomp_2() {
    run_test(&Instruction { mnemonic: Mnemonic::FUCOMP, operand1: Some(Direct(ST3)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[221, 235], OperandSize::Dword)
}

fn fucomp_3() {
    run_test(&Instruction { mnemonic: Mnemonic::FUCOMP, operand1: Some(Direct(ST7)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[221, 239], OperandSize::Qword)
}

