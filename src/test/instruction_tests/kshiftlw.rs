use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn kshiftlw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::KSHIFTLW, operand1: Some(Direct(K1)), operand2: Some(Direct(K4)), operand3: Some(Literal8(91)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 249, 50, 204, 91], OperandSize::Dword)
}

fn kshiftlw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::KSHIFTLW, operand1: Some(Direct(K1)), operand2: Some(Direct(K2)), operand3: Some(Literal8(8)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 249, 50, 202, 8], OperandSize::Qword)
}

