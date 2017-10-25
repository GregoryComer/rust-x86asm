use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn fcmovb_1() {
    run_test(&Instruction { mnemonic: Mnemonic::FCMOVB, operand1: Some(Direct(ST)), operand2: Some(Direct(ST6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[218, 198], OperandSize::Word)
}

fn fcmovb_2() {
    run_test(&Instruction { mnemonic: Mnemonic::FCMOVB, operand1: Some(Direct(ST)), operand2: Some(Direct(ST1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[218, 193], OperandSize::Dword)
}

fn fcmovb_3() {
    run_test(&Instruction { mnemonic: Mnemonic::FCMOVB, operand1: Some(Direct(ST)), operand2: Some(Direct(ST2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[218, 194], OperandSize::Qword)
}

