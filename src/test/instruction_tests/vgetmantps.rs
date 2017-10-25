use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vgetmantps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETMANTPS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM3)), operand3: Some(Literal8(71)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 243, 125, 143, 38, 235, 71], OperandSize::Dword)
}

fn vgetmantps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETMANTPS, operand1: Some(Direct(XMM3)), operand2: Some(IndirectScaledDisplaced(ECX, Eight, 374028645, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(67)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 243, 125, 137, 38, 28, 205, 101, 57, 75, 22, 67], OperandSize::Dword)
}

fn vgetmantps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETMANTPS, operand1: Some(Direct(XMM2)), operand2: Some(IndirectDisplaced(EDI, 1482691638, Some(OperandSize::Dword), None)), operand3: Some(Literal8(49)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 243, 125, 159, 38, 151, 54, 20, 96, 88, 49], OperandSize::Dword)
}

fn vgetmantps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETMANTPS, operand1: Some(Direct(XMM31)), operand2: Some(Direct(XMM19)), operand3: Some(Literal8(0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 35, 125, 143, 38, 251, 0], OperandSize::Qword)
}

fn vgetmantps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETMANTPS, operand1: Some(Direct(XMM19)), operand2: Some(IndirectScaledDisplaced(RAX, Eight, 1582237653, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(65)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 227, 125, 139, 38, 28, 197, 213, 7, 79, 94, 65], OperandSize::Qword)
}

fn vgetmantps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETMANTPS, operand1: Some(Direct(XMM21)), operand2: Some(IndirectScaledIndexed(RCX, RCX, Four, Some(OperandSize::Dword), None)), operand3: Some(Literal8(75)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 227, 125, 159, 38, 44, 137, 75], OperandSize::Qword)
}

fn vgetmantps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETMANTPS, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM1)), operand3: Some(Literal8(55)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 243, 125, 169, 38, 217, 55], OperandSize::Dword)
}

fn vgetmantps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETMANTPS, operand1: Some(Direct(YMM5)), operand2: Some(IndirectDisplaced(EBX, 180798770, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(119)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 243, 125, 175, 38, 171, 50, 197, 198, 10, 119], OperandSize::Dword)
}

fn vgetmantps_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETMANTPS, operand1: Some(Direct(YMM7)), operand2: Some(IndirectDisplaced(ECX, 538178806, Some(OperandSize::Dword), None)), operand3: Some(Literal8(70)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 243, 125, 188, 38, 185, 246, 244, 19, 32, 70], OperandSize::Dword)
}

fn vgetmantps_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETMANTPS, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM20)), operand3: Some(Literal8(19)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 179, 125, 170, 38, 220, 19], OperandSize::Qword)
}

fn vgetmantps_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETMANTPS, operand1: Some(Direct(YMM3)), operand2: Some(IndirectScaledIndexed(RDX, RDX, Four, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(111)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 243, 125, 170, 38, 28, 146, 111], OperandSize::Qword)
}

fn vgetmantps_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETMANTPS, operand1: Some(Direct(YMM31)), operand2: Some(IndirectScaledIndexed(RBX, RSI, Two, Some(OperandSize::Dword), None)), operand3: Some(Literal8(22)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 99, 125, 189, 38, 60, 115, 22], OperandSize::Qword)
}

fn vgetmantps_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETMANTPS, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM0)), operand3: Some(Literal8(53)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K1), broadcast: None }, &[98, 243, 125, 153, 38, 232, 53], OperandSize::Dword)
}

fn vgetmantps_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETMANTPS, operand1: Some(Direct(ZMM3)), operand2: Some(IndirectScaledIndexed(EAX, ESI, Eight, Some(OperandSize::Zmmword), None)), operand3: Some(Literal8(96)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 243, 125, 203, 38, 28, 240, 96], OperandSize::Dword)
}

fn vgetmantps_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETMANTPS, operand1: Some(Direct(ZMM6)), operand2: Some(IndirectScaledDisplaced(ESI, Four, 1442648441, Some(OperandSize::Dword), None)), operand3: Some(Literal8(83)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 243, 125, 222, 38, 52, 181, 121, 17, 253, 85, 83], OperandSize::Dword)
}

fn vgetmantps_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETMANTPS, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM27)), operand3: Some(Literal8(121)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K6), broadcast: None }, &[98, 147, 125, 158, 38, 235, 121], OperandSize::Qword)
}

fn vgetmantps_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETMANTPS, operand1: Some(Direct(ZMM0)), operand2: Some(IndirectScaledIndexed(RBX, RCX, Eight, Some(OperandSize::Zmmword), None)), operand3: Some(Literal8(97)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 243, 125, 205, 38, 4, 203, 97], OperandSize::Qword)
}

fn vgetmantps_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETMANTPS, operand1: Some(Direct(ZMM6)), operand2: Some(IndirectScaledIndexed(RAX, RBX, Two, Some(OperandSize::Dword), None)), operand3: Some(Literal8(95)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 243, 125, 220, 38, 52, 88, 95], OperandSize::Qword)
}

