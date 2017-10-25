use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn kandq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::KANDQ, operand1: Some(Direct(K7)), operand2: Some(Direct(K7)), operand3: Some(Direct(K6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 225, 196, 65, 254], OperandSize::Dword)
}

fn kandq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::KANDQ, operand1: Some(Direct(K2)), operand2: Some(Direct(K7)), operand3: Some(Direct(K4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 225, 196, 65, 212], OperandSize::Qword)
}

