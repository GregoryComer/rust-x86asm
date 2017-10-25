use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vunpcklpd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKLPD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 241, 20, 199], OperandSize::Dword)
}

fn vunpcklpd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKLPD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledDisplaced(EBX, Eight, 1875073590, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 217, 20, 44, 221, 54, 90, 195, 111], OperandSize::Dword)
}

fn vunpcklpd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKLPD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 225, 20, 230], OperandSize::Qword)
}

fn vunpcklpd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKLPD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledDisplaced(RSI, Four, 1469702772, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 225, 20, 44, 181, 116, 226, 153, 87], OperandSize::Qword)
}

fn vunpcklpd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKLPD, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(YMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 229, 20, 215], OperandSize::Dword)
}

fn vunpcklpd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKLPD, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectScaledIndexed(ESI, ECX, Eight, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 20, 36, 206], OperandSize::Dword)
}

fn vunpcklpd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKLPD, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 237, 20, 224], OperandSize::Qword)
}

fn vunpcklpd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKLPD, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectScaledDisplaced(RCX, Eight, 1222028801, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 213, 20, 12, 205, 1, 174, 214, 72], OperandSize::Qword)
}

fn vunpcklpd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKLPD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 221, 137, 20, 202], OperandSize::Dword)
}

fn vunpcklpd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKLPD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectScaledDisplaced(EAX, Two, 2044721263, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 245, 141, 20, 52, 69, 111, 248, 223, 121], OperandSize::Dword)
}

fn vunpcklpd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKLPD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectDisplaced(EAX, 1990311558, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 241, 245, 156, 20, 152, 134, 190, 161, 118], OperandSize::Dword)
}

fn vunpcklpd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKLPD, operand1: Some(Direct(XMM30)), operand2: Some(Direct(XMM31)), operand3: Some(Direct(XMM16)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 33, 133, 130, 20, 240], OperandSize::Qword)
}

fn vunpcklpd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKLPD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectScaledDisplaced(RCX, Two, 1327637111, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 205, 141, 20, 20, 77, 119, 34, 34, 79], OperandSize::Qword)
}

fn vunpcklpd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKLPD, operand1: Some(Direct(XMM11)), operand2: Some(Direct(XMM21)), operand3: Some(IndirectDisplaced(RDX, 1894048548, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 113, 213, 150, 20, 154, 36, 227, 228, 112], OperandSize::Qword)
}

fn vunpcklpd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKLPD, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(YMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 253, 172, 20, 219], OperandSize::Dword)
}

fn vunpcklpd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKLPD, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectScaledIndexedDisplaced(EDX, ESI, Eight, 772294974, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 253, 174, 20, 164, 242, 62, 73, 8, 46], OperandSize::Dword)
}

fn vunpcklpd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKLPD, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM4)), operand3: Some(Indirect(EDI, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 221, 188, 20, 7], OperandSize::Dword)
}

fn vunpcklpd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKLPD, operand1: Some(Direct(YMM26)), operand2: Some(Direct(YMM13)), operand3: Some(Direct(YMM31)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 1, 149, 174, 20, 215], OperandSize::Qword)
}

fn vunpcklpd_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKLPD, operand1: Some(Direct(YMM29)), operand2: Some(Direct(YMM0)), operand3: Some(Indirect(RSI, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 97, 253, 169, 20, 46], OperandSize::Qword)
}

fn vunpcklpd_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKLPD, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM27)), operand3: Some(IndirectScaledDisplaced(RCX, Eight, 775300339, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 165, 177, 20, 52, 205, 243, 36, 54, 46], OperandSize::Qword)
}

fn vunpcklpd_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKLPD, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM6)), operand3: Some(Direct(ZMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 205, 203, 20, 238], OperandSize::Dword)
}

fn vunpcklpd_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKLPD, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM7)), operand3: Some(IndirectScaledDisplaced(ESI, Four, 907444570, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 197, 205, 20, 20, 181, 90, 129, 22, 54], OperandSize::Dword)
}

fn vunpcklpd_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKLPD, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM5)), operand3: Some(IndirectDisplaced(EBX, 967636821, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 213, 218, 20, 155, 85, 247, 172, 57], OperandSize::Dword)
}

fn vunpcklpd_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKLPD, operand1: Some(Direct(ZMM8)), operand2: Some(Direct(ZMM17)), operand3: Some(Direct(ZMM14)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 81, 245, 196, 20, 198], OperandSize::Qword)
}

fn vunpcklpd_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKLPD, operand1: Some(Direct(ZMM21)), operand2: Some(Direct(ZMM17)), operand3: Some(IndirectScaledIndexedDisplaced(RDX, RBX, Four, 456925745, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 225, 245, 199, 20, 172, 154, 49, 34, 60, 27], OperandSize::Qword)
}

fn vunpcklpd_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKLPD, operand1: Some(Direct(ZMM23)), operand2: Some(Direct(ZMM4)), operand3: Some(IndirectScaledIndexedDisplaced(RDI, RDI, Eight, 320477157, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 225, 221, 220, 20, 188, 255, 229, 23, 26, 19], OperandSize::Qword)
}

