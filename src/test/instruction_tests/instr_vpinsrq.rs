use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpinsrq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPINSRQ, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(RSI)), operand4: Some(Literal8(60)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 193, 34, 238, 60], OperandSize::Qword)
}

#[test]
fn vpinsrq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPINSRQ, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectScaledDisplaced(RCX, Two, 2105445561, Some(OperandSize::Qword), None)), operand4: Some(Literal8(68)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 241, 34, 20, 77, 185, 140, 126, 125, 68], OperandSize::Qword)
}

#[test]
fn vpinsrq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPINSRQ, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM28)), operand3: Some(Direct(RDX)), operand4: Some(Literal8(17)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 243, 157, 0, 34, 242, 17], OperandSize::Qword)
}

#[test]
fn vpinsrq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPINSRQ, operand1: Some(Direct(XMM23)), operand2: Some(Direct(XMM12)), operand3: Some(IndirectDisplaced(RAX, 1458163118, Some(OperandSize::Qword), None)), operand4: Some(Literal8(108)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 227, 157, 8, 34, 184, 174, 205, 233, 86, 108], OperandSize::Qword)
}

