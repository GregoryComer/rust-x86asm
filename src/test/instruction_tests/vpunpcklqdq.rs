use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vpunpcklqdq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKLQDQ, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 201, 108, 200], OperandSize::Dword)
}

fn vpunpcklqdq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKLQDQ, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledIndexed(ESI, EDX, Eight, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 108, 60, 214], OperandSize::Dword)
}

fn vpunpcklqdq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKLQDQ, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 193, 108, 246], OperandSize::Qword)
}

fn vpunpcklqdq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKLQDQ, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectDisplaced(RCX, 1065473769, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 193, 108, 161, 233, 214, 129, 63], OperandSize::Qword)
}

fn vpunpcklqdq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKLQDQ, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(YMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 205, 108, 206], OperandSize::Dword)
}

fn vpunpcklqdq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKLQDQ, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectScaledIndexedDisplaced(EDX, EAX, Four, 1291193056, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 229, 108, 164, 130, 224, 10, 246, 76], OperandSize::Dword)
}

fn vpunpcklqdq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKLQDQ, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(YMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 108, 239], OperandSize::Qword)
}

fn vpunpcklqdq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKLQDQ, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectScaledIndexedDisplaced(RBX, RDX, Eight, 1190916097, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 237, 108, 148, 211, 1, 240, 251, 70], OperandSize::Qword)
}

fn vpunpcklqdq_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKLQDQ, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 205, 141, 108, 205], OperandSize::Dword)
}

fn vpunpcklqdq_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKLQDQ, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectDisplaced(EBX, 1873247164, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 221, 140, 108, 187, 188, 123, 167, 111], OperandSize::Dword)
}

fn vpunpcklqdq_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKLQDQ, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledIndexed(EAX, EBX, Eight, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 241, 237, 155, 108, 60, 216], OperandSize::Dword)
}

fn vpunpcklqdq_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKLQDQ, operand1: Some(Direct(XMM8)), operand2: Some(Direct(XMM31)), operand3: Some(Direct(XMM31)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 17, 133, 131, 108, 199], OperandSize::Qword)
}

fn vpunpcklqdq_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKLQDQ, operand1: Some(Direct(XMM17)), operand2: Some(Direct(XMM8)), operand3: Some(IndirectDisplaced(RAX, 1452512606, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 225, 189, 138, 108, 136, 94, 149, 147, 86], OperandSize::Qword)
}

fn vpunpcklqdq_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKLQDQ, operand1: Some(Direct(XMM16)), operand2: Some(Direct(XMM13)), operand3: Some(IndirectScaledDisplaced(RBX, Two, 215535390, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 225, 149, 156, 108, 4, 93, 30, 207, 216, 12], OperandSize::Qword)
}

