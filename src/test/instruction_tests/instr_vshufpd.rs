use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vshufpd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFPD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM4)), operand4: Some(Literal8(27)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 233, 198, 252, 27], OperandSize::Dword)
}

#[test]
fn vshufpd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFPD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectDisplaced(ESI, 641836861, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(53)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 241, 198, 174, 61, 167, 65, 38, 53], OperandSize::Dword)
}

#[test]
fn vshufpd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFPD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM7)), operand4: Some(Literal8(64)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 241, 198, 255, 64], OperandSize::Qword)
}

#[test]
fn vshufpd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFPD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectDisplaced(RDX, 2085696211, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(49)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 233, 198, 162, 211, 50, 81, 124, 49], OperandSize::Qword)
}

#[test]
fn vshufpd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFPD, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(YMM1)), operand4: Some(Literal8(83)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 198, 209, 83], OperandSize::Dword)
}

#[test]
fn vshufpd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFPD, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectScaledIndexedDisplaced(EAX, ESI, Eight, 2140513741, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(81)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 229, 198, 132, 240, 205, 165, 149, 127, 81], OperandSize::Dword)
}

#[test]
fn vshufpd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFPD, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(YMM3)), operand4: Some(Literal8(63)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 229, 198, 195, 63], OperandSize::Qword)
}

#[test]
fn vshufpd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFPD, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectScaledIndexed(RBX, RSI, Four, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(85)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 198, 12, 179, 85], OperandSize::Qword)
}

#[test]
fn vshufpd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFPD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM6)), operand4: Some(Literal8(38)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 221, 138, 198, 254, 38], OperandSize::Dword)
}

#[test]
fn vshufpd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFPD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledDisplaced(ESI, Four, 184692156, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(7)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 253, 143, 198, 28, 181, 188, 45, 2, 11, 7], OperandSize::Dword)
}

#[test]
fn vshufpd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFPD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectDisplaced(EDI, 1622218255, Some(OperandSize::Qword), None)), operand4: Some(Literal8(14)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 241, 213, 154, 198, 175, 15, 22, 177, 96, 14], OperandSize::Dword)
}

#[test]
fn vshufpd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFPD, operand1: Some(Direct(XMM30)), operand2: Some(Direct(XMM23)), operand3: Some(Direct(XMM9)), operand4: Some(Literal8(9)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 65, 197, 132, 198, 241, 9], OperandSize::Qword)
}

#[test]
fn vshufpd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFPD, operand1: Some(Direct(XMM13)), operand2: Some(Direct(XMM8)), operand3: Some(Indirect(RAX, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(85)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 113, 189, 139, 198, 40, 85], OperandSize::Qword)
}

#[test]
fn vshufpd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFPD, operand1: Some(Direct(XMM23)), operand2: Some(Direct(XMM18)), operand3: Some(IndirectScaledIndexedDisplaced(RDX, RDX, Eight, 888291455, Some(OperandSize::Qword), None)), operand4: Some(Literal8(13)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 225, 237, 147, 198, 188, 210, 127, 64, 242, 52, 13], OperandSize::Qword)
}

#[test]
fn vshufpd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFPD, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(YMM0)), operand4: Some(Literal8(7)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 213, 169, 198, 216, 7], OperandSize::Dword)
}

#[test]
fn vshufpd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFPD, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectDisplaced(ECX, 151512972, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(90)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 205, 175, 198, 177, 140, 231, 7, 9, 90], OperandSize::Dword)
}

#[test]
fn vshufpd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFPD, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectDisplaced(EDI, 527886676, Some(OperandSize::Qword), None)), operand4: Some(Literal8(66)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 205, 191, 198, 183, 84, 233, 118, 31, 66], OperandSize::Dword)
}

#[test]
fn vshufpd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFPD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(YMM30)), operand4: Some(Literal8(81)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 145, 213, 170, 198, 238, 81], OperandSize::Qword)
}

#[test]
fn vshufpd_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFPD, operand1: Some(Direct(YMM12)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectDisplaced(RDI, 1392403458, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(95)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 113, 197, 172, 198, 167, 2, 100, 254, 82, 95], OperandSize::Qword)
}

#[test]
fn vshufpd_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFPD, operand1: Some(Direct(YMM15)), operand2: Some(Direct(YMM20)), operand3: Some(IndirectScaledDisplaced(RBX, Four, 512106347, Some(OperandSize::Qword), None)), operand4: Some(Literal8(127)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 113, 221, 177, 198, 60, 157, 107, 31, 134, 30, 127], OperandSize::Qword)
}

#[test]
fn vshufpd_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFPD, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM4)), operand3: Some(Direct(ZMM3)), operand4: Some(Literal8(11)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 221, 203, 198, 227, 11], OperandSize::Dword)
}

#[test]
fn vshufpd_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFPD, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM6)), operand3: Some(IndirectDisplaced(ESI, 919750059, Some(OperandSize::Zmmword), None)), operand4: Some(Literal8(30)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 205, 204, 198, 142, 171, 69, 210, 54, 30], OperandSize::Dword)
}

#[test]
fn vshufpd_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFPD, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM0)), operand3: Some(Indirect(ECX, Some(OperandSize::Qword), None)), operand4: Some(Literal8(42)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 253, 219, 198, 1, 42], OperandSize::Dword)
}

#[test]
fn vshufpd_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFPD, operand1: Some(Direct(ZMM25)), operand2: Some(Direct(ZMM27)), operand3: Some(Direct(ZMM0)), operand4: Some(Literal8(71)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 97, 165, 199, 198, 200, 71], OperandSize::Qword)
}

#[test]
fn vshufpd_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFPD, operand1: Some(Direct(ZMM27)), operand2: Some(Direct(ZMM14)), operand3: Some(IndirectScaledIndexed(RDX, RSI, Two, Some(OperandSize::Zmmword), None)), operand4: Some(Literal8(52)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 97, 141, 203, 198, 28, 114, 52], OperandSize::Qword)
}

#[test]
fn vshufpd_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFPD, operand1: Some(Direct(ZMM23)), operand2: Some(Direct(ZMM21)), operand3: Some(IndirectDisplaced(RDI, 778884605, Some(OperandSize::Qword), None)), operand4: Some(Literal8(33)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 225, 213, 214, 198, 191, 253, 213, 108, 46, 33], OperandSize::Qword)
}

