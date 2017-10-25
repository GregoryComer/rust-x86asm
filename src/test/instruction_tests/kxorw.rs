use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn kxorw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::KXORW, operand1: Some(Direct(K4)), operand2: Some(Direct(K2)), operand3: Some(Direct(K6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 236, 71, 230], OperandSize::Dword)
}

fn kxorw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::KXORW, operand1: Some(Direct(K5)), operand2: Some(Direct(K7)), operand3: Some(Direct(K7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 196, 71, 239], OperandSize::Qword)
}

