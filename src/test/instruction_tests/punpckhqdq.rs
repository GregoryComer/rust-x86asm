use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn punpckhqdq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PUNPCKHQDQ, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 109, 203], OperandSize::Dword)
}

fn punpckhqdq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PUNPCKHQDQ, operand1: Some(Direct(XMM5)), operand2: Some(IndirectScaledIndexed(EAX, EAX, Four, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 109, 44, 128], OperandSize::Dword)
}

fn punpckhqdq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PUNPCKHQDQ, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 109, 229], OperandSize::Qword)
}

fn punpckhqdq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PUNPCKHQDQ, operand1: Some(Direct(XMM6)), operand2: Some(Indirect(RDI, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 109, 55], OperandSize::Qword)
}

