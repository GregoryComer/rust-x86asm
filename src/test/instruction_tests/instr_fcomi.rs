use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn fcomi_1() {
    run_test(&Instruction { mnemonic: Mnemonic::FCOMI, operand1: Some(Direct(ST)), operand2: Some(Direct(ST2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[219, 242], OperandSize::Word)
}

#[test]
fn fcomi_2() {
    run_test(&Instruction { mnemonic: Mnemonic::FCOMI, operand1: Some(Direct(ST)), operand2: Some(Direct(ST3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[219, 243], OperandSize::Dword)
}

#[test]
fn fcomi_3() {
    run_test(&Instruction { mnemonic: Mnemonic::FCOMI, operand1: Some(Direct(ST)), operand2: Some(Direct(ST4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[219, 244], OperandSize::Qword)
}

