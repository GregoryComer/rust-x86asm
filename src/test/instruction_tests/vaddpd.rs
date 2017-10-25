use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vaddpd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDPD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 88, 218], OperandSize::Dword)
}

fn vaddpd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDPD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM4)), operand3: Some(Indirect(EDI, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 217, 88, 7], OperandSize::Dword)
}

fn vaddpd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDPD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 209, 88, 248], OperandSize::Qword)
}

fn vaddpd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDPD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledDisplaced(RDX, Four, 1367854334, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 225, 88, 20, 149, 254, 204, 135, 81], OperandSize::Qword)
}

fn vaddpd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDPD, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(YMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 88, 247], OperandSize::Dword)
}

fn vaddpd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDPD, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectScaledDisplaced(EAX, Four, 385613219, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 237, 88, 12, 133, 163, 253, 251, 22], OperandSize::Dword)
}

fn vaddpd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDPD, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 245, 88, 208], OperandSize::Qword)
}

fn vaddpd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDPD, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM1)), operand3: Some(Indirect(RDX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 245, 88, 26], OperandSize::Qword)
}

fn vaddpd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDPD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 245, 139, 88, 214], OperandSize::Dword)
}

fn vaddpd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDPD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM2)), operand3: Some(Indirect(ESI, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 237, 137, 88, 62], OperandSize::Dword)
}

fn vaddpd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDPD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectScaledIndexedDisplaced(ESI, EDX, Four, 825429884, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 241, 245, 153, 88, 156, 150, 124, 15, 51, 49], OperandSize::Dword)
}

fn vaddpd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDPD, operand1: Some(Direct(XMM23)), operand2: Some(Direct(XMM13)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 225, 149, 137, 88, 254], OperandSize::Qword)
}

fn vaddpd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDPD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM19)), operand3: Some(IndirectScaledIndexedDisplaced(RCX, RBX, Four, 1562138088, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 229, 132, 88, 132, 153, 232, 85, 28, 93], OperandSize::Qword)
}

fn vaddpd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDPD, operand1: Some(Direct(XMM10)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectDisplaced(RDI, 1815773788, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 113, 197, 153, 88, 151, 92, 130, 58, 108], OperandSize::Qword)
}

fn vaddpd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDPD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(YMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 205, 173, 88, 250], OperandSize::Dword)
}

fn vaddpd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDPD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectDisplaced(ESI, 1678451005, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 197, 171, 88, 174, 61, 33, 11, 100], OperandSize::Dword)
}

fn vaddpd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDPD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectScaledDisplaced(ESI, Eight, 2090844640, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 237, 187, 88, 60, 245, 224, 193, 159, 124], OperandSize::Dword)
}

fn vaddpd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDPD, operand1: Some(Direct(YMM8)), operand2: Some(Direct(YMM27)), operand3: Some(Direct(YMM15)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 81, 165, 163, 88, 199], OperandSize::Qword)
}

fn vaddpd_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDPD, operand1: Some(Direct(YMM24)), operand2: Some(Direct(YMM18)), operand3: Some(Indirect(RDI, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 97, 237, 162, 88, 7], OperandSize::Qword)
}

fn vaddpd_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDPD, operand1: Some(Direct(YMM22)), operand2: Some(Direct(YMM31)), operand3: Some(IndirectDisplaced(RCX, 1277227436, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 225, 133, 181, 88, 177, 172, 241, 32, 76], OperandSize::Qword)
}

fn vaddpd_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDPD, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM1)), operand3: Some(Direct(ZMM6)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Up), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 245, 220, 88, 254], OperandSize::Dword)
}

fn vaddpd_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDPD, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM1)), operand3: Some(IndirectScaledIndexedDisplaced(EAX, ESI, Two, 1832389359, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 245, 207, 88, 148, 112, 239, 10, 56, 109], OperandSize::Dword)
}

fn vaddpd_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDPD, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM7)), operand3: Some(IndirectScaledIndexedDisplaced(ECX, ECX, Four, 1084654857, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 197, 222, 88, 172, 137, 9, 133, 166, 64], OperandSize::Dword)
}

fn vaddpd_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDPD, operand1: Some(Direct(ZMM26)), operand2: Some(Direct(ZMM20)), operand3: Some(Direct(ZMM0)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Zero), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 97, 221, 243, 88, 208], OperandSize::Qword)
}

fn vaddpd_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDPD, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM17)), operand3: Some(IndirectDisplaced(RSI, 1141847209, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 245, 193, 88, 134, 169, 52, 15, 68], OperandSize::Qword)
}

fn vaddpd_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDPD, operand1: Some(Direct(ZMM26)), operand2: Some(Direct(ZMM5)), operand3: Some(IndirectScaledDisplaced(RBX, Four, 1050177048, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 97, 213, 222, 88, 20, 157, 24, 110, 152, 62], OperandSize::Qword)
}

