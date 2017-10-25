use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn psubq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PSUBQ, operand1: Some(Direct(MM1)), operand2: Some(Direct(MM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 251, 206], OperandSize::Dword)
}

fn psubq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PSUBQ, operand1: Some(Direct(MM0)), operand2: Some(IndirectScaledIndexed(EAX, EDI, Two, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 251, 4, 120], OperandSize::Dword)
}

fn psubq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PSUBQ, operand1: Some(Direct(MM1)), operand2: Some(Direct(MM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 251, 200], OperandSize::Qword)
}

fn psubq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PSUBQ, operand1: Some(Direct(MM6)), operand2: Some(IndirectDisplaced(RSI, 1734635209, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 251, 182, 201, 110, 100, 103], OperandSize::Qword)
}

fn psubq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PSUBQ, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 251, 222], OperandSize::Dword)
}

fn psubq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PSUBQ, operand1: Some(Direct(XMM5)), operand2: Some(IndirectDisplaced(EDI, 822443064, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 251, 175, 56, 124, 5, 49], OperandSize::Dword)
}

fn psubq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PSUBQ, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 251, 215], OperandSize::Qword)
}

fn psubq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PSUBQ, operand1: Some(Direct(XMM0)), operand2: Some(IndirectScaledIndexed(RSI, RSI, Two, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 251, 4, 118], OperandSize::Qword)
}

