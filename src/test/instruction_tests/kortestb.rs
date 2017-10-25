use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn kortestb_1() {
    run_test(&Instruction { mnemonic: Mnemonic::KORTESTB, operand1: Some(Direct(K4)), operand2: Some(Direct(K2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 152, 226], OperandSize::Dword)
}

fn kortestb_2() {
    run_test(&Instruction { mnemonic: Mnemonic::KORTESTB, operand1: Some(Direct(K7)), operand2: Some(Direct(K7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 152, 255], OperandSize::Qword)
}

