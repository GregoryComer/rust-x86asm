use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vblendmps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VBLENDMPS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 109, 137, 101, 217], OperandSize::Dword)
}

fn vblendmps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VBLENDMPS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledIndexed(ECX, EDX, Two, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 85, 141, 101, 4, 81], OperandSize::Dword)
}

fn vblendmps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VBLENDMPS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM7)), operand3: Some(Indirect(ECX, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 69, 153, 101, 33], OperandSize::Dword)
}

fn vblendmps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VBLENDMPS, operand1: Some(Direct(XMM16)), operand2: Some(Direct(XMM23)), operand3: Some(Direct(XMM14)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 194, 69, 130, 101, 198], OperandSize::Qword)
}

fn vblendmps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VBLENDMPS, operand1: Some(Direct(XMM21)), operand2: Some(Direct(XMM20)), operand3: Some(IndirectDisplaced(RDX, 323485203, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 226, 93, 130, 101, 170, 19, 254, 71, 19], OperandSize::Qword)
}

fn vblendmps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VBLENDMPS, operand1: Some(Direct(XMM22)), operand2: Some(Direct(XMM23)), operand3: Some(IndirectScaledDisplaced(RSI, Eight, 2071091723, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 226, 69, 145, 101, 52, 245, 11, 90, 114, 123], OperandSize::Qword)
}

fn vblendmps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VBLENDMPS, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(YMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 85, 172, 101, 217], OperandSize::Dword)
}

fn vblendmps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VBLENDMPS, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectDisplaced(ESI, 417532710, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 77, 170, 101, 190, 38, 11, 227, 24], OperandSize::Dword)
}

fn vblendmps_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VBLENDMPS, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM4)), operand3: Some(Indirect(EAX, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 93, 191, 101, 24], OperandSize::Dword)
}

fn vblendmps_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VBLENDMPS, operand1: Some(Direct(YMM12)), operand2: Some(Direct(YMM25)), operand3: Some(Direct(YMM30)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 18, 53, 162, 101, 230], OperandSize::Qword)
}

fn vblendmps_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VBLENDMPS, operand1: Some(Direct(YMM21)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectScaledIndexed(RSI, RDX, Eight, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 226, 117, 175, 101, 44, 214], OperandSize::Qword)
}

fn vblendmps_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VBLENDMPS, operand1: Some(Direct(YMM9)), operand2: Some(Direct(YMM30)), operand3: Some(IndirectScaledIndexedDisplaced(RBX, RDI, Four, 1904223971, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 114, 13, 182, 101, 140, 187, 227, 38, 128, 113], OperandSize::Qword)
}

fn vblendmps_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VBLENDMPS, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM7)), operand3: Some(Direct(ZMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 69, 206, 101, 200], OperandSize::Dword)
}

fn vblendmps_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VBLENDMPS, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM5)), operand3: Some(Indirect(EBX, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 85, 205, 101, 35], OperandSize::Dword)
}

fn vblendmps_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VBLENDMPS, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM2)), operand3: Some(IndirectScaledIndexed(EDX, EDI, Eight, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 242, 109, 222, 101, 60, 250], OperandSize::Dword)
}

fn vblendmps_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VBLENDMPS, operand1: Some(Direct(ZMM29)), operand2: Some(Direct(ZMM31)), operand3: Some(Direct(ZMM13)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 66, 5, 199, 101, 237], OperandSize::Qword)
}

fn vblendmps_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VBLENDMPS, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM11)), operand3: Some(IndirectScaledDisplaced(RDX, Four, 1872180151, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 37, 203, 101, 52, 149, 183, 51, 151, 111], OperandSize::Qword)
}

fn vblendmps_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VBLENDMPS, operand1: Some(Direct(ZMM22)), operand2: Some(Direct(ZMM19)), operand3: Some(Indirect(RAX, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 226, 101, 213, 101, 48], OperandSize::Qword)
}

