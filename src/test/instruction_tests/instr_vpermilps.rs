use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpermilps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 65, 12, 255], OperandSize::Dword)
}

#[test]
fn vpermilps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledDisplaced(EDI, Eight, 868480113, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 105, 12, 36, 253, 113, 244, 195, 51], OperandSize::Dword)
}

#[test]
fn vpermilps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 105, 12, 211], OperandSize::Qword)
}

#[test]
fn vpermilps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledDisplaced(RDI, Two, 552731710, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 105, 12, 60, 125, 62, 4, 242, 32], OperandSize::Qword)
}

#[test]
fn vpermilps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPS, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 93, 12, 255], OperandSize::Dword)
}

#[test]
fn vpermilps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPS, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectScaledIndexed(EAX, ESI, Four, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 109, 12, 12, 176], OperandSize::Dword)
}

#[test]
fn vpermilps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPS, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(YMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 85, 12, 240], OperandSize::Qword)
}

#[test]
fn vpermilps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPS, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectScaledIndexedDisplaced(RAX, RCX, Two, 1257510928, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 101, 12, 164, 72, 16, 24, 244, 74], OperandSize::Qword)
}

#[test]
fn vpermilps_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 69, 142, 12, 216], OperandSize::Dword)
}

#[test]
fn vpermilps_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM0)), operand3: Some(Indirect(ECX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 125, 141, 12, 57], OperandSize::Dword)
}

#[test]
fn vpermilps_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectDisplaced(EAX, 637136361, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 69, 153, 12, 160, 233, 237, 249, 37], OperandSize::Dword)
}

#[test]
fn vpermilps_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM13)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 210, 93, 141, 12, 197], OperandSize::Qword)
}

#[test]
fn vpermilps_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledIndexed(RSI, RDX, Two, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 93, 142, 12, 12, 86], OperandSize::Qword)
}

#[test]
fn vpermilps_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPS, operand1: Some(Direct(XMM17)), operand2: Some(Direct(XMM23)), operand3: Some(Indirect(RCX, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 226, 69, 148, 12, 9], OperandSize::Qword)
}

#[test]
fn vpermilps_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPS, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 93, 170, 12, 224], OperandSize::Dword)
}

#[test]
fn vpermilps_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPS, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectScaledDisplaced(EDI, Eight, 142699279, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 109, 169, 12, 20, 253, 15, 107, 129, 8], OperandSize::Dword)
}

#[test]
fn vpermilps_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPS, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM3)), operand3: Some(Indirect(EDI, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 101, 189, 12, 39], OperandSize::Dword)
}

#[test]
fn vpermilps_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPS, operand1: Some(Direct(YMM18)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM31)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 130, 109, 169, 12, 215], OperandSize::Qword)
}

#[test]
fn vpermilps_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPS, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM13)), operand3: Some(IndirectDisplaced(RCX, 1675158490, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 21, 170, 12, 145, 218, 227, 216, 99], OperandSize::Qword)
}

#[test]
fn vpermilps_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPS, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM4)), operand3: Some(Indirect(RBX, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 93, 189, 12, 59], OperandSize::Qword)
}

#[test]
fn vpermilps_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPS, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM2)), operand3: Some(Direct(ZMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 109, 201, 12, 237], OperandSize::Dword)
}

#[test]
fn vpermilps_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPS, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM1)), operand3: Some(IndirectScaledDisplaced(EBX, Two, 949041319, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 117, 207, 12, 52, 93, 167, 56, 145, 56], OperandSize::Dword)
}

#[test]
fn vpermilps_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPS, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM1)), operand3: Some(IndirectDisplaced(EDI, 624243099, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 242, 117, 223, 12, 167, 155, 49, 53, 37], OperandSize::Dword)
}

#[test]
fn vpermilps_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPS, operand1: Some(Direct(ZMM27)), operand2: Some(Direct(ZMM27)), operand3: Some(Direct(ZMM28)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 2, 37, 196, 12, 220], OperandSize::Qword)
}

#[test]
fn vpermilps_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPS, operand1: Some(Direct(ZMM26)), operand2: Some(Direct(ZMM24)), operand3: Some(IndirectDisplaced(RAX, 2005925440, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 98, 61, 197, 12, 144, 64, 254, 143, 119], OperandSize::Qword)
}

#[test]
fn vpermilps_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPS, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM19)), operand3: Some(IndirectDisplaced(RDI, 999950092, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 242, 101, 213, 12, 151, 12, 7, 154, 59], OperandSize::Qword)
}

#[test]
fn vpermilps_27() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM2)), operand3: Some(Literal8(75)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 121, 4, 226, 75], OperandSize::Dword)
}

#[test]
fn vpermilps_28() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPS, operand1: Some(Direct(XMM0)), operand2: Some(IndirectScaledDisplaced(ESI, Eight, 458108979, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(114)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 121, 4, 4, 245, 51, 48, 78, 27, 114], OperandSize::Dword)
}

#[test]
fn vpermilps_29() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM7)), operand3: Some(Literal8(111)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 121, 4, 247, 111], OperandSize::Qword)
}

#[test]
fn vpermilps_30() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPS, operand1: Some(Direct(XMM2)), operand2: Some(IndirectDisplaced(RDI, 869749225, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(46)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 121, 4, 151, 233, 81, 215, 51, 46], OperandSize::Qword)
}

#[test]
fn vpermilps_31() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPS, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM0)), operand3: Some(Literal8(24)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 125, 4, 240, 24], OperandSize::Dword)
}

#[test]
fn vpermilps_32() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPS, operand1: Some(Direct(YMM7)), operand2: Some(IndirectScaledDisplaced(ESI, Two, 824592870, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(73)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 125, 4, 60, 117, 230, 73, 38, 49, 73], OperandSize::Dword)
}

