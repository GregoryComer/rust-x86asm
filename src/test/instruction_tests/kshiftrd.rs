use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn kshiftrd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::KSHIFTRD, operand1: Some(Direct(K7)), operand2: Some(Direct(K6)), operand3: Some(Literal8(51)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 121, 49, 254, 51], OperandSize::Dword)
}

fn kshiftrd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::KSHIFTRD, operand1: Some(Direct(K5)), operand2: Some(Direct(K7)), operand3: Some(Literal8(99)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 121, 49, 239, 99], OperandSize::Qword)
}

