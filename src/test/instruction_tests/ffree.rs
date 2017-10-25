use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn ffree_1() {
    run_test(&Instruction { mnemonic: Mnemonic::FFREE, operand1: Some(Direct(ST1)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[221, 193], OperandSize::Word)
}

fn ffree_2() {
    run_test(&Instruction { mnemonic: Mnemonic::FFREE, operand1: Some(Direct(ST6)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[221, 198], OperandSize::Dword)
}

fn ffree_3() {
    run_test(&Instruction { mnemonic: Mnemonic::FFREE, operand1: Some(Direct(ST4)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[221, 196], OperandSize::Qword)
}

