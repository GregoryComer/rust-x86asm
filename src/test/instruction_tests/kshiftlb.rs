use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn kshiftlb_1() {
    run_test(&Instruction { mnemonic: Mnemonic::KSHIFTLB, operand1: Some(Direct(K1)), operand2: Some(Direct(K3)), operand3: Some(Literal8(109)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 121, 50, 203, 109], OperandSize::Dword)
}

fn kshiftlb_2() {
    run_test(&Instruction { mnemonic: Mnemonic::KSHIFTLB, operand1: Some(Direct(K1)), operand2: Some(Direct(K4)), operand3: Some(Literal8(63)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 121, 50, 204, 63], OperandSize::Qword)
}

