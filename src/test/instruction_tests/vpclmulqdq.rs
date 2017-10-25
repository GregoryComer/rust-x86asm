use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vpclmulqdq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCLMULQDQ, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM6)), operand4: Some(Literal8(91)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 113, 68, 206, 91], OperandSize::Dword)
}

fn vpclmulqdq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCLMULQDQ, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledDisplaced(EAX, Two, 1449849722, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(39)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 105, 68, 28, 69, 122, 243, 106, 86, 39], OperandSize::Dword)
}

fn vpclmulqdq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCLMULQDQ, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM7)), operand4: Some(Literal8(66)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 97, 68, 247, 66], OperandSize::Qword)
}

fn vpclmulqdq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCLMULQDQ, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectDisplaced(RBX, 1068253788, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(20)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 73, 68, 187, 92, 66, 172, 63, 20], OperandSize::Qword)
}

