use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn pinsrq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PINSRQ, operand1: Some(Direct(XMM5)), operand2: Some(Direct(RSP)), operand3: Some(Literal8(74)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 72, 15, 58, 34, 236, 74], OperandSize::Qword)
}

#[test]
fn pinsrq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PINSRQ, operand1: Some(Direct(XMM7)), operand2: Some(Indirect(RBX, Some(OperandSize::Qword), None)), operand3: Some(Literal8(12)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 72, 15, 58, 34, 59, 12], OperandSize::Qword)
}

