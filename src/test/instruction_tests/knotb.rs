use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn knotb_1() {
    run_test(&Instruction { mnemonic: Mnemonic::KNOTB, operand1: Some(Direct(K5)), operand2: Some(Direct(K6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 68, 238], OperandSize::Dword)
}

fn knotb_2() {
    run_test(&Instruction { mnemonic: Mnemonic::KNOTB, operand1: Some(Direct(K2)), operand2: Some(Direct(K1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 68, 209], OperandSize::Qword)
}

