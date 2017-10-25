use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn kaddw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::KADDW, operand1: Some(Direct(K1)), operand2: Some(Direct(K7)), operand3: Some(Direct(K6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 196, 74, 206], OperandSize::Dword)
}

fn kaddw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::KADDW, operand1: Some(Direct(K4)), operand2: Some(Direct(K2)), operand3: Some(Direct(K1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 236, 74, 225], OperandSize::Qword)
}

