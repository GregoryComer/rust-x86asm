use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vsubps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VSUBPS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 208, 92, 228], OperandSize::Dword)
}

fn vsubps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VSUBPS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectScaledIndexedDisplaced(EDI, EAX, Eight, 1187753513, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 200, 92, 188, 199, 41, 174, 203, 70], OperandSize::Dword)
}

fn vsubps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VSUBPS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 240, 92, 215], OperandSize::Qword)
}

fn vsubps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VSUBPS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledIndexedDisplaced(RAX, RCX, Four, 1940336321, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 208, 92, 180, 136, 193, 46, 167, 115], OperandSize::Qword)
}

fn vsubps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VSUBPS, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 244, 92, 224], OperandSize::Dword)
}

fn vsubps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VSUBPS, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectScaledIndexedDisplaced(EAX, EDI, Two, 986575461, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 228, 92, 180, 120, 101, 242, 205, 58], OperandSize::Dword)
}

fn vsubps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VSUBPS, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(YMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 204, 92, 208], OperandSize::Qword)
}

fn vsubps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VSUBPS, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectScaledIndexed(RDX, RDX, Four, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 244, 92, 12, 146], OperandSize::Qword)
}

fn vsubps_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VSUBPS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 124, 140, 92, 239], OperandSize::Dword)
}

fn vsubps_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VSUBPS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledIndexedDisplaced(ESI, EBX, Four, 1783049908, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 100, 139, 92, 172, 158, 180, 46, 71, 106], OperandSize::Dword)
}

fn vsubps_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VSUBPS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectDisplaced(EAX, 435368424, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 100, 155, 92, 152, 232, 49, 243, 25], OperandSize::Dword)
}

fn vsubps_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VSUBPS, operand1: Some(Direct(XMM12)), operand2: Some(Direct(XMM12)), operand3: Some(Direct(XMM10)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 81, 28, 137, 92, 226], OperandSize::Qword)
}

fn vsubps_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VSUBPS, operand1: Some(Direct(XMM13)), operand2: Some(Direct(XMM12)), operand3: Some(IndirectScaledDisplaced(RAX, Four, 881634041, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 113, 28, 143, 92, 44, 133, 249, 170, 140, 52], OperandSize::Qword)
}

fn vsubps_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VSUBPS, operand1: Some(Direct(XMM16)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledDisplaced(RBX, Two, 1142695216, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 225, 108, 158, 92, 4, 93, 48, 37, 28, 68], OperandSize::Qword)
}

fn vsubps_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VSUBPS, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(YMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 124, 172, 92, 229], OperandSize::Dword)
}

fn vsubps_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VSUBPS, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM5)), operand3: Some(Indirect(ESI, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 84, 174, 92, 62], OperandSize::Dword)
}

fn vsubps_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VSUBPS, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectScaledDisplaced(EAX, Eight, 941302179, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 68, 187, 92, 60, 197, 163, 33, 27, 56], OperandSize::Dword)
}

fn vsubps_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VSUBPS, operand1: Some(Direct(YMM29)), operand2: Some(Direct(YMM27)), operand3: Some(Direct(YMM29)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 1, 36, 162, 92, 237], OperandSize::Qword)
}

fn vsubps_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VSUBPS, operand1: Some(Direct(YMM18)), operand2: Some(Direct(YMM28)), operand3: Some(IndirectScaledIndexedDisplaced(RCX, RSI, Two, 139460469, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 225, 28, 164, 92, 148, 113, 117, 255, 79, 8], OperandSize::Qword)
}

fn vsubps_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VSUBPS, operand1: Some(Direct(YMM14)), operand2: Some(Direct(YMM16)), operand3: Some(IndirectScaledIndexedDisplaced(RBX, RSI, Two, 1156365411, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 113, 124, 180, 92, 180, 115, 99, 188, 236, 68], OperandSize::Qword)
}

fn vsubps_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VSUBPS, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM0)), operand3: Some(Direct(ZMM4)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Zero), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 124, 254, 92, 204], OperandSize::Dword)
}

fn vsubps_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VSUBPS, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM7)), operand3: Some(IndirectScaledIndexed(EAX, EBX, Four, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 68, 202, 92, 60, 152], OperandSize::Dword)
}

fn vsubps_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VSUBPS, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM1)), operand3: Some(IndirectScaledIndexedDisplaced(EDI, EDX, Two, 521759769, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 241, 116, 223, 92, 180, 87, 25, 108, 25, 31], OperandSize::Dword)
}

fn vsubps_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VSUBPS, operand1: Some(Direct(ZMM18)), operand2: Some(Direct(ZMM14)), operand3: Some(Direct(ZMM28)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Down), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 129, 12, 187, 92, 212], OperandSize::Qword)
}

fn vsubps_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VSUBPS, operand1: Some(Direct(ZMM28)), operand2: Some(Direct(ZMM1)), operand3: Some(Indirect(RAX, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 97, 116, 202, 92, 32], OperandSize::Qword)
}

fn vsubps_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VSUBPS, operand1: Some(Direct(ZMM29)), operand2: Some(Direct(ZMM4)), operand3: Some(IndirectScaledIndexedDisplaced(RSI, RDI, Two, 1574484985, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 97, 92, 221, 92, 172, 126, 249, 187, 216, 93], OperandSize::Qword)
}

