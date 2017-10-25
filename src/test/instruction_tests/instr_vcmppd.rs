use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vcmppd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VCMPPD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM7)), operand4: Some(Literal8(12)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 233, 194, 199, 12], OperandSize::Dword)
}

#[test]
fn vcmppd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VCMPPD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectDisplaced(ECX, 2121381527, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(66)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 233, 194, 137, 151, 182, 113, 126, 66], OperandSize::Dword)
}

#[test]
fn vcmppd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VCMPPD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM5)), operand4: Some(Literal8(101)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 193, 194, 253, 101], OperandSize::Qword)
}

#[test]
fn vcmppd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VCMPPD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectDisplaced(RDI, 2050804899, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(50)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 241, 194, 167, 163, 204, 60, 122, 50], OperandSize::Qword)
}

#[test]
fn vcmppd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VCMPPD, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(YMM2)), operand4: Some(Literal8(98)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 229, 194, 202, 98], OperandSize::Dword)
}

#[test]
fn vcmppd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VCMPPD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectScaledIndexedDisplaced(EDI, EAX, Four, 553876216, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(53)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 245, 194, 172, 135, 248, 122, 3, 33, 53], OperandSize::Dword)
}

#[test]
fn vcmppd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VCMPPD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(YMM1)), operand4: Some(Literal8(116)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 229, 194, 233, 116], OperandSize::Qword)
}

#[test]
fn vcmppd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VCMPPD, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectScaledDisplaced(RBX, Two, 2042836136, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(113)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 205, 194, 4, 93, 168, 52, 195, 121, 113], OperandSize::Qword)
}

#[test]
fn vcmppd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VCMPPD, operand1: Some(Direct(K6)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM3)), operand4: Some(Literal8(78)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 253, 14, 194, 243, 78], OperandSize::Dword)
}

#[test]
fn vcmppd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VCMPPD, operand1: Some(Direct(K2)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledDisplaced(EDX, Four, 1233784158, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(20)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 213, 9, 194, 20, 149, 94, 13, 138, 73, 20], OperandSize::Dword)
}

#[test]
fn vcmppd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VCMPPD, operand1: Some(Direct(K1)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectDisplaced(ESI, 85056583, Some(OperandSize::Qword), None)), operand4: Some(Literal8(88)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 241, 245, 28, 194, 142, 71, 220, 17, 5, 88], OperandSize::Dword)
}

#[test]
fn vcmppd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VCMPPD, operand1: Some(Direct(K1)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM5)), operand4: Some(Literal8(82)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 253, 15, 194, 205, 82], OperandSize::Qword)
}

#[test]
fn vcmppd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VCMPPD, operand1: Some(Direct(K3)), operand2: Some(Direct(XMM15)), operand3: Some(IndirectScaledDisplaced(RDI, Four, 1931114009, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(50)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 133, 15, 194, 28, 189, 25, 118, 26, 115, 50], OperandSize::Qword)
}

#[test]
fn vcmppd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VCMPPD, operand1: Some(Direct(K7)), operand2: Some(Direct(XMM29)), operand3: Some(Indirect(RDI, Some(OperandSize::Qword), None)), operand4: Some(Literal8(81)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 241, 149, 19, 194, 63, 81], OperandSize::Qword)
}

#[test]
fn vcmppd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VCMPPD, operand1: Some(Direct(K5)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(YMM4)), operand4: Some(Literal8(75)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 229, 41, 194, 236, 75], OperandSize::Dword)
}

#[test]
fn vcmppd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VCMPPD, operand1: Some(Direct(K3)), operand2: Some(Direct(YMM0)), operand3: Some(Indirect(EBX, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(5)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 253, 47, 194, 27, 5], OperandSize::Dword)
}

#[test]
fn vcmppd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VCMPPD, operand1: Some(Direct(K3)), operand2: Some(Direct(YMM3)), operand3: Some(Indirect(EAX, Some(OperandSize::Qword), None)), operand4: Some(Literal8(107)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 229, 60, 194, 24, 107], OperandSize::Dword)
}

#[test]
fn vcmppd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VCMPPD, operand1: Some(Direct(K5)), operand2: Some(Direct(YMM26)), operand3: Some(Direct(YMM16)), operand4: Some(Literal8(115)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 177, 173, 39, 194, 232, 115], OperandSize::Qword)
}

#[test]
fn vcmppd_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VCMPPD, operand1: Some(Direct(K7)), operand2: Some(Direct(YMM29)), operand3: Some(Indirect(RAX, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(102)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 149, 39, 194, 56, 102], OperandSize::Qword)
}

#[test]
fn vcmppd_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VCMPPD, operand1: Some(Direct(K5)), operand2: Some(Direct(YMM23)), operand3: Some(IndirectScaledDisplaced(RBX, Four, 109506585, Some(OperandSize::Qword), None)), operand4: Some(Literal8(99)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 197, 52, 194, 44, 157, 25, 240, 134, 6, 99], OperandSize::Qword)
}

#[test]
fn vcmppd_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VCMPPD, operand1: Some(Direct(K5)), operand2: Some(Direct(ZMM1)), operand3: Some(Direct(ZMM4)), operand4: Some(Literal8(84)), lock: false, rounding_mode: None, merge_mode: None, sae: true, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 245, 28, 194, 236, 84], OperandSize::Dword)
}

#[test]
fn vcmppd_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VCMPPD, operand1: Some(Direct(K1)), operand2: Some(Direct(ZMM2)), operand3: Some(Indirect(ECX, Some(OperandSize::Zmmword), None)), operand4: Some(Literal8(117)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 237, 79, 194, 9, 117], OperandSize::Dword)
}

#[test]
fn vcmppd_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VCMPPD, operand1: Some(Direct(K2)), operand2: Some(Direct(ZMM5)), operand3: Some(IndirectScaledDisplaced(EAX, Four, 1933551086, Some(OperandSize::Qword), None)), operand4: Some(Literal8(106)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 213, 90, 194, 20, 133, 238, 165, 63, 115, 106], OperandSize::Dword)
}

#[test]
fn vcmppd_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VCMPPD, operand1: Some(Direct(K6)), operand2: Some(Direct(ZMM20)), operand3: Some(Direct(ZMM19)), operand4: Some(Literal8(65)), lock: false, rounding_mode: None, merge_mode: None, sae: true, mask: Some(MaskReg::K6), broadcast: None }, &[98, 177, 221, 22, 194, 243, 65], OperandSize::Qword)
}

#[test]
fn vcmppd_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VCMPPD, operand1: Some(Direct(K3)), operand2: Some(Direct(ZMM24)), operand3: Some(IndirectScaledIndexedDisplaced(RAX, RAX, Eight, 1761047634, Some(OperandSize::Zmmword), None)), operand4: Some(Literal8(54)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 189, 70, 194, 156, 192, 82, 116, 247, 104, 54], OperandSize::Qword)
}

#[test]
fn vcmppd_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VCMPPD, operand1: Some(Direct(K2)), operand2: Some(Direct(ZMM27)), operand3: Some(IndirectScaledIndexedDisplaced(RSI, RBX, Eight, 83698049, Some(OperandSize::Qword), None)), operand4: Some(Literal8(117)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 165, 86, 194, 148, 222, 129, 33, 253, 4, 117], OperandSize::Qword)
}

