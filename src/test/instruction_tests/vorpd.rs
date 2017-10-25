use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vorpd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VORPD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 209, 86, 226], OperandSize::Dword)
}

fn vorpd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VORPD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM6)), operand3: Some(Indirect(EDI, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 201, 86, 15], OperandSize::Dword)
}

fn vorpd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VORPD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 217, 86, 246], OperandSize::Qword)
}

fn vorpd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VORPD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM3)), operand3: Some(Indirect(RSI, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 225, 86, 22], OperandSize::Qword)
}

fn vorpd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VORPD, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(YMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 229, 86, 200], OperandSize::Dword)
}

fn vorpd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VORPD, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectScaledDisplaced(EAX, Four, 1875111397, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 86, 20, 133, 229, 237, 195, 111], OperandSize::Dword)
}

fn vorpd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VORPD, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 237, 86, 195], OperandSize::Qword)
}

fn vorpd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VORPD, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectScaledDisplaced(RBX, Eight, 914852825, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 86, 52, 221, 217, 139, 135, 54], OperandSize::Qword)
}

fn vorpd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VORPD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 221, 141, 86, 236], OperandSize::Dword)
}

fn vorpd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VORPD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectScaledDisplaced(ESI, Four, 685541571, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 205, 141, 86, 12, 181, 195, 136, 220, 40], OperandSize::Dword)
}

fn vorpd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VORPD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM7)), operand3: Some(Indirect(ESI, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 241, 197, 154, 86, 22], OperandSize::Dword)
}

fn vorpd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VORPD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM31)), operand3: Some(Direct(XMM19)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 177, 133, 133, 86, 211], OperandSize::Qword)
}

fn vorpd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VORPD, operand1: Some(Direct(XMM28)), operand2: Some(Direct(XMM30)), operand3: Some(Indirect(RDI, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 97, 141, 133, 86, 39], OperandSize::Qword)
}

fn vorpd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VORPD, operand1: Some(Direct(XMM21)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectDisplaced(RBX, 1251523765, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 225, 253, 154, 86, 171, 181, 188, 152, 74], OperandSize::Qword)
}

fn vorpd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VORPD, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(YMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 253, 174, 86, 225], OperandSize::Dword)
}

fn vorpd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VORPD, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectDisplaced(EBX, 1589993306, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 213, 175, 86, 179, 90, 95, 197, 94], OperandSize::Dword)
}

fn vorpd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VORPD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectScaledIndexed(EDX, EDX, Eight, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 229, 186, 86, 44, 210], OperandSize::Dword)
}

fn vorpd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VORPD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM9)), operand3: Some(Direct(YMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 181, 173, 86, 251], OperandSize::Qword)
}

fn vorpd_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VORPD, operand1: Some(Direct(YMM12)), operand2: Some(Direct(YMM4)), operand3: Some(Indirect(RAX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 113, 221, 172, 86, 32], OperandSize::Qword)
}

fn vorpd_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VORPD, operand1: Some(Direct(YMM17)), operand2: Some(Direct(YMM11)), operand3: Some(IndirectScaledIndexedDisplaced(RCX, RAX, Four, 635147223, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 225, 165, 191, 86, 140, 129, 215, 147, 219, 37], OperandSize::Qword)
}

fn vorpd_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VORPD, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM7)), operand3: Some(Direct(ZMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 197, 207, 86, 229], OperandSize::Dword)
}

fn vorpd_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VORPD, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM7)), operand3: Some(IndirectScaledIndexed(ECX, EAX, Two, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 197, 203, 86, 28, 65], OperandSize::Dword)
}

fn vorpd_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VORPD, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM3)), operand3: Some(IndirectScaledDisplaced(ECX, Four, 936850213, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 229, 222, 86, 4, 141, 37, 51, 215, 55], OperandSize::Dword)
}

fn vorpd_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VORPD, operand1: Some(Direct(ZMM13)), operand2: Some(Direct(ZMM22)), operand3: Some(Direct(ZMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 113, 205, 194, 86, 238], OperandSize::Qword)
}

fn vorpd_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VORPD, operand1: Some(Direct(ZMM15)), operand2: Some(Direct(ZMM23)), operand3: Some(IndirectDisplaced(RCX, 415821908, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 113, 197, 194, 86, 185, 84, 240, 200, 24], OperandSize::Qword)
}

fn vorpd_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VORPD, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM20)), operand3: Some(IndirectScaledDisplaced(RCX, Eight, 1919345154, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 221, 212, 86, 36, 205, 2, 226, 102, 114], OperandSize::Qword)
}

