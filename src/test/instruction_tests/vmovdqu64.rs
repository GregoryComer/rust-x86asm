use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vmovdqu64_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU64, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 254, 139, 111, 230], OperandSize::Dword)
}

fn vmovdqu64_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU64, operand1: Some(Direct(XMM6)), operand2: Some(IndirectScaledDisplaced(ESI, Four, 687145931, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 254, 141, 111, 52, 181, 203, 3, 245, 40], OperandSize::Dword)
}

fn vmovdqu64_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU64, operand1: Some(Direct(XMM11)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 113, 254, 143, 111, 221], OperandSize::Qword)
}

fn vmovdqu64_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU64, operand1: Some(Direct(XMM26)), operand2: Some(IndirectScaledIndexed(RCX, RAX, Four, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 97, 254, 139, 111, 20, 129], OperandSize::Qword)
}

fn vmovdqu64_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU64, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 254, 171, 111, 217], OperandSize::Dword)
}

fn vmovdqu64_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU64, operand1: Some(Direct(YMM5)), operand2: Some(IndirectDisplaced(ESI, 705150405, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 254, 170, 111, 174, 197, 189, 7, 42], OperandSize::Dword)
}

fn vmovdqu64_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU64, operand1: Some(Direct(YMM18)), operand2: Some(Direct(YMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 225, 254, 173, 111, 209], OperandSize::Qword)
}

fn vmovdqu64_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU64, operand1: Some(Direct(YMM12)), operand2: Some(Indirect(RCX, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 113, 254, 173, 111, 33], OperandSize::Qword)
}

fn vmovdqu64_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU64, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 254, 201, 111, 220], OperandSize::Dword)
}

fn vmovdqu64_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU64, operand1: Some(Direct(ZMM2)), operand2: Some(IndirectScaledIndexedDisplaced(EAX, ECX, Four, 1665306664, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 254, 201, 111, 148, 136, 40, 144, 66, 99], OperandSize::Dword)
}

fn vmovdqu64_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU64, operand1: Some(Direct(ZMM14)), operand2: Some(Direct(ZMM10)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 81, 254, 204, 111, 242], OperandSize::Qword)
}

fn vmovdqu64_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU64, operand1: Some(Direct(ZMM25)), operand2: Some(IndirectDisplaced(RCX, 172185680, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 97, 254, 202, 111, 137, 80, 88, 67, 10], OperandSize::Qword)
}

fn vmovdqu64_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU64, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 254, 141, 111, 204], OperandSize::Dword)
}

fn vmovdqu64_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU64, operand1: Some(Indirect(EDX, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 254, 8, 127, 50], OperandSize::Dword)
}

fn vmovdqu64_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU64, operand1: Some(Direct(XMM25)), operand2: Some(Direct(XMM31)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 1, 254, 142, 111, 207], OperandSize::Qword)
}

fn vmovdqu64_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU64, operand1: Some(Indirect(RDX, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM16)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 225, 254, 8, 127, 2], OperandSize::Qword)
}

fn vmovdqu64_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU64, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 254, 171, 111, 228], OperandSize::Dword)
}

fn vmovdqu64_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU64, operand1: Some(IndirectScaledIndexed(EDI, EAX, Four, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(YMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 254, 40, 127, 12, 135], OperandSize::Dword)
}

fn vmovdqu64_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU64, operand1: Some(Direct(YMM20)), operand2: Some(Direct(YMM12)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 193, 254, 170, 111, 228], OperandSize::Qword)
}

fn vmovdqu64_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU64, operand1: Some(IndirectScaledDisplaced(RDX, Two, 1060697901, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(YMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 254, 40, 127, 4, 85, 45, 247, 56, 63], OperandSize::Qword)
}

fn vmovdqu64_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU64, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 254, 201, 111, 238], OperandSize::Dword)
}

fn vmovdqu64_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU64, operand1: Some(Indirect(EBX, Some(OperandSize::Zmmword), None)), operand2: Some(Direct(ZMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 254, 72, 127, 51], OperandSize::Dword)
}

fn vmovdqu64_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU64, operand1: Some(Direct(ZMM14)), operand2: Some(Direct(ZMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 113, 254, 205, 111, 246], OperandSize::Qword)
}

fn vmovdqu64_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU64, operand1: Some(IndirectScaledIndexed(RDX, RSI, Four, Some(OperandSize::Zmmword), None)), operand2: Some(Direct(ZMM13)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 113, 254, 72, 127, 44, 178], OperandSize::Qword)
}

