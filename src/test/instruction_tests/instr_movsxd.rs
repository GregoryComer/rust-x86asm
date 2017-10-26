use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn movsxd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVSXD, operand1: Some(Direct(RSI)), operand2: Some(Direct(ECX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 99, 241], OperandSize::Qword)
}

#[test]
fn movsxd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVSXD, operand1: Some(Direct(RSI)), operand2: Some(Indirect(RDI, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 99, 55], OperandSize::Qword)
}

