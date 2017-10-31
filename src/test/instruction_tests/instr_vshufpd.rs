use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vshufpd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFPD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM4)), operand4: Some(Literal8(85)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 193, 198, 220, 85], OperandSize::Dword)
}

#[test]
fn vshufpd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFPD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledDisplaced(EAX, Eight, 1046091243, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(30)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 198, 20, 197, 235, 21, 90, 62, 30], OperandSize::Dword)
}

#[test]
fn vshufpd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFPD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM0)), operand4: Some(Literal8(91)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 233, 198, 248, 91], OperandSize::Qword)
}

#[test]
fn vshufpd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFPD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectDisplaced(RAX, 956289631, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(123)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 193, 198, 128, 95, 210, 255, 56, 123], OperandSize::Qword)
}

#[test]
fn vshufpd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFPD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(YMM1)), operand4: Some(Literal8(65)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 213, 198, 233, 65], OperandSize::Dword)
}

#[test]
fn vshufpd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFPD, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectScaledIndexedDisplaced(EDI, EDI, Eight, 1229681107, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(93)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 245, 198, 132, 255, 211, 113, 75, 73, 93], OperandSize::Dword)
}

#[test]
fn vshufpd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFPD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(YMM4)), operand4: Some(Literal8(10)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 205, 198, 236, 10], OperandSize::Qword)
}

#[test]
fn vshufpd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFPD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectScaledIndexedDisplaced(RBX, RDX, Two, 1583237196, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(37)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 213, 198, 172, 83, 76, 72, 94, 94, 37], OperandSize::Qword)
}

#[test]
fn vshufpd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFPD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM1)), operand4: Some(Literal8(94)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 229, 141, 198, 233, 94], OperandSize::Dword)
}

#[test]
fn vshufpd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFPD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectDisplaced(EDX, 1319669097, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(38)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 205, 143, 198, 146, 105, 141, 168, 78, 38], OperandSize::Dword)
}

#[test]
fn vshufpd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFPD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM5)), operand3: Some(Indirect(EDX, Some(OperandSize::Qword), None)), operand4: Some(Literal8(64)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 241, 213, 153, 198, 26, 64], OperandSize::Dword)
}

#[test]
fn vshufpd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFPD, operand1: Some(Direct(XMM22)), operand2: Some(Direct(XMM19)), operand3: Some(Direct(XMM18)), operand4: Some(Literal8(107)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 161, 229, 134, 198, 242, 107], OperandSize::Qword)
}

#[test]
fn vshufpd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFPD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM16)), operand3: Some(IndirectScaledDisplaced(RBX, Four, 421087180, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(25)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 253, 132, 198, 60, 157, 204, 71, 25, 25, 25], OperandSize::Qword)
}

#[test]
fn vshufpd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFPD, operand1: Some(Direct(XMM31)), operand2: Some(Direct(XMM4)), operand3: Some(Indirect(RDI, Some(OperandSize::Qword), None)), operand4: Some(Literal8(89)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 97, 221, 153, 198, 63, 89], OperandSize::Qword)
}

#[test]
fn vshufpd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFPD, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM1)), operand4: Some(Literal8(116)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 197, 170, 198, 193, 116], OperandSize::Dword)
}

#[test]
fn vshufpd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFPD, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectScaledDisplaced(ECX, Four, 52128410, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(49)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 213, 175, 198, 4, 141, 154, 106, 27, 3, 49], OperandSize::Dword)
}

#[test]
fn vshufpd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFPD, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectScaledDisplaced(ESI, Two, 1352686089, Some(OperandSize::Qword), None)), operand4: Some(Literal8(72)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 221, 190, 198, 20, 117, 9, 90, 160, 80, 72], OperandSize::Dword)
}

#[test]
fn vshufpd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFPD, operand1: Some(Direct(YMM10)), operand2: Some(Direct(YMM23)), operand3: Some(Direct(YMM6)), operand4: Some(Literal8(99)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 113, 197, 167, 198, 214, 99], OperandSize::Qword)
}

#[test]
fn vshufpd_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFPD, operand1: Some(Direct(YMM21)), operand2: Some(Direct(YMM9)), operand3: Some(Indirect(RSI, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(3)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 225, 181, 173, 198, 46, 3], OperandSize::Qword)
}

#[test]
fn vshufpd_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFPD, operand1: Some(Direct(YMM24)), operand2: Some(Direct(YMM17)), operand3: Some(Indirect(RDI, Some(OperandSize::Qword), None)), operand4: Some(Literal8(95)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 97, 245, 183, 198, 7, 95], OperandSize::Qword)
}

#[test]
fn vshufpd_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFPD, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM6)), operand3: Some(Direct(ZMM2)), operand4: Some(Literal8(58)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 205, 206, 198, 210, 58], OperandSize::Dword)
}

#[test]
fn vshufpd_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFPD, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM6)), operand3: Some(Indirect(ESI, Some(OperandSize::Zmmword), None)), operand4: Some(Literal8(123)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 205, 207, 198, 46, 123], OperandSize::Dword)
}

#[test]
fn vshufpd_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFPD, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM7)), operand3: Some(IndirectScaledIndexedDisplaced(EDX, EAX, Four, 58988792, Some(OperandSize::Qword), None)), operand4: Some(Literal8(80)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 197, 219, 198, 172, 130, 248, 24, 132, 3, 80], OperandSize::Dword)
}

#[test]
fn vshufpd_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFPD, operand1: Some(Direct(ZMM11)), operand2: Some(Direct(ZMM13)), operand3: Some(Direct(ZMM11)), operand4: Some(Literal8(118)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 81, 149, 202, 198, 219, 118], OperandSize::Qword)
}

#[test]
fn vshufpd_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFPD, operand1: Some(Direct(ZMM13)), operand2: Some(Direct(ZMM29)), operand3: Some(IndirectDisplaced(RBX, 64382792, Some(OperandSize::Zmmword), None)), operand4: Some(Literal8(87)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 113, 149, 198, 198, 171, 72, 103, 214, 3, 87], OperandSize::Qword)
}

#[test]
fn vshufpd_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFPD, operand1: Some(Direct(ZMM10)), operand2: Some(Direct(ZMM3)), operand3: Some(IndirectDisplaced(RAX, 2011542412, Some(OperandSize::Qword), None)), operand4: Some(Literal8(118)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 113, 229, 223, 198, 144, 140, 179, 229, 119, 118], OperandSize::Qword)
}

