use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vaddps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDPS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 240, 88, 255], OperandSize::Dword)
}

fn vaddps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDPS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectDisplaced(EBX, 1455175338, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 224, 88, 139, 170, 54, 188, 86], OperandSize::Dword)
}

fn vaddps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDPS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 216, 88, 216], OperandSize::Qword)
}

fn vaddps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDPS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM6)), operand3: Some(Indirect(RCX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 200, 88, 25], OperandSize::Qword)
}

fn vaddps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDPS, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(YMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 228, 88, 205], OperandSize::Dword)
}

fn vaddps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDPS, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM5)), operand3: Some(Indirect(EAX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 212, 88, 32], OperandSize::Dword)
}

fn vaddps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDPS, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 244, 88, 220], OperandSize::Qword)
}

fn vaddps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDPS, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM2)), operand3: Some(Indirect(RSI, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 236, 88, 62], OperandSize::Qword)
}

fn vaddps_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDPS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 124, 140, 88, 215], OperandSize::Dword)
}

fn vaddps_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDPS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectScaledIndexedDisplaced(ECX, EBX, Four, 1708954285, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 116, 138, 88, 148, 153, 173, 146, 220, 101], OperandSize::Dword)
}

fn vaddps_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDPS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledDisplaced(ESI, Two, 2051780532, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 124, 154, 88, 44, 117, 180, 175, 75, 122], OperandSize::Dword)
}

fn vaddps_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDPS, operand1: Some(Direct(XMM12)), operand2: Some(Direct(XMM28)), operand3: Some(Direct(XMM18)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 49, 28, 129, 88, 226], OperandSize::Qword)
}

fn vaddps_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDPS, operand1: Some(Direct(XMM21)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledDisplaced(RDI, Four, 1155812809, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 225, 124, 140, 88, 44, 189, 201, 77, 228, 68], OperandSize::Qword)
}

fn vaddps_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDPS, operand1: Some(Direct(XMM26)), operand2: Some(Direct(XMM19)), operand3: Some(IndirectScaledDisplaced(RAX, Four, 830737728, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 97, 100, 147, 88, 20, 133, 64, 13, 132, 49], OperandSize::Qword)
}

fn vaddps_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDPS, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(YMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 76, 171, 88, 196], OperandSize::Dword)
}

fn vaddps_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDPS, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM4)), operand3: Some(Indirect(EBX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 92, 171, 88, 51], OperandSize::Dword)
}

fn vaddps_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDPS, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectScaledIndexed(EAX, EDI, Two, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 84, 188, 88, 36, 120], OperandSize::Dword)
}

fn vaddps_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDPS, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM26)), operand3: Some(Direct(YMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 44, 167, 88, 198], OperandSize::Qword)
}

fn vaddps_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDPS, operand1: Some(Direct(YMM17)), operand2: Some(Direct(YMM16)), operand3: Some(IndirectScaledDisplaced(RDX, Two, 798393740, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 225, 124, 162, 88, 12, 85, 140, 133, 150, 47], OperandSize::Qword)
}

fn vaddps_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDPS, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM18)), operand3: Some(IndirectScaledIndexedDisplaced(RCX, RAX, Four, 582126552, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 108, 177, 88, 180, 129, 216, 139, 178, 34], OperandSize::Qword)
}

fn vaddps_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDPS, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM6)), operand3: Some(Direct(ZMM3)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Nearest), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 76, 157, 88, 211], OperandSize::Dword)
}

fn vaddps_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDPS, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM6)), operand3: Some(IndirectScaledIndexed(EDI, EDI, Four, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 76, 206, 88, 52, 191], OperandSize::Dword)
}

fn vaddps_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDPS, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM4)), operand3: Some(IndirectDisplaced(EBX, 24658066, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 241, 92, 223, 88, 147, 146, 64, 120, 1], OperandSize::Dword)
}

fn vaddps_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDPS, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM12)), operand3: Some(Direct(ZMM19)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Down), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 177, 28, 191, 88, 203], OperandSize::Qword)
}

fn vaddps_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDPS, operand1: Some(Direct(ZMM11)), operand2: Some(Direct(ZMM19)), operand3: Some(Indirect(RDX, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 113, 100, 198, 88, 26], OperandSize::Qword)
}

fn vaddps_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDPS, operand1: Some(Direct(ZMM13)), operand2: Some(Direct(ZMM19)), operand3: Some(IndirectDisplaced(RDX, 503008698, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 113, 100, 213, 88, 170, 186, 77, 251, 29], OperandSize::Qword)
}

