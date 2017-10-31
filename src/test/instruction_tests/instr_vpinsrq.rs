use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpinsrq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPINSRQ, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(RBP)), operand4: Some(Literal8(99)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 217, 34, 229, 99], OperandSize::Qword)
}

#[test]
fn vpinsrq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPINSRQ, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledDisplaced(RSI, Four, 1816078182, Some(OperandSize::Qword), None)), operand4: Some(Literal8(83)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 209, 34, 12, 181, 102, 39, 63, 108, 83], OperandSize::Qword)
}

#[test]
fn vpinsrq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPINSRQ, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM8)), operand3: Some(Direct(RSP)), operand4: Some(Literal8(80)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 185, 34, 196, 80], OperandSize::Qword)
}

#[test]
fn vpinsrq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPINSRQ, operand1: Some(Direct(XMM21)), operand2: Some(Direct(XMM19)), operand3: Some(IndirectScaledIndexed(RDI, RDI, Eight, Some(OperandSize::Qword), None)), operand4: Some(Literal8(126)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 227, 229, 0, 34, 44, 255, 126], OperandSize::Qword)
}

