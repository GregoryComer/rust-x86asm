use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vcmppd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VCMPPD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM2)), operand4: Some(Literal8(45)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 209, 194, 226, 45], OperandSize::Dword)
}

#[test]
fn vcmppd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VCMPPD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectDisplaced(EAX, 1890248948, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(112)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 201, 194, 152, 244, 232, 170, 112, 112], OperandSize::Dword)
}

#[test]
fn vcmppd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VCMPPD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM1)), operand4: Some(Literal8(53)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 194, 225, 53], OperandSize::Qword)
}

#[test]
fn vcmppd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VCMPPD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledDisplaced(RSI, Two, 832034833, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(46)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 225, 194, 36, 117, 17, 216, 151, 49, 46], OperandSize::Qword)
}

#[test]
fn vcmppd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VCMPPD, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM3)), operand4: Some(Literal8(0)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 245, 194, 227, 0], OperandSize::Dword)
}

#[test]
fn vcmppd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VCMPPD, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectDisplaced(EDX, 5493428, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(60)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 194, 162, 180, 210, 83, 0, 60], OperandSize::Dword)
}

#[test]
fn vcmppd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VCMPPD, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM2)), operand4: Some(Literal8(1)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 237, 194, 194, 1], OperandSize::Qword)
}

#[test]
fn vcmppd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VCMPPD, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectScaledIndexedDisplaced(RDX, RAX, Two, 511201880, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(58)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 237, 194, 156, 66, 88, 82, 120, 30, 58], OperandSize::Qword)
}

#[test]
fn vcmppd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VCMPPD, operand1: Some(Direct(K4)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM0)), operand4: Some(Literal8(35)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 197, 11, 194, 224, 35], OperandSize::Dword)
}

#[test]
fn vcmppd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VCMPPD, operand1: Some(Direct(K7)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectDisplaced(EAX, 186021053, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(32)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 237, 9, 194, 184, 189, 116, 22, 11, 32], OperandSize::Dword)
}

#[test]
fn vcmppd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VCMPPD, operand1: Some(Direct(K3)), operand2: Some(Direct(XMM0)), operand3: Some(Indirect(EDX, Some(OperandSize::Qword), None)), operand4: Some(Literal8(52)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 241, 253, 31, 194, 26, 52], OperandSize::Dword)
}

#[test]
fn vcmppd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VCMPPD, operand1: Some(Direct(K2)), operand2: Some(Direct(XMM21)), operand3: Some(Direct(XMM2)), operand4: Some(Literal8(69)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 213, 4, 194, 210, 69], OperandSize::Qword)
}

#[test]
fn vcmppd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VCMPPD, operand1: Some(Direct(K3)), operand2: Some(Direct(XMM12)), operand3: Some(IndirectScaledIndexed(RCX, RBX, Two, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(66)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 157, 11, 194, 28, 89, 66], OperandSize::Qword)
}

#[test]
fn vcmppd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VCMPPD, operand1: Some(Direct(K1)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledIndexed(RAX, RDX, Eight, Some(OperandSize::Qword), None)), operand4: Some(Literal8(113)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 241, 237, 26, 194, 12, 208, 113], OperandSize::Qword)
}

#[test]
fn vcmppd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VCMPPD, operand1: Some(Direct(K6)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(YMM7)), operand4: Some(Literal8(31)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 213, 42, 194, 247, 31], OperandSize::Dword)
}

#[test]
fn vcmppd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VCMPPD, operand1: Some(Direct(K4)), operand2: Some(Direct(YMM7)), operand3: Some(Indirect(ESI, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(56)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 197, 47, 194, 38, 56], OperandSize::Dword)
}

#[test]
fn vcmppd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VCMPPD, operand1: Some(Direct(K3)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectDisplaced(EDI, 976741982, Some(OperandSize::Qword), None)), operand4: Some(Literal8(0)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 245, 59, 194, 159, 94, 230, 55, 58, 0], OperandSize::Dword)
}

#[test]
fn vcmppd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VCMPPD, operand1: Some(Direct(K2)), operand2: Some(Direct(YMM28)), operand3: Some(Direct(YMM4)), operand4: Some(Literal8(93)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 157, 33, 194, 212, 93], OperandSize::Qword)
}

#[test]
fn vcmppd_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VCMPPD, operand1: Some(Direct(K2)), operand2: Some(Direct(YMM21)), operand3: Some(Indirect(RCX, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(41)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 213, 36, 194, 17, 41], OperandSize::Qword)
}

#[test]
fn vcmppd_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VCMPPD, operand1: Some(Direct(K7)), operand2: Some(Direct(YMM12)), operand3: Some(IndirectScaledIndexedDisplaced(RCX, RAX, Eight, 1072826238, Some(OperandSize::Qword), None)), operand4: Some(Literal8(1)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 157, 60, 194, 188, 193, 126, 7, 242, 63, 1], OperandSize::Qword)
}

#[test]
fn vcmppd_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VCMPPD, operand1: Some(Direct(K6)), operand2: Some(Direct(ZMM1)), operand3: Some(Direct(ZMM2)), operand4: Some(Literal8(97)), lock: false, rounding_mode: None, merge_mode: None, sae: true, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 245, 27, 194, 242, 97], OperandSize::Dword)
}

#[test]
fn vcmppd_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VCMPPD, operand1: Some(Direct(K5)), operand2: Some(Direct(ZMM3)), operand3: Some(IndirectScaledIndexed(ESI, EAX, Eight, Some(OperandSize::Zmmword), None)), operand4: Some(Literal8(47)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 229, 79, 194, 44, 198, 47], OperandSize::Dword)
}

#[test]
fn vcmppd_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VCMPPD, operand1: Some(Direct(K5)), operand2: Some(Direct(ZMM3)), operand3: Some(IndirectScaledIndexedDisplaced(EDI, EAX, Two, 1314384095, Some(OperandSize::Qword), None)), operand4: Some(Literal8(111)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 229, 92, 194, 172, 71, 223, 232, 87, 78, 111], OperandSize::Dword)
}

#[test]
fn vcmppd_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VCMPPD, operand1: Some(Direct(K5)), operand2: Some(Direct(ZMM20)), operand3: Some(Direct(ZMM29)), operand4: Some(Literal8(89)), lock: false, rounding_mode: None, merge_mode: None, sae: true, mask: Some(MaskReg::K7), broadcast: None }, &[98, 145, 221, 23, 194, 237, 89], OperandSize::Qword)
}

#[test]
fn vcmppd_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VCMPPD, operand1: Some(Direct(K7)), operand2: Some(Direct(ZMM8)), operand3: Some(IndirectDisplaced(RDX, 1526906295, Some(OperandSize::Zmmword), None)), operand4: Some(Literal8(5)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 189, 75, 194, 186, 183, 189, 2, 91, 5], OperandSize::Qword)
}

#[test]
fn vcmppd_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VCMPPD, operand1: Some(Direct(K2)), operand2: Some(Direct(ZMM11)), operand3: Some(IndirectScaledDisplaced(RCX, Two, 1307610085, Some(OperandSize::Qword), None)), operand4: Some(Literal8(123)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 165, 89, 194, 20, 77, 229, 139, 240, 77, 123], OperandSize::Qword)
}

