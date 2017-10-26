use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn xabort_1() {
    run_test(&Instruction { mnemonic: Mnemonic::XABORT, operand1: Some(Literal8(97)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[198, 248, 97], OperandSize::Dword)
}

#[test]
fn xabort_2() {
    run_test(&Instruction { mnemonic: Mnemonic::XABORT, operand1: Some(Literal8(108)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[198, 248, 108], OperandSize::Qword)
}

