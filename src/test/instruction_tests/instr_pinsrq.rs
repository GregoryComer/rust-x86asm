use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn pinsrq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PINSRQ, operand1: Some(Direct(XMM5)), operand2: Some(Direct(RBP)), operand3: Some(Literal8(22)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 72, 15, 58, 34, 237, 22], OperandSize::Qword)
}

#[test]
fn pinsrq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PINSRQ, operand1: Some(Direct(XMM4)), operand2: Some(IndirectScaledDisplaced(RSI, Eight, 1948738369, Some(OperandSize::Qword), None)), operand3: Some(Literal8(46)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 72, 15, 58, 34, 36, 245, 65, 99, 39, 116, 46], OperandSize::Qword)
}

