use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vreducepd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VREDUCEPD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM1)), operand3: Some(Literal8(111)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 243, 253, 140, 86, 201, 111], OperandSize::Dword)
}

#[test]
fn vreducepd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VREDUCEPD, operand1: Some(Direct(XMM4)), operand2: Some(IndirectDisplaced(ECX, 871575647, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(93)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 243, 253, 139, 86, 161, 95, 48, 243, 51, 93], OperandSize::Dword)
}

#[test]
fn vreducepd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VREDUCEPD, operand1: Some(Direct(XMM0)), operand2: Some(IndirectScaledDisplaced(EDI, Eight, 501717306, Some(OperandSize::Qword), None)), operand3: Some(Literal8(115)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 243, 253, 157, 86, 4, 253, 58, 153, 231, 29, 115], OperandSize::Dword)
}

#[test]
fn vreducepd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VREDUCEPD, operand1: Some(Direct(XMM23)), operand2: Some(Direct(XMM20)), operand3: Some(Literal8(11)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 163, 253, 139, 86, 252, 11], OperandSize::Qword)
}

#[test]
fn vreducepd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VREDUCEPD, operand1: Some(Direct(XMM12)), operand2: Some(IndirectDisplaced(RBX, 810138733, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(94)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 115, 253, 140, 86, 163, 109, 188, 73, 48, 94], OperandSize::Qword)
}

#[test]
fn vreducepd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VREDUCEPD, operand1: Some(Direct(XMM17)), operand2: Some(IndirectScaledIndexedDisplaced(RCX, RBX, Eight, 1975120149, Some(OperandSize::Qword), None)), operand3: Some(Literal8(41)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 227, 253, 159, 86, 140, 217, 21, 241, 185, 117, 41], OperandSize::Qword)
}

#[test]
fn vreducepd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VREDUCEPD, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM5)), operand3: Some(Literal8(7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 243, 253, 172, 86, 221, 7], OperandSize::Dword)
}

#[test]
fn vreducepd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VREDUCEPD, operand1: Some(Direct(YMM2)), operand2: Some(IndirectScaledDisplaced(EAX, Eight, 1951674136, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(21)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 243, 253, 172, 86, 20, 197, 24, 47, 84, 116, 21], OperandSize::Dword)
}

#[test]
fn vreducepd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VREDUCEPD, operand1: Some(Direct(YMM3)), operand2: Some(IndirectScaledDisplaced(ESI, Two, 640269370, Some(OperandSize::Qword), None)), operand3: Some(Literal8(112)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 243, 253, 186, 86, 28, 117, 58, 188, 41, 38, 112], OperandSize::Dword)
}

#[test]
fn vreducepd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VREDUCEPD, operand1: Some(Direct(YMM25)), operand2: Some(Direct(YMM29)), operand3: Some(Literal8(91)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 3, 253, 172, 86, 205, 91], OperandSize::Qword)
}

#[test]
fn vreducepd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VREDUCEPD, operand1: Some(Direct(YMM20)), operand2: Some(Indirect(RDX, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(84)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 227, 253, 175, 86, 34, 84], OperandSize::Qword)
}

#[test]
fn vreducepd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VREDUCEPD, operand1: Some(Direct(YMM26)), operand2: Some(Indirect(RCX, Some(OperandSize::Qword), None)), operand3: Some(Literal8(53)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 99, 253, 188, 86, 17, 53], OperandSize::Qword)
}

#[test]
fn vreducepd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VREDUCEPD, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM3)), operand3: Some(Literal8(69)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K6), broadcast: None }, &[98, 243, 253, 158, 86, 251, 69], OperandSize::Dword)
}

#[test]
fn vreducepd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VREDUCEPD, operand1: Some(Direct(ZMM7)), operand2: Some(IndirectDisplaced(EDX, 1599210436, Some(OperandSize::Zmmword), None)), operand3: Some(Literal8(61)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 243, 253, 201, 86, 186, 196, 3, 82, 95, 61], OperandSize::Dword)
}

#[test]
fn vreducepd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VREDUCEPD, operand1: Some(Direct(ZMM2)), operand2: Some(IndirectScaledIndexedDisplaced(EDI, EDI, Eight, 1567849217, Some(OperandSize::Qword), None)), operand3: Some(Literal8(6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 243, 253, 217, 86, 148, 255, 1, 123, 115, 93, 6], OperandSize::Dword)
}

#[test]
fn vreducepd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VREDUCEPD, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM26)), operand3: Some(Literal8(85)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K4), broadcast: None }, &[98, 147, 253, 156, 86, 250, 85], OperandSize::Qword)
}

#[test]
fn vreducepd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VREDUCEPD, operand1: Some(Direct(ZMM22)), operand2: Some(IndirectScaledDisplaced(RCX, Four, 314662139, Some(OperandSize::Zmmword), None)), operand3: Some(Literal8(119)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 227, 253, 204, 86, 52, 141, 251, 92, 193, 18, 119], OperandSize::Qword)
}

#[test]
fn vreducepd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VREDUCEPD, operand1: Some(Direct(ZMM31)), operand2: Some(IndirectScaledIndexed(RDI, RDX, Four, Some(OperandSize::Qword), None)), operand3: Some(Literal8(98)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 99, 253, 220, 86, 60, 151, 98], OperandSize::Qword)
}

