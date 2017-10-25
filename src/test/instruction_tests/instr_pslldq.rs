use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn pslldq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PSLLDQ, operand1: Some(Direct(XMM6)), operand2: Some(Literal8(92)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 115, 254, 92], OperandSize::Dword)
}

#[test]
fn pslldq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PSLLDQ, operand1: Some(Direct(XMM5)), operand2: Some(Literal8(61)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 115, 253, 61], OperandSize::Qword)
}

