use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn pminud_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PMINUD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 59, 205], OperandSize::Dword)
}

fn pminud_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PMINUD, operand1: Some(Direct(XMM4)), operand2: Some(IndirectScaledDisplaced(ESI, Two, 1986214127, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 59, 36, 117, 239, 56, 99, 118], OperandSize::Dword)
}

fn pminud_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PMINUD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 59, 211], OperandSize::Qword)
}

fn pminud_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PMINUD, operand1: Some(Direct(XMM3)), operand2: Some(IndirectScaledDisplaced(RDX, Two, 623852833, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 59, 28, 85, 33, 61, 47, 37], OperandSize::Qword)
}

