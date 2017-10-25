use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn fcmovbe_1() {
    run_test(&Instruction { mnemonic: Mnemonic::FCMOVBE, operand1: Some(Direct(ST)), operand2: Some(Direct(ST5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[218, 213], OperandSize::Word)
}

fn fcmovbe_2() {
    run_test(&Instruction { mnemonic: Mnemonic::FCMOVBE, operand1: Some(Direct(ST)), operand2: Some(Direct(ST4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[218, 212], OperandSize::Dword)
}

fn fcmovbe_3() {
    run_test(&Instruction { mnemonic: Mnemonic::FCMOVBE, operand1: Some(Direct(ST)), operand2: Some(Direct(ST7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[218, 215], OperandSize::Qword)
}

