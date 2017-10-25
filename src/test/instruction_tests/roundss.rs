use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn roundss_1() {
    run_test(&Instruction { mnemonic: Mnemonic::ROUNDSS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM4)), operand3: Some(Literal8(43)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 10, 196, 43], OperandSize::Dword)
}

fn roundss_2() {
    run_test(&Instruction { mnemonic: Mnemonic::ROUNDSS, operand1: Some(Direct(XMM7)), operand2: Some(IndirectScaledIndexedDisplaced(EDX, EDX, Eight, 668329949, Some(OperandSize::Dword), None)), operand3: Some(Literal8(119)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 10, 188, 210, 221, 231, 213, 39, 119], OperandSize::Dword)
}

fn roundss_3() {
    run_test(&Instruction { mnemonic: Mnemonic::ROUNDSS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM7)), operand3: Some(Literal8(90)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 10, 239, 90], OperandSize::Qword)
}

fn roundss_4() {
    run_test(&Instruction { mnemonic: Mnemonic::ROUNDSS, operand1: Some(Direct(XMM2)), operand2: Some(IndirectScaledIndexed(RSI, RBX, Two, Some(OperandSize::Dword), None)), operand3: Some(Literal8(60)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 10, 20, 94, 60], OperandSize::Qword)
}

