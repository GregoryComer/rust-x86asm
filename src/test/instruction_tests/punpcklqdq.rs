use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn punpcklqdq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PUNPCKLQDQ, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 108, 216], OperandSize::Dword)
}

fn punpcklqdq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PUNPCKLQDQ, operand1: Some(Direct(XMM3)), operand2: Some(IndirectDisplaced(EDX, 301743310, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 108, 154, 206, 60, 252, 17], OperandSize::Dword)
}

fn punpcklqdq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PUNPCKLQDQ, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 108, 237], OperandSize::Qword)
}

fn punpcklqdq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PUNPCKLQDQ, operand1: Some(Direct(XMM7)), operand2: Some(IndirectScaledDisplaced(RBX, Four, 1165893422, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 108, 60, 157, 46, 31, 126, 69], OperandSize::Qword)
}

