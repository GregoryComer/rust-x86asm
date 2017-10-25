use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vrangepd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VRANGEPD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM4)), operand4: Some(Literal8(69)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 243, 245, 141, 80, 220, 69], OperandSize::Dword)
}

fn vrangepd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VRANGEPD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledDisplaced(EBX, Four, 797033837, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(83)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 243, 213, 141, 80, 60, 157, 109, 197, 129, 47, 83], OperandSize::Dword)
}

fn vrangepd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VRANGEPD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledIndexed(ESI, EDX, Four, Some(OperandSize::Qword), None)), operand4: Some(Literal8(120)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 243, 221, 157, 80, 20, 150, 120], OperandSize::Dword)
}

fn vrangepd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VRANGEPD, operand1: Some(Direct(XMM29)), operand2: Some(Direct(XMM17)), operand3: Some(Direct(XMM2)), operand4: Some(Literal8(2)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 99, 245, 134, 80, 234, 2], OperandSize::Qword)
}

fn vrangepd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VRANGEPD, operand1: Some(Direct(XMM23)), operand2: Some(Direct(XMM23)), operand3: Some(Indirect(RAX, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(105)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 227, 197, 129, 80, 56, 105], OperandSize::Qword)
}

fn vrangepd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VRANGEPD, operand1: Some(Direct(XMM22)), operand2: Some(Direct(XMM25)), operand3: Some(IndirectScaledDisplaced(RDI, Eight, 127711367, Some(OperandSize::Qword), None)), operand4: Some(Literal8(0)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 227, 181, 151, 80, 52, 253, 135, 184, 156, 7, 0], OperandSize::Qword)
}

fn vrangepd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VRANGEPD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(YMM3)), operand4: Some(Literal8(92)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 243, 213, 175, 80, 251, 92], OperandSize::Dword)
}

fn vrangepd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VRANGEPD, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectScaledDisplaced(ESI, Four, 1670419171, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(17)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 243, 221, 172, 80, 4, 181, 227, 146, 144, 99, 17], OperandSize::Dword)
}

fn vrangepd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VRANGEPD, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectScaledIndexed(EDX, EAX, Four, Some(OperandSize::Qword), None)), operand4: Some(Literal8(91)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 243, 213, 191, 80, 4, 130, 91], OperandSize::Dword)
}

fn vrangepd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VRANGEPD, operand1: Some(Direct(YMM31)), operand2: Some(Direct(YMM9)), operand3: Some(Direct(YMM25)), operand4: Some(Literal8(39)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 3, 181, 173, 80, 249, 39], OperandSize::Qword)
}

fn vrangepd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VRANGEPD, operand1: Some(Direct(YMM12)), operand2: Some(Direct(YMM17)), operand3: Some(Indirect(RAX, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(25)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 115, 245, 165, 80, 32, 25], OperandSize::Qword)
}

fn vrangepd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VRANGEPD, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectDisplaced(RSI, 1748536558, Some(OperandSize::Qword), None)), operand4: Some(Literal8(58)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 243, 213, 189, 80, 158, 238, 140, 56, 104, 58], OperandSize::Qword)
}

fn vrangepd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VRANGEPD, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM6)), operand3: Some(Direct(ZMM7)), operand4: Some(Literal8(51)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K1), broadcast: None }, &[98, 243, 205, 153, 80, 247, 51], OperandSize::Dword)
}

fn vrangepd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VRANGEPD, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM5)), operand3: Some(IndirectScaledIndexedDisplaced(EBX, EBX, Two, 2035016992, Some(OperandSize::Zmmword), None)), operand4: Some(Literal8(39)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 243, 213, 206, 80, 132, 91, 32, 229, 75, 121, 39], OperandSize::Dword)
}

fn vrangepd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VRANGEPD, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM7)), operand3: Some(IndirectDisplaced(EBX, 404007606, Some(OperandSize::Qword), None)), operand4: Some(Literal8(26)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 243, 197, 221, 80, 131, 182, 170, 20, 24, 26], OperandSize::Dword)
}

fn vrangepd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VRANGEPD, operand1: Some(Direct(ZMM16)), operand2: Some(Direct(ZMM24)), operand3: Some(Direct(ZMM3)), operand4: Some(Literal8(103)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K1), broadcast: None }, &[98, 227, 189, 145, 80, 195, 103], OperandSize::Qword)
}

fn vrangepd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VRANGEPD, operand1: Some(Direct(ZMM31)), operand2: Some(Direct(ZMM22)), operand3: Some(IndirectScaledIndexedDisplaced(RSI, RCX, Two, 2013018719, Some(OperandSize::Zmmword), None)), operand4: Some(Literal8(41)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 99, 205, 198, 80, 188, 78, 95, 58, 252, 119, 41], OperandSize::Qword)
}

fn vrangepd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VRANGEPD, operand1: Some(Direct(ZMM30)), operand2: Some(Direct(ZMM24)), operand3: Some(IndirectDisplaced(RAX, 1118729137, Some(OperandSize::Qword), None)), operand4: Some(Literal8(91)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 99, 189, 211, 80, 176, 177, 115, 174, 66, 91], OperandSize::Qword)
}

