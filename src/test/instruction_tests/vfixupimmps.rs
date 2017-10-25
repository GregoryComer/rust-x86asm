use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vfixupimmps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VFIXUPIMMPS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM0)), operand4: Some(Literal8(92)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 243, 93, 140, 84, 232, 92], OperandSize::Dword)
}

fn vfixupimmps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VFIXUPIMMPS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM1)), operand3: Some(Indirect(EBX, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(81)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 243, 117, 142, 84, 59, 81], OperandSize::Dword)
}

fn vfixupimmps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VFIXUPIMMPS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledIndexedDisplaced(ECX, EAX, Two, 2143510215, Some(OperandSize::Dword), None)), operand4: Some(Literal8(117)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 243, 109, 154, 84, 156, 65, 199, 94, 195, 127, 117], OperandSize::Dword)
}

fn vfixupimmps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VFIXUPIMMPS, operand1: Some(Direct(XMM16)), operand2: Some(Direct(XMM25)), operand3: Some(Direct(XMM18)), operand4: Some(Literal8(59)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 163, 53, 132, 84, 194, 59], OperandSize::Qword)
}

fn vfixupimmps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VFIXUPIMMPS, operand1: Some(Direct(XMM9)), operand2: Some(Direct(XMM25)), operand3: Some(IndirectScaledIndexed(RBX, RBX, Eight, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(56)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 115, 53, 129, 84, 12, 219, 56], OperandSize::Qword)
}

fn vfixupimmps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VFIXUPIMMPS, operand1: Some(Direct(XMM13)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectScaledDisplaced(RDI, Two, 802724717, Some(OperandSize::Dword), None)), operand4: Some(Literal8(105)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 115, 117, 157, 84, 44, 125, 109, 155, 216, 47, 105], OperandSize::Qword)
}

fn vfixupimmps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VFIXUPIMMPS, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM0)), operand4: Some(Literal8(65)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 243, 117, 170, 84, 232, 65], OperandSize::Dword)
}

fn vfixupimmps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VFIXUPIMMPS, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectScaledDisplaced(EDI, Four, 936103294, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(62)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 243, 77, 171, 84, 28, 189, 126, 205, 203, 55, 62], OperandSize::Dword)
}

fn vfixupimmps_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VFIXUPIMMPS, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectScaledIndexed(EDI, EAX, Eight, Some(OperandSize::Dword), None)), operand4: Some(Literal8(85)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 243, 69, 187, 84, 12, 199, 85], OperandSize::Dword)
}

fn vfixupimmps_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VFIXUPIMMPS, operand1: Some(Direct(YMM8)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM11)), operand4: Some(Literal8(39)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 83, 117, 170, 84, 195, 39], OperandSize::Qword)
}

fn vfixupimmps_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VFIXUPIMMPS, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM30)), operand3: Some(IndirectScaledIndexed(RDI, RDX, Two, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(92)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 243, 13, 165, 84, 4, 87, 92], OperandSize::Qword)
}

fn vfixupimmps_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VFIXUPIMMPS, operand1: Some(Direct(YMM30)), operand2: Some(Direct(YMM16)), operand3: Some(IndirectScaledIndexedDisplaced(RDI, RAX, Eight, 380604858, Some(OperandSize::Dword), None)), operand4: Some(Literal8(125)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 99, 125, 177, 84, 180, 199, 186, 145, 175, 22, 125], OperandSize::Qword)
}

fn vfixupimmps_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VFIXUPIMMPS, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM6)), operand3: Some(Direct(ZMM0)), operand4: Some(Literal8(81)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K5), broadcast: None }, &[98, 243, 77, 157, 84, 208, 81], OperandSize::Dword)
}

fn vfixupimmps_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VFIXUPIMMPS, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM7)), operand3: Some(IndirectScaledIndexedDisplaced(EAX, EDX, Eight, 1914456679, Some(OperandSize::Zmmword), None)), operand4: Some(Literal8(104)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 243, 69, 206, 84, 156, 208, 103, 74, 28, 114, 104], OperandSize::Dword)
}

fn vfixupimmps_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VFIXUPIMMPS, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM2)), operand3: Some(IndirectScaledDisplaced(EDX, Four, 1611574538, Some(OperandSize::Dword), None)), operand4: Some(Literal8(118)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 243, 109, 223, 84, 52, 149, 10, 173, 14, 96, 118], OperandSize::Dword)
}

fn vfixupimmps_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VFIXUPIMMPS, operand1: Some(Direct(ZMM23)), operand2: Some(Direct(ZMM26)), operand3: Some(Direct(ZMM23)), operand4: Some(Literal8(65)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K2), broadcast: None }, &[98, 163, 45, 146, 84, 255, 65], OperandSize::Qword)
}

fn vfixupimmps_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VFIXUPIMMPS, operand1: Some(Direct(ZMM23)), operand2: Some(Direct(ZMM25)), operand3: Some(IndirectDisplaced(RBX, 940273417, Some(OperandSize::Zmmword), None)), operand4: Some(Literal8(119)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 227, 53, 199, 84, 187, 9, 111, 11, 56, 119], OperandSize::Qword)
}

fn vfixupimmps_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VFIXUPIMMPS, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM2)), operand3: Some(IndirectDisplaced(RDI, 1822836095, Some(OperandSize::Dword), None)), operand4: Some(Literal8(72)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 243, 109, 223, 84, 175, 127, 69, 166, 108, 72], OperandSize::Qword)
}

