use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn fcmovnb_1() {
    run_test(&Instruction { mnemonic: Mnemonic::FCMOVNB, operand1: Some(Direct(ST)), operand2: Some(Direct(ST7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[219, 199], OperandSize::Word)
}

fn fcmovnb_2() {
    run_test(&Instruction { mnemonic: Mnemonic::FCMOVNB, operand1: Some(Direct(ST)), operand2: Some(Direct(ST1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[219, 193], OperandSize::Dword)
}

fn fcmovnb_3() {
    run_test(&Instruction { mnemonic: Mnemonic::FCMOVNB, operand1: Some(Direct(ST)), operand2: Some(Direct(ST7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[219, 199], OperandSize::Qword)
}

