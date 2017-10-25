use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn pcmpeqd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PCMPEQD, operand1: Some(Direct(MM1)), operand2: Some(Direct(MM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 118, 203], OperandSize::Dword)
}

fn pcmpeqd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PCMPEQD, operand1: Some(Direct(MM5)), operand2: Some(IndirectScaledIndexedDisplaced(ESI, ESI, Four, 947566226, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 118, 172, 182, 146, 182, 122, 56], OperandSize::Dword)
}

fn pcmpeqd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PCMPEQD, operand1: Some(Direct(MM4)), operand2: Some(Direct(MM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 118, 230], OperandSize::Qword)
}

fn pcmpeqd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PCMPEQD, operand1: Some(Direct(MM3)), operand2: Some(Indirect(RCX, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 118, 25], OperandSize::Qword)
}

fn pcmpeqd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PCMPEQD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 118, 237], OperandSize::Dword)
}

fn pcmpeqd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PCMPEQD, operand1: Some(Direct(XMM0)), operand2: Some(IndirectScaledIndexed(ESI, EDX, Two, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 118, 4, 86], OperandSize::Dword)
}

fn pcmpeqd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PCMPEQD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 118, 204], OperandSize::Qword)
}

fn pcmpeqd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PCMPEQD, operand1: Some(Direct(XMM6)), operand2: Some(IndirectDisplaced(RSI, 1655550485, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 118, 182, 21, 178, 173, 98], OperandSize::Qword)
}

