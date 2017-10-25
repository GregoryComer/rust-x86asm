use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn kunpckwd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::KUNPCKWD, operand1: Some(Direct(K2)), operand2: Some(Direct(K5)), operand3: Some(Direct(K5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 212, 75, 213], OperandSize::Dword)
}

fn kunpckwd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::KUNPCKWD, operand1: Some(Direct(K3)), operand2: Some(Direct(K1)), operand3: Some(Direct(K6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 244, 75, 222], OperandSize::Qword)
}

