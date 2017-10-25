use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn ktestw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::KTESTW, operand1: Some(Direct(K4)), operand2: Some(Direct(K2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 153, 226], OperandSize::Dword)
}

fn ktestw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::KTESTW, operand1: Some(Direct(K5)), operand2: Some(Direct(K4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 153, 236], OperandSize::Qword)
}

