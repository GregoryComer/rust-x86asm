use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn fsubrp_1() {
    run_test(&Instruction { mnemonic: Mnemonic::FSUBRP, operand1: Some(Direct(ST2)), operand2: Some(Direct(ST)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[222, 226], OperandSize::Word)
}

fn fsubrp_2() {
    run_test(&Instruction { mnemonic: Mnemonic::FSUBRP, operand1: Some(Direct(ST3)), operand2: Some(Direct(ST)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[222, 227], OperandSize::Dword)
}

fn fsubrp_3() {
    run_test(&Instruction { mnemonic: Mnemonic::FSUBRP, operand1: Some(Direct(ST5)), operand2: Some(Direct(ST)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[222, 229], OperandSize::Qword)
}

