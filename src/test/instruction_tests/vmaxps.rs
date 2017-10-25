use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vmaxps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VMAXPS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 208, 95, 219], OperandSize::Dword)
}

fn vmaxps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VMAXPS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledIndexed(EDX, ECX, Four, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 208, 95, 12, 138], OperandSize::Dword)
}

fn vmaxps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VMAXPS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 224, 95, 222], OperandSize::Qword)
}

fn vmaxps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VMAXPS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectScaledDisplaced(RSI, Eight, 1966587483, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 240, 95, 60, 245, 91, 190, 55, 117], OperandSize::Qword)
}

fn vmaxps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VMAXPS, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 220, 95, 254], OperandSize::Dword)
}

fn vmaxps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VMAXPS, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM3)), operand3: Some(Indirect(EDI, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 228, 95, 39], OperandSize::Dword)
}

fn vmaxps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VMAXPS, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(YMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 228, 95, 208], OperandSize::Qword)
}

fn vmaxps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VMAXPS, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectScaledDisplaced(RAX, Eight, 1209469693, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 236, 95, 52, 197, 253, 10, 23, 72], OperandSize::Qword)
}

fn vmaxps_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VMAXPS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 76, 142, 95, 195], OperandSize::Dword)
}

fn vmaxps_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VMAXPS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM2)), operand3: Some(Indirect(EBX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 108, 138, 95, 59], OperandSize::Dword)
}

fn vmaxps_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VMAXPS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledIndexedDisplaced(ESI, ECX, Four, 647408728, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 124, 153, 95, 148, 142, 88, 172, 150, 38], OperandSize::Dword)
}

fn vmaxps_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VMAXPS, operand1: Some(Direct(XMM10)), operand2: Some(Direct(XMM13)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 113, 20, 139, 95, 208], OperandSize::Qword)
}

fn vmaxps_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VMAXPS, operand1: Some(Direct(XMM31)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledIndexedDisplaced(RDX, RDI, Eight, 875490072, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 97, 92, 137, 95, 188, 250, 24, 235, 46, 52], OperandSize::Qword)
}

fn vmaxps_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VMAXPS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM21)), operand3: Some(Indirect(RDX, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 84, 146, 95, 10], OperandSize::Qword)
}

fn vmaxps_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VMAXPS, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 108, 173, 95, 243], OperandSize::Dword)
}

fn vmaxps_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VMAXPS, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectDisplaced(EBX, 825522777, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 108, 173, 95, 187, 89, 122, 52, 49], OperandSize::Dword)
}

fn vmaxps_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VMAXPS, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectScaledIndexedDisplaced(EBX, ECX, Two, 691755713, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 84, 188, 95, 140, 75, 193, 90, 59, 41], OperandSize::Dword)
}

fn vmaxps_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VMAXPS, operand1: Some(Direct(YMM24)), operand2: Some(Direct(YMM26)), operand3: Some(Direct(YMM11)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 65, 44, 161, 95, 195], OperandSize::Qword)
}

fn vmaxps_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VMAXPS, operand1: Some(Direct(YMM22)), operand2: Some(Direct(YMM20)), operand3: Some(IndirectScaledIndexedDisplaced(RDX, RBX, Two, 1906544548, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 225, 92, 167, 95, 180, 90, 164, 143, 163, 113], OperandSize::Qword)
}

fn vmaxps_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VMAXPS, operand1: Some(Direct(YMM13)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectDisplaced(RDI, 340816694, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 113, 124, 186, 95, 175, 54, 115, 80, 20], OperandSize::Qword)
}

fn vmaxps_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VMAXPS, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM1)), operand3: Some(Direct(ZMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 116, 159, 95, 199], OperandSize::Dword)
}

fn vmaxps_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VMAXPS, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM0)), operand3: Some(IndirectScaledIndexed(EBX, ESI, Four, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 124, 206, 95, 44, 179], OperandSize::Dword)
}

fn vmaxps_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VMAXPS, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM4)), operand3: Some(IndirectScaledIndexed(EDI, ECX, Four, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 241, 92, 222, 95, 28, 143], OperandSize::Dword)
}

fn vmaxps_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VMAXPS, operand1: Some(Direct(ZMM13)), operand2: Some(Direct(ZMM21)), operand3: Some(Direct(ZMM11)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K4), broadcast: None }, &[98, 81, 84, 148, 95, 235], OperandSize::Qword)
}

fn vmaxps_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VMAXPS, operand1: Some(Direct(ZMM11)), operand2: Some(Direct(ZMM19)), operand3: Some(IndirectDisplaced(RSI, 451925172, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 113, 100, 193, 95, 158, 180, 212, 239, 26], OperandSize::Qword)
}

fn vmaxps_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VMAXPS, operand1: Some(Direct(ZMM8)), operand2: Some(Direct(ZMM9)), operand3: Some(IndirectScaledIndexed(RCX, RDI, Eight, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 113, 52, 222, 95, 4, 249], OperandSize::Qword)
}

