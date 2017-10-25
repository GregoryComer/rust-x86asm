use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vsqrtps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VSQRTPS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 81, 213], OperandSize::Dword)
}

fn vsqrtps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VSQRTPS, operand1: Some(Direct(XMM0)), operand2: Some(IndirectDisplaced(EAX, 535021258, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 81, 128, 202, 198, 227, 31], OperandSize::Dword)
}

fn vsqrtps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VSQRTPS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 81, 238], OperandSize::Qword)
}

fn vsqrtps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VSQRTPS, operand1: Some(Direct(XMM3)), operand2: Some(Indirect(RBX, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 81, 27], OperandSize::Qword)
}

fn vsqrtps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VSQRTPS, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 252, 81, 246], OperandSize::Dword)
}

fn vsqrtps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VSQRTPS, operand1: Some(Direct(YMM5)), operand2: Some(IndirectScaledIndexedDisplaced(ESI, EDX, Two, 17706981, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 252, 81, 172, 86, 229, 47, 14, 1], OperandSize::Dword)
}

fn vsqrtps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VSQRTPS, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 252, 81, 200], OperandSize::Qword)
}

fn vsqrtps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VSQRTPS, operand1: Some(Direct(YMM5)), operand2: Some(IndirectScaledIndexedDisplaced(RCX, RAX, Eight, 701831037, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 252, 81, 172, 193, 125, 23, 213, 41], OperandSize::Qword)
}

fn vsqrtps_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VSQRTPS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 124, 138, 81, 209], OperandSize::Dword)
}

fn vsqrtps_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VSQRTPS, operand1: Some(Direct(XMM4)), operand2: Some(IndirectScaledDisplaced(ESI, Four, 1869105385, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 124, 141, 81, 36, 181, 233, 72, 104, 111], OperandSize::Dword)
}

fn vsqrtps_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VSQRTPS, operand1: Some(Direct(XMM1)), operand2: Some(Indirect(ESI, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 124, 156, 81, 14], OperandSize::Dword)
}

fn vsqrtps_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VSQRTPS, operand1: Some(Direct(XMM14)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 113, 124, 142, 81, 240], OperandSize::Qword)
}

fn vsqrtps_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VSQRTPS, operand1: Some(Direct(XMM28)), operand2: Some(IndirectScaledIndexedDisplaced(RAX, RDX, Four, 1648283875, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 97, 124, 139, 81, 164, 144, 227, 208, 62, 98], OperandSize::Qword)
}

fn vsqrtps_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VSQRTPS, operand1: Some(Direct(XMM22)), operand2: Some(IndirectScaledDisplaced(RCX, Four, 1535420414, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 225, 124, 155, 81, 52, 141, 254, 167, 132, 91], OperandSize::Qword)
}

fn vsqrtps_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VSQRTPS, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 124, 175, 81, 250], OperandSize::Dword)
}

fn vsqrtps_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VSQRTPS, operand1: Some(Direct(YMM3)), operand2: Some(IndirectScaledIndexed(EAX, EBX, Two, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 124, 171, 81, 28, 88], OperandSize::Dword)
}

fn vsqrtps_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VSQRTPS, operand1: Some(Direct(YMM4)), operand2: Some(IndirectDisplaced(EBX, 1736248453, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 124, 191, 81, 163, 133, 12, 125, 103], OperandSize::Dword)
}

fn vsqrtps_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VSQRTPS, operand1: Some(Direct(YMM15)), operand2: Some(Direct(YMM9)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 81, 124, 175, 81, 249], OperandSize::Qword)
}

fn vsqrtps_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VSQRTPS, operand1: Some(Direct(YMM29)), operand2: Some(Indirect(RAX, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 97, 124, 174, 81, 40], OperandSize::Qword)
}

fn vsqrtps_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VSQRTPS, operand1: Some(Direct(YMM12)), operand2: Some(Indirect(RDI, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 113, 124, 187, 81, 39], OperandSize::Qword)
}

fn vsqrtps_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VSQRTPS, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM1)), operand3: None, operand4: None, lock: false, rounding_mode: Some(RoundingMode::Down), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 124, 187, 81, 249], OperandSize::Dword)
}

fn vsqrtps_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VSQRTPS, operand1: Some(Direct(ZMM0)), operand2: Some(IndirectScaledIndexedDisplaced(EAX, ESI, Four, 1445054322, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 124, 207, 81, 132, 176, 114, 199, 33, 86], OperandSize::Dword)
}

fn vsqrtps_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VSQRTPS, operand1: Some(Direct(ZMM1)), operand2: Some(Indirect(ESI, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 241, 124, 218, 81, 14], OperandSize::Dword)
}

fn vsqrtps_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VSQRTPS, operand1: Some(Direct(ZMM21)), operand2: Some(Direct(ZMM26)), operand3: None, operand4: None, lock: false, rounding_mode: Some(RoundingMode::Up), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 129, 124, 223, 81, 234], OperandSize::Qword)
}

fn vsqrtps_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VSQRTPS, operand1: Some(Direct(ZMM10)), operand2: Some(IndirectScaledDisplaced(RDX, Two, 30560114, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 113, 124, 205, 81, 20, 85, 114, 79, 210, 1], OperandSize::Qword)
}

fn vsqrtps_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VSQRTPS, operand1: Some(Direct(ZMM3)), operand2: Some(IndirectScaledIndexedDisplaced(RSI, RAX, Eight, 109785754, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 241, 124, 220, 81, 156, 198, 154, 50, 139, 6], OperandSize::Qword)
}

