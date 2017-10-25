use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn fcmovnu_1() {
    run_test(&Instruction { mnemonic: Mnemonic::FCMOVNU, operand1: Some(Direct(ST)), operand2: Some(Direct(ST7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[219, 223], OperandSize::Word)
}

fn fcmovnu_2() {
    run_test(&Instruction { mnemonic: Mnemonic::FCMOVNU, operand1: Some(Direct(ST)), operand2: Some(Direct(ST3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[219, 219], OperandSize::Dword)
}

fn fcmovnu_3() {
    run_test(&Instruction { mnemonic: Mnemonic::FCMOVNU, operand1: Some(Direct(ST)), operand2: Some(Direct(ST3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[219, 219], OperandSize::Qword)
}

