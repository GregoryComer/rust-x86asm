use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vandpd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDPD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 193, 84, 239], OperandSize::Dword)
}

fn vandpd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDPD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledDisplaced(EDI, Eight, 1182721905, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 217, 84, 52, 253, 113, 231, 126, 70], OperandSize::Dword)
}

fn vandpd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDPD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 241, 84, 209], OperandSize::Qword)
}

fn vandpd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDPD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledDisplaced(RAX, Two, 926970353, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 225, 84, 20, 69, 241, 113, 64, 55], OperandSize::Qword)
}

fn vandpd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDPD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 245, 84, 238], OperandSize::Dword)
}

fn vandpd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDPD, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectScaledIndexedDisplaced(EDI, EDX, Eight, 1316063625, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 84, 164, 215, 137, 137, 113, 78], OperandSize::Dword)
}

fn vandpd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDPD, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(YMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 84, 200], OperandSize::Qword)
}

fn vandpd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDPD, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectDisplaced(RDI, 1690412679, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 237, 84, 159, 135, 166, 193, 100], OperandSize::Qword)
}

fn vandpd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDPD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 237, 139, 84, 206], OperandSize::Dword)
}

fn vandpd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDPD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectDisplaced(ECX, 20964165, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 245, 137, 84, 185, 69, 227, 63, 1], OperandSize::Dword)
}

fn vandpd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDPD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledIndexedDisplaced(ESI, EDI, Four, 2118735305, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 241, 221, 158, 84, 172, 190, 201, 85, 73, 126], OperandSize::Dword)
}

fn vandpd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDPD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM12)), operand3: Some(Direct(XMM14)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 209, 157, 139, 84, 206], OperandSize::Qword)
}

fn vandpd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDPD, operand1: Some(Direct(XMM14)), operand2: Some(Direct(XMM23)), operand3: Some(IndirectScaledIndexed(RDX, RDX, Eight, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 113, 197, 135, 84, 52, 210], OperandSize::Qword)
}

fn vandpd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDPD, operand1: Some(Direct(XMM8)), operand2: Some(Direct(XMM15)), operand3: Some(IndirectScaledIndexedDisplaced(RDX, RDX, Four, 1850206319, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 113, 133, 158, 84, 132, 146, 111, 232, 71, 110], OperandSize::Qword)
}

fn vandpd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDPD, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 237, 169, 84, 241], OperandSize::Dword)
}

fn vandpd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDPD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectScaledDisplaced(EDX, Four, 40975800, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 229, 171, 84, 60, 149, 184, 61, 113, 2], OperandSize::Dword)
}

fn vandpd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDPD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectScaledIndexedDisplaced(ESI, EAX, Four, 215207622, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 197, 189, 84, 188, 134, 198, 206, 211, 12], OperandSize::Dword)
}

fn vandpd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDPD, operand1: Some(Direct(YMM21)), operand2: Some(Direct(YMM10)), operand3: Some(Direct(YMM11)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 193, 173, 172, 84, 235], OperandSize::Qword)
}

fn vandpd_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDPD, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM19)), operand3: Some(IndirectScaledIndexedDisplaced(RAX, RBX, Two, 1104373013, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 229, 165, 84, 156, 88, 21, 101, 211, 65], OperandSize::Qword)
}

fn vandpd_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDPD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM15)), operand3: Some(IndirectScaledIndexedDisplaced(RSI, RDX, Two, 1804138548, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 133, 186, 84, 188, 86, 52, 248, 136, 107], OperandSize::Qword)
}

fn vandpd_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDPD, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM4)), operand3: Some(Direct(ZMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 221, 202, 84, 253], OperandSize::Dword)
}

fn vandpd_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDPD, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM2)), operand3: Some(IndirectScaledIndexed(EAX, EDX, Eight, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 237, 206, 84, 20, 208], OperandSize::Dword)
}

fn vandpd_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDPD, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM6)), operand3: Some(IndirectScaledIndexedDisplaced(EDI, EAX, Four, 1469331943, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 205, 219, 84, 164, 135, 231, 57, 148, 87], OperandSize::Dword)
}

fn vandpd_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDPD, operand1: Some(Direct(ZMM18)), operand2: Some(Direct(ZMM29)), operand3: Some(Direct(ZMM19)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 161, 149, 194, 84, 211], OperandSize::Qword)
}

fn vandpd_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDPD, operand1: Some(Direct(ZMM31)), operand2: Some(Direct(ZMM24)), operand3: Some(IndirectScaledIndexedDisplaced(RAX, RSI, Four, 1888244086, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 97, 189, 199, 84, 188, 176, 118, 81, 140, 112], OperandSize::Qword)
}

fn vandpd_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDPD, operand1: Some(Direct(ZMM15)), operand2: Some(Direct(ZMM17)), operand3: Some(IndirectScaledIndexedDisplaced(RAX, RBX, Two, 1615396431, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 113, 245, 215, 84, 188, 88, 79, 254, 72, 96], OperandSize::Qword)
}

