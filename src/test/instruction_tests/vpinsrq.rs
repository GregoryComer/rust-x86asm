use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vpinsrq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPINSRQ, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(RSI)), operand4: Some(Literal8(19)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 209, 34, 230, 19], OperandSize::Qword)
}

fn vpinsrq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPINSRQ, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM2)), operand3: Some(Indirect(RDI, Some(OperandSize::Qword), None)), operand4: Some(Literal8(95)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 233, 34, 39, 95], OperandSize::Qword)
}

fn vpinsrq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPINSRQ, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM8)), operand3: Some(Direct(RBX)), operand4: Some(Literal8(54)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 185, 34, 219, 54], OperandSize::Qword)
}

fn vpinsrq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPINSRQ, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM9)), operand3: Some(IndirectDisplaced(RBX, 1496530561, Some(OperandSize::Qword), None)), operand4: Some(Literal8(117)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 177, 34, 171, 129, 62, 51, 89, 117], OperandSize::Qword)
}

