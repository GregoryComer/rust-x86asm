use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn kaddd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::KADDD, operand1: Some(Direct(K2)), operand2: Some(Direct(K3)), operand3: Some(Direct(K5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 225, 229, 74, 213], OperandSize::Dword)
}

fn kaddd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::KADDD, operand1: Some(Direct(K7)), operand2: Some(Direct(K5)), operand3: Some(Direct(K5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 225, 213, 74, 253], OperandSize::Qword)
}

