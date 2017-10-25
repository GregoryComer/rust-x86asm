use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn kshiftrb_1() {
    run_test(&Instruction { mnemonic: Mnemonic::KSHIFTRB, operand1: Some(Direct(K5)), operand2: Some(Direct(K4)), operand3: Some(Literal8(41)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 121, 48, 236, 41], OperandSize::Dword)
}

fn kshiftrb_2() {
    run_test(&Instruction { mnemonic: Mnemonic::KSHIFTRB, operand1: Some(Direct(K4)), operand2: Some(Direct(K3)), operand3: Some(Literal8(102)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 121, 48, 227, 102], OperandSize::Qword)
}

