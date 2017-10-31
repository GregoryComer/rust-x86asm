use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn kaddb_1() {
    run_test(&Instruction { mnemonic: Mnemonic::KADDB, operand1: Some(Direct(K4)), operand2: Some(Direct(K6)), operand3: Some(Direct(K7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 205, 74, 231], OperandSize::Dword)
}

#[test]
fn kaddb_2() {
    run_test(&Instruction { mnemonic: Mnemonic::KADDB, operand1: Some(Direct(K7)), operand2: Some(Direct(K6)), operand3: Some(Direct(K4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 205, 74, 252], OperandSize::Qword)
}

