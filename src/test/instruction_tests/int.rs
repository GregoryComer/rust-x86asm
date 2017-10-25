use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn int_1() {
    run_test(&Instruction { mnemonic: Mnemonic::INT, operand1: Some(Literal8(3)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[204], OperandSize::Word)
}

fn int_2() {
    run_test(&Instruction { mnemonic: Mnemonic::INT, operand1: Some(Literal8(3)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[204], OperandSize::Dword)
}

fn int_3() {
    run_test(&Instruction { mnemonic: Mnemonic::INT, operand1: Some(Literal8(3)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[204], OperandSize::Qword)
}

