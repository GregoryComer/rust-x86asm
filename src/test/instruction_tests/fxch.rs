use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn fxch_1() {
    run_test(&Instruction { mnemonic: Mnemonic::FXCH, operand1: Some(Direct(ST1)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[217, 201], OperandSize::Word)
}

fn fxch_2() {
    run_test(&Instruction { mnemonic: Mnemonic::FXCH, operand1: Some(Direct(ST3)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[217, 203], OperandSize::Dword)
}

fn fxch_3() {
    run_test(&Instruction { mnemonic: Mnemonic::FXCH, operand1: Some(Direct(ST3)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[217, 203], OperandSize::Qword)
}

