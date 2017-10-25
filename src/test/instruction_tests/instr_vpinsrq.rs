use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpinsrq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPINSRQ, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(RBP)), operand4: Some(Literal8(51)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 249, 34, 237, 51], OperandSize::Qword)
}

#[test]
fn vpinsrq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPINSRQ, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledIndexedDisplaced(RBX, RCX, Two, 1116810070, Some(OperandSize::Qword), None)), operand4: Some(Literal8(118)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 225, 34, 188, 75, 86, 43, 145, 66, 118], OperandSize::Qword)
}

#[test]
fn vpinsrq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPINSRQ, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(RSP)), operand4: Some(Literal8(54)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 217, 34, 228, 54], OperandSize::Qword)
}

#[test]
fn vpinsrq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPINSRQ, operand1: Some(Direct(XMM16)), operand2: Some(Direct(XMM25)), operand3: Some(IndirectScaledIndexedDisplaced(RBX, RSI, Two, 956422095, Some(OperandSize::Qword), None)), operand4: Some(Literal8(114)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 227, 181, 0, 34, 132, 115, 207, 215, 1, 57, 114], OperandSize::Qword)
}

