use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vunpckhpd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKHPD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 217, 21, 236], OperandSize::Dword)
}

fn vunpckhpd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKHPD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM4)), operand3: Some(Indirect(ECX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 217, 21, 25], OperandSize::Dword)
}

fn vunpckhpd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKHPD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 233, 21, 246], OperandSize::Qword)
}

fn vunpckhpd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKHPD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM7)), operand3: Some(Indirect(RBX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 193, 21, 11], OperandSize::Qword)
}

fn vunpckhpd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKHPD, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 221, 21, 194], OperandSize::Dword)
}

fn vunpckhpd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKHPD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectScaledIndexed(EAX, EBX, Four, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 237, 21, 60, 152], OperandSize::Dword)
}

fn vunpckhpd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKHPD, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 221, 21, 231], OperandSize::Qword)
}

fn vunpckhpd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKHPD, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectScaledIndexed(RDX, RDI, Four, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 229, 21, 12, 186], OperandSize::Qword)
}

fn vunpckhpd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKHPD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 213, 138, 21, 198], OperandSize::Dword)
}

fn vunpckhpd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKHPD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledIndexed(ESI, EDX, Eight, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 229, 141, 21, 12, 214], OperandSize::Dword)
}

fn vunpckhpd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKHPD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledIndexed(EAX, ECX, Eight, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 241, 221, 158, 21, 12, 200], OperandSize::Dword)
}

fn vunpckhpd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKHPD, operand1: Some(Direct(XMM22)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 225, 237, 138, 21, 240], OperandSize::Qword)
}

fn vunpckhpd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKHPD, operand1: Some(Direct(XMM18)), operand2: Some(Direct(XMM15)), operand3: Some(IndirectDisplaced(RBX, 1807084360, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 225, 133, 143, 21, 147, 72, 235, 181, 107], OperandSize::Qword)
}

fn vunpckhpd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKHPD, operand1: Some(Direct(XMM20)), operand2: Some(Direct(XMM31)), operand3: Some(IndirectScaledIndexedDisplaced(RBX, RCX, Four, 733806277, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 225, 133, 145, 21, 164, 139, 197, 254, 188, 43], OperandSize::Qword)
}

fn vunpckhpd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKHPD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 237, 169, 21, 235], OperandSize::Dword)
}

fn vunpckhpd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKHPD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM6)), operand3: Some(Indirect(EAX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 205, 171, 21, 40], OperandSize::Dword)
}

fn vunpckhpd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKHPD, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectScaledDisplaced(EBX, Four, 2105473747, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 245, 185, 21, 52, 157, 211, 250, 126, 125], OperandSize::Dword)
}

fn vunpckhpd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKHPD, operand1: Some(Direct(YMM30)), operand2: Some(Direct(YMM13)), operand3: Some(Direct(YMM27)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 1, 149, 173, 21, 243], OperandSize::Qword)
}

fn vunpckhpd_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKHPD, operand1: Some(Direct(YMM23)), operand2: Some(Direct(YMM24)), operand3: Some(IndirectDisplaced(RCX, 924382906, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 225, 189, 165, 21, 185, 186, 246, 24, 55], OperandSize::Qword)
}

fn vunpckhpd_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKHPD, operand1: Some(Direct(YMM25)), operand2: Some(Direct(YMM31)), operand3: Some(IndirectScaledIndexed(RAX, RDI, Eight, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 97, 133, 177, 21, 12, 248], OperandSize::Qword)
}

fn vunpckhpd_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKHPD, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM4)), operand3: Some(Direct(ZMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 221, 206, 21, 194], OperandSize::Dword)
}

fn vunpckhpd_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKHPD, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM0)), operand3: Some(IndirectScaledDisplaced(ESI, Eight, 489704437, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 253, 201, 21, 4, 245, 245, 75, 48, 29], OperandSize::Dword)
}

fn vunpckhpd_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKHPD, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM6)), operand3: Some(Indirect(EBX, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 205, 223, 21, 27], OperandSize::Dword)
}

fn vunpckhpd_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKHPD, operand1: Some(Direct(ZMM20)), operand2: Some(Direct(ZMM21)), operand3: Some(Direct(ZMM8)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 193, 213, 196, 21, 224], OperandSize::Qword)
}

fn vunpckhpd_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKHPD, operand1: Some(Direct(ZMM16)), operand2: Some(Direct(ZMM22)), operand3: Some(IndirectScaledIndexed(RAX, RBX, Eight, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 225, 205, 197, 21, 4, 216], OperandSize::Qword)
}

fn vunpckhpd_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKHPD, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM15)), operand3: Some(IndirectScaledDisplaced(RCX, Eight, 1055878238, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 133, 218, 21, 44, 205, 94, 108, 239, 62], OperandSize::Qword)
}

