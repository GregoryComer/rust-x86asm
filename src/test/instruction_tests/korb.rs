use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn korb_1() {
    run_test(&Instruction { mnemonic: Mnemonic::KORB, operand1: Some(Direct(K6)), operand2: Some(Direct(K1)), operand3: Some(Direct(K7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 245, 69, 247], OperandSize::Dword)
}

fn korb_2() {
    run_test(&Instruction { mnemonic: Mnemonic::KORB, operand1: Some(Direct(K4)), operand2: Some(Direct(K1)), operand3: Some(Direct(K6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 245, 69, 230], OperandSize::Qword)
}

