use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn kshiftld_1() {
    run_test(&Instruction { mnemonic: Mnemonic::KSHIFTLD, operand1: Some(Direct(K7)), operand2: Some(Direct(K4)), operand3: Some(Literal8(92)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 121, 51, 252, 92], OperandSize::Dword)
}

fn kshiftld_2() {
    run_test(&Instruction { mnemonic: Mnemonic::KSHIFTLD, operand1: Some(Direct(K4)), operand2: Some(Direct(K2)), operand3: Some(Literal8(96)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 121, 51, 226, 96], OperandSize::Qword)
}

