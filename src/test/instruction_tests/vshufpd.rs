use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vshufpd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFPD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM0)), operand4: Some(Literal8(54)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 217, 198, 208, 54], OperandSize::Dword)
}

fn vshufpd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFPD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectScaledIndexedDisplaced(EDI, EDX, Two, 1184338311, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(113)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 241, 198, 148, 87, 135, 145, 151, 70, 113], OperandSize::Dword)
}

fn vshufpd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFPD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM3)), operand4: Some(Literal8(123)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 198, 195, 123], OperandSize::Qword)
}

fn vshufpd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFPD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM2)), operand3: Some(Indirect(RAX, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(34)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 233, 198, 56, 34], OperandSize::Qword)
}

fn vshufpd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFPD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM3)), operand4: Some(Literal8(51)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 237, 198, 251, 51], OperandSize::Dword)
}

fn vshufpd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFPD, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectScaledDisplaced(EAX, Two, 796926871, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(64)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 221, 198, 36, 69, 151, 35, 128, 47, 64], OperandSize::Dword)
}

fn vshufpd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFPD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(YMM7)), operand4: Some(Literal8(57)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 198, 255, 57], OperandSize::Qword)
}

fn vshufpd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFPD, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectScaledDisplaced(RDI, Two, 150922570, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(105)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 221, 198, 20, 125, 74, 229, 254, 8, 105], OperandSize::Qword)
}

fn vshufpd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFPD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM2)), operand4: Some(Literal8(66)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 237, 137, 198, 210, 66], OperandSize::Dword)
}

fn vshufpd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFPD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectScaledIndexedDisplaced(ESI, EDI, Two, 643462563, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(117)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 205, 137, 198, 140, 126, 163, 117, 90, 38, 117], OperandSize::Dword)
}

fn vshufpd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFPD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM4)), operand3: Some(Indirect(EBX, Some(OperandSize::Qword), None)), operand4: Some(Literal8(109)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 241, 221, 156, 198, 27, 109], OperandSize::Dword)
}

fn vshufpd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFPD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM0)), operand4: Some(Literal8(86)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 213, 138, 198, 248, 86], OperandSize::Qword)
}

fn vshufpd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFPD, operand1: Some(Direct(XMM16)), operand2: Some(Direct(XMM31)), operand3: Some(IndirectScaledIndexed(RDX, RCX, Four, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(87)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 225, 133, 135, 198, 4, 138, 87], OperandSize::Qword)
}

fn vshufpd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFPD, operand1: Some(Direct(XMM27)), operand2: Some(Direct(XMM19)), operand3: Some(Indirect(RDX, Some(OperandSize::Qword), None)), operand4: Some(Literal8(78)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 97, 229, 150, 198, 26, 78], OperandSize::Qword)
}

fn vshufpd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFPD, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(YMM7)), operand4: Some(Literal8(82)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 213, 171, 198, 215, 82], OperandSize::Dword)
}

fn vshufpd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFPD, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectScaledIndexed(EDX, EDX, Two, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(25)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 205, 171, 198, 36, 82, 25], OperandSize::Dword)
}

fn vshufpd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFPD, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectScaledIndexed(ESI, EBX, Four, Some(OperandSize::Qword), None)), operand4: Some(Literal8(73)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 253, 186, 198, 28, 158, 73], OperandSize::Dword)
}

fn vshufpd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFPD, operand1: Some(Direct(YMM24)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(YMM24)), operand4: Some(Literal8(94)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 1, 253, 174, 198, 192, 94], OperandSize::Qword)
}

fn vshufpd_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFPD, operand1: Some(Direct(YMM27)), operand2: Some(Direct(YMM14)), operand3: Some(IndirectScaledDisplaced(RCX, Two, 1421913751, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(70)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 97, 141, 170, 198, 28, 77, 151, 174, 192, 84, 70], OperandSize::Qword)
}

fn vshufpd_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFPD, operand1: Some(Direct(YMM9)), operand2: Some(Direct(YMM1)), operand3: Some(Indirect(RBX, Some(OperandSize::Qword), None)), operand4: Some(Literal8(74)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 113, 245, 185, 198, 11, 74], OperandSize::Qword)
}

fn vshufpd_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFPD, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM7)), operand3: Some(Direct(ZMM1)), operand4: Some(Literal8(87)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 197, 201, 198, 225, 87], OperandSize::Dword)
}

fn vshufpd_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFPD, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM4)), operand3: Some(IndirectScaledIndexed(EDI, EBX, Two, Some(OperandSize::Zmmword), None)), operand4: Some(Literal8(35)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 221, 202, 198, 44, 95, 35], OperandSize::Dword)
}

fn vshufpd_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFPD, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM7)), operand3: Some(IndirectScaledDisplaced(ESI, Two, 1979943236, Some(OperandSize::Qword), None)), operand4: Some(Literal8(8)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 197, 219, 198, 44, 117, 68, 137, 3, 118, 8], OperandSize::Dword)
}

fn vshufpd_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFPD, operand1: Some(Direct(ZMM22)), operand2: Some(Direct(ZMM15)), operand3: Some(Direct(ZMM26)), operand4: Some(Literal8(61)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 129, 133, 205, 198, 242, 61], OperandSize::Qword)
}

fn vshufpd_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFPD, operand1: Some(Direct(ZMM19)), operand2: Some(Direct(ZMM8)), operand3: Some(IndirectScaledDisplaced(RCX, Two, 1189653937, Some(OperandSize::Zmmword), None)), operand4: Some(Literal8(76)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 225, 189, 202, 198, 28, 77, 177, 173, 232, 70, 76], OperandSize::Qword)
}

fn vshufpd_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFPD, operand1: Some(Direct(ZMM27)), operand2: Some(Direct(ZMM22)), operand3: Some(IndirectScaledIndexedDisplaced(RDX, RAX, Eight, 1082776280, Some(OperandSize::Qword), None)), operand4: Some(Literal8(104)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 97, 205, 213, 198, 156, 194, 216, 218, 137, 64, 104], OperandSize::Qword)
}