#[test]
fn vpermilps_33() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPS, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM0)), operand3: Some(Literal8(1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 125, 4, 192, 1], OperandSize::Qword)
}

#[test]
fn vpermilps_34() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPS, operand1: Some(Direct(YMM7)), operand2: Some(Indirect(RCX, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(109)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 125, 4, 57, 109], OperandSize::Qword)
}

#[test]
fn vpermilps_35() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM4)), operand3: Some(Literal8(24)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 243, 125, 143, 4, 252, 24], OperandSize::Dword)
}

#[test]
fn vpermilps_36() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPS, operand1: Some(Direct(XMM0)), operand2: Some(IndirectScaledDisplaced(EAX, Eight, 276140191, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(33)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 243, 125, 140, 4, 4, 197, 159, 144, 117, 16, 33], OperandSize::Dword)
}

#[test]
fn vpermilps_37() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPS, operand1: Some(Direct(XMM3)), operand2: Some(IndirectScaledIndexedDisplaced(EDX, ESI, Two, 1381461977, Some(OperandSize::Dword), None)), operand3: Some(Literal8(41)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 243, 125, 154, 4, 156, 114, 217, 111, 87, 82, 41], OperandSize::Dword)
}

#[test]
fn vpermilps_38() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPS, operand1: Some(Direct(XMM31)), operand2: Some(Direct(XMM31)), operand3: Some(Literal8(117)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 3, 125, 137, 4, 255, 117], OperandSize::Qword)
}

#[test]
fn vpermilps_39() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPS, operand1: Some(Direct(XMM0)), operand2: Some(IndirectScaledIndexed(RSI, RCX, Two, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(94)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 243, 125, 140, 4, 4, 78, 94], OperandSize::Qword)
}

#[test]
fn vpermilps_40() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPS, operand1: Some(Direct(XMM27)), operand2: Some(IndirectScaledDisplaced(RAX, Four, 1994872738, Some(OperandSize::Dword), None)), operand3: Some(Literal8(5)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 99, 125, 157, 4, 28, 133, 162, 87, 231, 118, 5], OperandSize::Qword)
}

#[test]
fn vpermilps_41() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPS, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM5)), operand3: Some(Literal8(53)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 243, 125, 173, 4, 221, 53], OperandSize::Dword)
}

#[test]
fn vpermilps_42() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPS, operand1: Some(Direct(YMM7)), operand2: Some(IndirectScaledDisplaced(EAX, Eight, 1666720105, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(109)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 243, 125, 170, 4, 60, 197, 105, 33, 88, 99, 109], OperandSize::Dword)
}

#[test]
fn vpermilps_43() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPS, operand1: Some(Direct(YMM0)), operand2: Some(IndirectScaledIndexedDisplaced(EBX, ECX, Four, 572594231, Some(OperandSize::Dword), None)), operand3: Some(Literal8(45)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 243, 125, 190, 4, 132, 139, 55, 24, 33, 34, 45], OperandSize::Dword)
}

#[test]
fn vpermilps_44() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPS, operand1: Some(Direct(YMM13)), operand2: Some(Direct(YMM17)), operand3: Some(Literal8(79)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 51, 125, 169, 4, 233, 79], OperandSize::Qword)
}

#[test]
fn vpermilps_45() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPS, operand1: Some(Direct(YMM8)), operand2: Some(IndirectScaledIndexedDisplaced(RBX, RDI, Eight, 353174744, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(86)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 115, 125, 175, 4, 132, 251, 216, 4, 13, 21, 86], OperandSize::Qword)
}

#[test]
fn vpermilps_46() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPS, operand1: Some(Direct(YMM20)), operand2: Some(IndirectScaledIndexed(RCX, RCX, Eight, Some(OperandSize::Dword), None)), operand3: Some(Literal8(72)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 227, 125, 191, 4, 36, 201, 72], OperandSize::Qword)
}

#[test]
fn vpermilps_47() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPS, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM5)), operand3: Some(Literal8(1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 243, 125, 206, 4, 205, 1], OperandSize::Dword)
}

#[test]
fn vpermilps_48() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPS, operand1: Some(Direct(ZMM6)), operand2: Some(IndirectDisplaced(ESI, 1912753400, Some(OperandSize::Zmmword), None)), operand3: Some(Literal8(44)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 243, 125, 205, 4, 182, 248, 76, 2, 114, 44], OperandSize::Dword)
}

#[test]
fn vpermilps_49() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPS, operand1: Some(Direct(ZMM0)), operand2: Some(IndirectDisplaced(EDX, 1622468630, Some(OperandSize::Dword), None)), operand3: Some(Literal8(29)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 243, 125, 221, 4, 130, 22, 232, 180, 96, 29], OperandSize::Dword)
}

#[test]
fn vpermilps_50() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPS, operand1: Some(Direct(ZMM11)), operand2: Some(Direct(ZMM25)), operand3: Some(Literal8(4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 19, 125, 203, 4, 217, 4], OperandSize::Qword)
}

#[test]
fn vpermilps_51() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPS, operand1: Some(Direct(ZMM16)), operand2: Some(IndirectScaledIndexedDisplaced(RDX, RAX, Eight, 1725708906, Some(OperandSize::Zmmword), None)), operand3: Some(Literal8(73)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 227, 125, 201, 4, 132, 194, 106, 58, 220, 102, 73], OperandSize::Qword)
}

#[test]
fn vpermilps_52() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPS, operand1: Some(Direct(ZMM5)), operand2: Some(IndirectScaledIndexedDisplaced(RDX, RAX, Two, 1653916021, Some(OperandSize::Dword), None)), operand3: Some(Literal8(102)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 243, 125, 218, 4, 172, 66, 117, 193, 148, 98, 102], OperandSize::Qword)
}

