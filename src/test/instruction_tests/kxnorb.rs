use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn kxnorb_1() {
    run_test(&Instruction { mnemonic: Mnemonic::KXNORB, operand1: Some(Direct(K1)), operand2: Some(Direct(K5)), operand3: Some(Direct(K5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 213, 70, 205], OperandSize::Dword)
}

fn kxnorb_2() {
    run_test(&Instruction { mnemonic: Mnemonic::KXNORB, operand1: Some(Direct(K4)), operand2: Some(Direct(K6)), operand3: Some(Direct(K1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 205, 70, 225], OperandSize::Qword)
}

