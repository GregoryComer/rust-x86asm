use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn pcmpeqq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PCMPEQQ, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 41, 215], OperandSize::Dword)
}

fn pcmpeqq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PCMPEQQ, operand1: Some(Direct(XMM6)), operand2: Some(IndirectScaledIndexed(EDX, ESI, Two, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 41, 52, 114], OperandSize::Dword)
}

fn pcmpeqq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PCMPEQQ, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 41, 217], OperandSize::Qword)
}

fn pcmpeqq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PCMPEQQ, operand1: Some(Direct(XMM3)), operand2: Some(IndirectScaledDisplaced(RDI, Eight, 2108884197, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 41, 28, 253, 229, 4, 179, 125], OperandSize::Qword)
}

