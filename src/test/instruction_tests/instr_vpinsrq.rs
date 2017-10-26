use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpinsrq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPINSRQ, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(RSI)), operand4: Some(Literal8(12)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 193, 34, 206, 12], OperandSize::Qword)
}

#[test]
fn vpinsrq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPINSRQ, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledDisplaced(RCX, Four, 1429991973, Some(OperandSize::Qword), None)), operand4: Some(Literal8(84)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 193, 34, 12, 141, 37, 242, 59, 85, 84], OperandSize::Qword)
}

#[test]
fn vpinsrq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPINSRQ, operand1: Some(Direct(XMM12)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(RSI)), operand4: Some(Literal8(58)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 99, 193, 34, 230, 58], OperandSize::Qword)
}

#[test]
fn vpinsrq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPINSRQ, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM17)), operand3: Some(IndirectScaledIndexed(RAX, RCX, Two, Some(OperandSize::Qword), None)), operand4: Some(Literal8(18)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 243, 245, 0, 34, 36, 72, 18], OperandSize::Qword)
}

