use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn kxnord_1() {
    run_test(&Instruction { mnemonic: Mnemonic::KXNORD, operand1: Some(Direct(K2)), operand2: Some(Direct(K5)), operand3: Some(Direct(K6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 225, 213, 70, 214], OperandSize::Dword)
}

fn kxnord_2() {
    run_test(&Instruction { mnemonic: Mnemonic::KXNORD, operand1: Some(Direct(K4)), operand2: Some(Direct(K5)), operand3: Some(Direct(K5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 225, 213, 70, 229], OperandSize::Qword)
}

