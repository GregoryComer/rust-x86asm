use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn fcmovnbe_1() {
    run_test(&Instruction { mnemonic: Mnemonic::FCMOVNBE, operand1: Some(Direct(ST)), operand2: Some(Direct(ST2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[219, 210], OperandSize::Word)
}

fn fcmovnbe_2() {
    run_test(&Instruction { mnemonic: Mnemonic::FCMOVNBE, operand1: Some(Direct(ST)), operand2: Some(Direct(ST3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[219, 211], OperandSize::Dword)
}

fn fcmovnbe_3() {
    run_test(&Instruction { mnemonic: Mnemonic::FCMOVNBE, operand1: Some(Direct(ST)), operand2: Some(Direct(ST5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[219, 213], OperandSize::Qword)
}

