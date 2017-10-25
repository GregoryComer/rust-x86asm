use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn ktestb_1() {
    run_test(&Instruction { mnemonic: Mnemonic::KTESTB, operand1: Some(Direct(K6)), operand2: Some(Direct(K3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 153, 243], OperandSize::Dword)
}

fn ktestb_2() {
    run_test(&Instruction { mnemonic: Mnemonic::KTESTB, operand1: Some(Direct(K7)), operand2: Some(Direct(K2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 153, 250], OperandSize::Qword)
}

