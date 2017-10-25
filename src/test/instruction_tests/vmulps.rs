use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vmulps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VMULPS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 89, 216], OperandSize::Dword)
}

fn vmulps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VMULPS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledDisplaced(EDX, Two, 669705048, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 192, 89, 60, 85, 88, 227, 234, 39], OperandSize::Dword)
}

fn vmulps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VMULPS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 240, 89, 202], OperandSize::Qword)
}

fn vmulps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VMULPS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectScaledIndexed(RBX, RBX, Four, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 240, 89, 36, 155], OperandSize::Qword)
}

fn vmulps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VMULPS, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(YMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 252, 89, 206], OperandSize::Dword)
}

fn vmulps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VMULPS, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM2)), operand3: Some(Indirect(EAX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 236, 89, 40], OperandSize::Dword)
}

fn vmulps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VMULPS, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 244, 89, 251], OperandSize::Qword)
}

fn vmulps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VMULPS, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM5)), operand3: Some(Indirect(RCX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 212, 89, 17], OperandSize::Qword)
}

fn vmulps_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VMULPS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 68, 142, 89, 214], OperandSize::Dword)
}

fn vmulps_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VMULPS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectDisplaced(ECX, 1475791404, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 116, 143, 89, 185, 44, 202, 246, 87], OperandSize::Dword)
}

fn vmulps_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VMULPS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledIndexedDisplaced(EAX, EDX, Eight, 1825291349, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 124, 159, 89, 132, 208, 85, 188, 203, 108], OperandSize::Dword)
}

fn vmulps_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VMULPS, operand1: Some(Direct(XMM11)), operand2: Some(Direct(XMM24)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 113, 60, 129, 89, 216], OperandSize::Qword)
}

fn vmulps_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VMULPS, operand1: Some(Direct(XMM21)), operand2: Some(Direct(XMM13)), operand3: Some(IndirectScaledDisplaced(RDI, Four, 2044714084, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 225, 20, 140, 89, 44, 189, 100, 220, 223, 121], OperandSize::Qword)
}

fn vmulps_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VMULPS, operand1: Some(Direct(XMM10)), operand2: Some(Direct(XMM11)), operand3: Some(IndirectScaledIndexedDisplaced(RAX, RDX, Two, 2116661218, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 113, 36, 153, 89, 148, 80, 226, 175, 41, 126], OperandSize::Qword)
}

fn vmulps_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VMULPS, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(YMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 100, 174, 89, 223], OperandSize::Dword)
}

fn vmulps_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VMULPS, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectScaledIndexed(EAX, EBX, Eight, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 124, 175, 89, 60, 216], OperandSize::Dword)
}

fn vmulps_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VMULPS, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectScaledIndexedDisplaced(EAX, EAX, Four, 1418066906, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 116, 190, 89, 132, 128, 218, 251, 133, 84], OperandSize::Dword)
}

fn vmulps_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VMULPS, operand1: Some(Direct(YMM14)), operand2: Some(Direct(YMM16)), operand3: Some(Direct(YMM20)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 49, 124, 164, 89, 244], OperandSize::Qword)
}

fn vmulps_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VMULPS, operand1: Some(Direct(YMM21)), operand2: Some(Direct(YMM31)), operand3: Some(IndirectScaledIndexed(RAX, RSI, Four, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 225, 4, 164, 89, 44, 176], OperandSize::Qword)
}

fn vmulps_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VMULPS, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectDisplaced(RBX, 908725545, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 68, 191, 89, 147, 41, 13, 42, 54], OperandSize::Qword)
}

fn vmulps_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VMULPS, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM7)), operand3: Some(Direct(ZMM1)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Down), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 68, 186, 89, 249], OperandSize::Dword)
}

fn vmulps_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VMULPS, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM1)), operand3: Some(IndirectScaledIndexed(ECX, ECX, Eight, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 116, 204, 89, 20, 201], OperandSize::Dword)
}

fn vmulps_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VMULPS, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM3)), operand3: Some(IndirectScaledIndexed(EBX, ESI, Two, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 241, 100, 220, 89, 20, 115], OperandSize::Dword)
}

fn vmulps_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VMULPS, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM20)), operand3: Some(Direct(ZMM11)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Zero), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 209, 92, 242, 89, 211], OperandSize::Qword)
}

fn vmulps_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VMULPS, operand1: Some(Direct(ZMM26)), operand2: Some(Direct(ZMM25)), operand3: Some(IndirectDisplaced(RDX, 14020398, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 97, 52, 197, 89, 146, 46, 239, 213, 0], OperandSize::Qword)
}

fn vmulps_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VMULPS, operand1: Some(Direct(ZMM18)), operand2: Some(Direct(ZMM18)), operand3: Some(IndirectScaledIndexed(RBX, RDX, Two, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 225, 108, 210, 89, 20, 83], OperandSize::Qword)
}

