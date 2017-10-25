use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn kandnq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::KANDNQ, operand1: Some(Direct(K7)), operand2: Some(Direct(K4)), operand3: Some(Direct(K4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 225, 220, 66, 252], OperandSize::Dword)
}

fn kandnq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::KANDNQ, operand1: Some(Direct(K1)), operand2: Some(Direct(K5)), operand3: Some(Direct(K4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 225, 212, 66, 204], OperandSize::Qword)
}

