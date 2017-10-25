use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vpunpcklbw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKLBW, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 217, 96, 212], OperandSize::Dword)
}

fn vpunpcklbw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKLBW, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM6)), operand3: Some(Indirect(EAX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 201, 96, 16], OperandSize::Dword)
}

fn vpunpcklbw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKLBW, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 96, 234], OperandSize::Qword)
}

fn vpunpcklbw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKLBW, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM3)), operand3: Some(Indirect(RAX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 225, 96, 56], OperandSize::Qword)
}

fn vpunpcklbw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKLBW, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(YMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 229, 96, 232], OperandSize::Dword)
}

fn vpunpcklbw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKLBW, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectDisplaced(ECX, 1651166646, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 213, 96, 145, 182, 205, 106, 98], OperandSize::Dword)
}

fn vpunpcklbw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKLBW, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(YMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 96, 226], OperandSize::Qword)
}

fn vpunpcklbw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKLBW, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectScaledIndexed(RDI, RDI, Two, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 237, 96, 4, 127], OperandSize::Qword)
}

fn vpunpcklbw_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKLBW, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 69, 143, 96, 237], OperandSize::Dword)
}

fn vpunpcklbw_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKLBW, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledIndexed(EDX, ESI, Two, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 69, 142, 96, 28, 114], OperandSize::Dword)
}

fn vpunpcklbw_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKLBW, operand1: Some(Direct(XMM11)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 113, 93, 141, 96, 220], OperandSize::Qword)
}

fn vpunpcklbw_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKLBW, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM11)), operand3: Some(IndirectScaledDisplaced(RDI, Two, 340667630, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 37, 142, 96, 12, 125, 238, 44, 78, 20], OperandSize::Qword)
}

