use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn kortestw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::KORTESTW, operand1: Some(Direct(K1)), operand2: Some(Direct(K3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 152, 203], OperandSize::Dword)
}

fn kortestw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::KORTESTW, operand1: Some(Direct(K2)), operand2: Some(Direct(K5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 152, 213], OperandSize::Qword)
}

