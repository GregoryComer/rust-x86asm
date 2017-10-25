use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn kshiftlq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::KSHIFTLQ, operand1: Some(Direct(K5)), operand2: Some(Direct(K3)), operand3: Some(Literal8(78)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 249, 51, 235, 78], OperandSize::Dword)
}

fn kshiftlq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::KSHIFTLQ, operand1: Some(Direct(K3)), operand2: Some(Direct(K4)), operand3: Some(Literal8(80)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 249, 51, 220, 80], OperandSize::Qword)
}

