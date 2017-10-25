use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vdivps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VDIVPS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 216, 94, 230], OperandSize::Dword)
}

fn vdivps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VDIVPS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledIndexed(EAX, ECX, Four, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 94, 12, 136], OperandSize::Dword)
}

fn vdivps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VDIVPS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 232, 94, 248], OperandSize::Qword)
}

fn vdivps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VDIVPS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectScaledDisplaced(RDI, Four, 1063568218, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 200, 94, 52, 189, 90, 195, 100, 63], OperandSize::Qword)
}

fn vdivps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VDIVPS, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 244, 94, 199], OperandSize::Dword)
}

fn vdivps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VDIVPS, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectDisplaced(EDX, 220527622, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 244, 94, 154, 6, 252, 36, 13], OperandSize::Dword)
}

fn vdivps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VDIVPS, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 236, 94, 204], OperandSize::Qword)
}

fn vdivps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VDIVPS, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectScaledIndexed(RDI, RCX, Two, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 244, 94, 4, 79], OperandSize::Qword)
}

fn vdivps_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VDIVPS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 92, 142, 94, 232], OperandSize::Dword)
}

fn vdivps_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VDIVPS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledDisplaced(ESI, Eight, 1035835824, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 108, 139, 94, 36, 245, 176, 153, 189, 61], OperandSize::Dword)
}

fn vdivps_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VDIVPS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM4)), operand3: Some(Indirect(EDX, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 92, 157, 94, 18], OperandSize::Dword)
}

fn vdivps_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VDIVPS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM28)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 28, 134, 94, 216], OperandSize::Qword)
}

fn vdivps_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VDIVPS, operand1: Some(Direct(XMM15)), operand2: Some(Direct(XMM10)), operand3: Some(IndirectScaledIndexed(RDI, RDX, Four, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 113, 44, 143, 94, 60, 151], OperandSize::Qword)
}

fn vdivps_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VDIVPS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledIndexed(RSI, RBX, Four, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 92, 154, 94, 44, 158], OperandSize::Qword)
}

fn vdivps_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VDIVPS, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(YMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 76, 172, 94, 202], OperandSize::Dword)
}

fn vdivps_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VDIVPS, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectScaledDisplaced(EDX, Four, 1639449905, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 68, 172, 94, 52, 149, 49, 5, 184, 97], OperandSize::Dword)
}

fn vdivps_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VDIVPS, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectScaledIndexedDisplaced(ESI, EAX, Four, 882641375, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 116, 188, 94, 140, 134, 223, 9, 156, 52], OperandSize::Dword)
}

fn vdivps_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VDIVPS, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(YMM13)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 209, 84, 175, 94, 197], OperandSize::Qword)
}

fn vdivps_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VDIVPS, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM26)), operand3: Some(IndirectScaledDisplaced(RCX, Eight, 1869029172, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 44, 167, 94, 4, 205, 52, 31, 103, 111], OperandSize::Qword)
}

fn vdivps_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VDIVPS, operand1: Some(Direct(YMM11)), operand2: Some(Direct(YMM26)), operand3: Some(Indirect(RBX, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 113, 44, 180, 94, 27], OperandSize::Qword)
}

fn vdivps_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VDIVPS, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM4)), operand3: Some(Direct(ZMM5)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Up), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 92, 221, 94, 229], OperandSize::Dword)
}

fn vdivps_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VDIVPS, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM6)), operand3: Some(IndirectScaledDisplaced(EDX, Eight, 1689621794, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 76, 204, 94, 20, 213, 34, 149, 181, 100], OperandSize::Dword)
}

fn vdivps_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VDIVPS, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM5)), operand3: Some(IndirectScaledDisplaced(EAX, Four, 352775967, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 241, 84, 220, 94, 44, 133, 31, 239, 6, 21], OperandSize::Dword)
}

fn vdivps_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VDIVPS, operand1: Some(Direct(ZMM18)), operand2: Some(Direct(ZMM25)), operand3: Some(Direct(ZMM27)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Down), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 129, 52, 183, 94, 211], OperandSize::Qword)
}

fn vdivps_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VDIVPS, operand1: Some(Direct(ZMM30)), operand2: Some(Direct(ZMM7)), operand3: Some(IndirectScaledDisplaced(RCX, Four, 926443843, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 97, 68, 205, 94, 52, 141, 67, 105, 56, 55], OperandSize::Qword)
}

fn vdivps_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VDIVPS, operand1: Some(Direct(ZMM21)), operand2: Some(Direct(ZMM23)), operand3: Some(IndirectDisplaced(RSI, 956030248, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 225, 68, 211, 94, 174, 40, 221, 251, 56], OperandSize::Qword)
}

