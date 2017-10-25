use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vunpckhps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKHPS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 240, 21, 202], OperandSize::Dword)
}

fn vunpckhps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKHPS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM6)), operand3: Some(Indirect(EDI, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 200, 21, 39], OperandSize::Dword)
}

fn vunpckhps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKHPS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 240, 21, 251], OperandSize::Qword)
}

fn vunpckhps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKHPS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectDisplaced(RDI, 655743204, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 240, 21, 183, 228, 216, 21, 39], OperandSize::Qword)
}

fn vunpckhps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKHPS, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 220, 21, 229], OperandSize::Dword)
}

fn vunpckhps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKHPS, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM6)), operand3: Some(Indirect(EDI, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 204, 21, 7], OperandSize::Dword)
}

fn vunpckhps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKHPS, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 196, 21, 220], OperandSize::Qword)
}

fn vunpckhps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKHPS, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectDisplaced(RSI, 571467278, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 252, 21, 190, 14, 230, 15, 34], OperandSize::Qword)
}

fn vunpckhps_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKHPS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 116, 141, 21, 233], OperandSize::Dword)
}

fn vunpckhps_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKHPS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledIndexedDisplaced(EDI, EAX, Two, 991585887, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 92, 138, 21, 164, 71, 95, 102, 26, 59], OperandSize::Dword)
}

fn vunpckhps_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKHPS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledIndexedDisplaced(EDI, EDX, Eight, 1462759180, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 84, 154, 21, 172, 215, 12, 239, 47, 87], OperandSize::Dword)
}

fn vunpckhps_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKHPS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM20)), operand3: Some(Direct(XMM26)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 145, 92, 134, 21, 250], OperandSize::Qword)
}

fn vunpckhps_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKHPS, operand1: Some(Direct(XMM17)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledIndexedDisplaced(RDX, RBX, Eight, 1145843463, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 225, 124, 141, 21, 140, 218, 7, 47, 76, 68], OperandSize::Qword)
}

fn vunpckhps_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKHPS, operand1: Some(Direct(XMM11)), operand2: Some(Direct(XMM18)), operand3: Some(IndirectScaledDisplaced(RBX, Four, 851898330, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 113, 108, 147, 21, 28, 157, 218, 239, 198, 50], OperandSize::Qword)
}

fn vunpckhps_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKHPS, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 116, 169, 21, 226], OperandSize::Dword)
}

fn vunpckhps_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKHPS, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectDisplaced(EAX, 1888693620, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 76, 174, 21, 152, 116, 45, 147, 112], OperandSize::Dword)
}

fn vunpckhps_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKHPS, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM5)), operand3: Some(Indirect(ESI, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 84, 187, 21, 14], OperandSize::Dword)
}

fn vunpckhps_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKHPS, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM20)), operand3: Some(Direct(YMM21)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 177, 92, 167, 21, 213], OperandSize::Qword)
}

fn vunpckhps_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKHPS, operand1: Some(Direct(YMM16)), operand2: Some(Direct(YMM7)), operand3: Some(Indirect(RCX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 225, 68, 173, 21, 1], OperandSize::Qword)
}

fn vunpckhps_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKHPS, operand1: Some(Direct(YMM31)), operand2: Some(Direct(YMM26)), operand3: Some(Indirect(RSI, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 97, 44, 182, 21, 62], OperandSize::Qword)
}

fn vunpckhps_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKHPS, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM0)), operand3: Some(Direct(ZMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 124, 201, 21, 208], OperandSize::Dword)
}

fn vunpckhps_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKHPS, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM4)), operand3: Some(IndirectDisplaced(ESI, 1895943775, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 92, 207, 21, 174, 95, 206, 1, 113], OperandSize::Dword)
}

fn vunpckhps_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKHPS, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM2)), operand3: Some(IndirectScaledIndexedDisplaced(EDI, ESI, Four, 381745520, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 241, 108, 220, 21, 180, 183, 112, 249, 192, 22], OperandSize::Dword)
}

fn vunpckhps_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKHPS, operand1: Some(Direct(ZMM9)), operand2: Some(Direct(ZMM0)), operand3: Some(Direct(ZMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 113, 124, 206, 21, 200], OperandSize::Qword)
}

fn vunpckhps_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKHPS, operand1: Some(Direct(ZMM28)), operand2: Some(Direct(ZMM1)), operand3: Some(IndirectDisplaced(RDI, 1975045733, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 97, 116, 204, 21, 167, 101, 206, 184, 117], OperandSize::Qword)
}

fn vunpckhps_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKHPS, operand1: Some(Direct(ZMM25)), operand2: Some(Direct(ZMM6)), operand3: Some(Indirect(RBX, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 97, 76, 223, 21, 11], OperandSize::Qword)
}

