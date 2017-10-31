use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn kandb_1() {
    run_test(&Instruction { mnemonic: Mnemonic::KANDB, operand1: Some(Direct(K6)), operand2: Some(Direct(K4)), operand3: Some(Direct(K3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 221, 65, 243], OperandSize::Dword)
}

#[test]
fn kandb_2() {
    run_test(&Instruction { mnemonic: Mnemonic::KANDB, operand1: Some(Direct(K5)), operand2: Some(Direct(K3)), operand3: Some(Direct(K3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 229, 65, 235], OperandSize::Qword)
}

