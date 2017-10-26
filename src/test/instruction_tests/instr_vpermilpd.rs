use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpermilpd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 73, 13, 248], OperandSize::Dword)
}

#[test]
fn vpermilpd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledDisplaced(EDX, Four, 1713075574, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 105, 13, 20, 149, 118, 117, 27, 102], OperandSize::Dword)
}

#[test]
fn vpermilpd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 89, 13, 231], OperandSize::Qword)
}

#[test]
fn vpermilpd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM4)), operand3: Some(Indirect(RDI, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 89, 13, 23], OperandSize::Qword)
}

#[test]
fn vpermilpd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPD, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(YMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 101, 13, 242], OperandSize::Dword)
}

#[test]
fn vpermilpd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPD, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM1)), operand3: Some(Indirect(ESI, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 117, 13, 30], OperandSize::Dword)
}

#[test]
fn vpermilpd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPD, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 109, 13, 200], OperandSize::Qword)
}

#[test]
fn vpermilpd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPD, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectDisplaced(RCX, 1195770712, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 101, 13, 177, 88, 3, 70, 71], OperandSize::Qword)
}

#[test]
fn vpermilpd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 245, 141, 13, 245], OperandSize::Dword)
}

#[test]
fn vpermilpd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledIndexedDisplaced(ESI, ECX, Two, 483059091, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 237, 142, 13, 140, 78, 147, 229, 202, 28], OperandSize::Dword)
}

#[test]
fn vpermilpd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM1)), operand3: Some(Indirect(EBX, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 242, 245, 157, 13, 51], OperandSize::Dword)
}

#[test]
fn vpermilpd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 245, 138, 13, 220], OperandSize::Qword)
}

#[test]
fn vpermilpd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPD, operand1: Some(Direct(XMM31)), operand2: Some(Direct(XMM21)), operand3: Some(IndirectScaledDisplaced(RBX, Four, 1557486052, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 98, 213, 135, 13, 60, 157, 228, 89, 213, 92], OperandSize::Qword)
}

#[test]
fn vpermilpd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPD, operand1: Some(Direct(XMM28)), operand2: Some(Direct(XMM27)), operand3: Some(IndirectScaledIndexedDisplaced(RAX, RBX, Eight, 1570144773, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 98, 165, 151, 13, 164, 216, 5, 130, 150, 93], OperandSize::Qword)
}

#[test]
fn vpermilpd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPD, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(YMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 205, 175, 13, 241], OperandSize::Dword)
}

#[test]
fn vpermilpd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPD, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM1)), operand3: Some(Indirect(EDX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 245, 171, 13, 26], OperandSize::Dword)
}

#[test]
fn vpermilpd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPD, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectScaledDisplaced(EAX, Eight, 335998439, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 197, 187, 13, 36, 197, 231, 237, 6, 20], OperandSize::Dword)
}

#[test]
fn vpermilpd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPD, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM31)), operand3: Some(Direct(YMM14)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 210, 133, 161, 13, 198], OperandSize::Qword)
}

#[test]
fn vpermilpd_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPD, operand1: Some(Direct(YMM23)), operand2: Some(Direct(YMM30)), operand3: Some(Indirect(RSI, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 226, 141, 161, 13, 62], OperandSize::Qword)
}

#[test]
fn vpermilpd_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPD, operand1: Some(Direct(YMM29)), operand2: Some(Direct(YMM26)), operand3: Some(Indirect(RDI, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 98, 173, 183, 13, 47], OperandSize::Qword)
}

#[test]
fn vpermilpd_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPD, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM6)), operand3: Some(Direct(ZMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 205, 202, 13, 236], OperandSize::Dword)
}

#[test]
fn vpermilpd_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPD, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM1)), operand3: Some(IndirectDisplaced(EBX, 1482267748, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 245, 201, 13, 139, 100, 156, 89, 88], OperandSize::Dword)
}

#[test]
fn vpermilpd_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPD, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM0)), operand3: Some(IndirectScaledDisplaced(EBX, Two, 575085724, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 253, 220, 13, 28, 93, 156, 28, 71, 34], OperandSize::Dword)
}

#[test]
fn vpermilpd_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPD, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM22)), operand3: Some(Direct(ZMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 205, 198, 13, 212], OperandSize::Qword)
}

#[test]
fn vpermilpd_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPD, operand1: Some(Direct(ZMM15)), operand2: Some(Direct(ZMM18)), operand3: Some(IndirectScaledIndexedDisplaced(RBX, RAX, Two, 1579428799, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 114, 237, 194, 13, 188, 67, 191, 43, 36, 94], OperandSize::Qword)
}

#[test]
fn vpermilpd_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPD, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM30)), operand3: Some(IndirectDisplaced(RDI, 1922378514, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 141, 211, 13, 143, 18, 43, 149, 114], OperandSize::Qword)
}

#[test]
fn vpermilpd_27() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM0)), operand3: Some(Literal8(35)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 121, 5, 232, 35], OperandSize::Dword)
}

#[test]
fn vpermilpd_28() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPD, operand1: Some(Direct(XMM1)), operand2: Some(IndirectScaledIndexedDisplaced(EDI, ESI, Four, 1474538394, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(101)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 121, 5, 140, 183, 154, 171, 227, 87, 101], OperandSize::Dword)
}

#[test]
fn vpermilpd_29() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM6)), operand3: Some(Literal8(53)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 121, 5, 254, 53], OperandSize::Qword)
}

#[test]
fn vpermilpd_30() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPD, operand1: Some(Direct(XMM0)), operand2: Some(Indirect(RSI, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(110)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 121, 5, 6, 110], OperandSize::Qword)
}

#[test]
fn vpermilpd_31() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPD, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM4)), operand3: Some(Literal8(73)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 125, 5, 220, 73], OperandSize::Dword)
}

#[test]
fn vpermilpd_32() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPD, operand1: Some(Direct(YMM5)), operand2: Some(IndirectDisplaced(EAX, 310951448, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(75)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 125, 5, 168, 24, 190, 136, 18, 75], OperandSize::Dword)
}

#[test]
fn vpermilpd_33() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM6)), operand3: Some(Literal8(62)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 125, 5, 238, 62], OperandSize::Qword)
}

#[test]
fn vpermilpd_34() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPD, operand1: Some(Direct(YMM7)), operand2: Some(Indirect(RDX, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 125, 5, 58, 5], OperandSize::Qword)
}

#[test]
fn vpermilpd_35() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM4)), operand3: Some(Literal8(89)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 243, 253, 138, 5, 228, 89], OperandSize::Dword)
}

#[test]
fn vpermilpd_36() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPD, operand1: Some(Direct(XMM7)), operand2: Some(IndirectDisplaced(EDI, 554943944, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(74)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 243, 253, 138, 5, 191, 200, 197, 19, 33, 74], OperandSize::Dword)
}

#[test]
fn vpermilpd_37() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPD, operand1: Some(Direct(XMM3)), operand2: Some(IndirectScaledDisplaced(ESI, Four, 920586801, Some(OperandSize::Qword), None)), operand3: Some(Literal8(20)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 243, 253, 156, 5, 28, 181, 49, 10, 223, 54, 20], OperandSize::Dword)
}

#[test]
fn vpermilpd_38() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM19)), operand3: Some(Literal8(39)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 179, 253, 137, 5, 251, 39], OperandSize::Qword)
}

#[test]
fn vpermilpd_39() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPD, operand1: Some(Direct(XMM17)), operand2: Some(IndirectScaledDisplaced(RCX, Four, 111407566, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(64)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 227, 253, 142, 5, 12, 141, 206, 241, 163, 6, 64], OperandSize::Qword)
}

#[test]
fn vpermilpd_40() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPD, operand1: Some(Direct(XMM31)), operand2: Some(Indirect(RBX, Some(OperandSize::Qword), None)), operand3: Some(Literal8(101)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 99, 253, 156, 5, 59, 101], OperandSize::Qword)
}

#[test]
fn vpermilpd_41() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPD, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM6)), operand3: Some(Literal8(90)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 243, 253, 174, 5, 230, 90], OperandSize::Dword)
}

#[test]
fn vpermilpd_42() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPD, operand1: Some(Direct(YMM2)), operand2: Some(IndirectScaledIndexedDisplaced(EAX, EBX, Eight, 908264019, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(59)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 243, 253, 169, 5, 148, 216, 83, 2, 35, 54, 59], OperandSize::Dword)
}

#[test]
fn vpermilpd_43() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPD, operand1: Some(Direct(YMM4)), operand2: Some(IndirectScaledDisplaced(EDX, Two, 251456322, Some(OperandSize::Qword), None)), operand3: Some(Literal8(81)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 243, 253, 188, 5, 36, 85, 66, 235, 252, 14, 81], OperandSize::Dword)
}

#[test]
fn vpermilpd_44() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM4)), operand3: Some(Literal8(107)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 243, 253, 170, 5, 252, 107], OperandSize::Qword)
}

#[test]
fn vpermilpd_45() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPD, operand1: Some(Direct(YMM6)), operand2: Some(Indirect(RBX, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(44)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 243, 253, 175, 5, 51, 44], OperandSize::Qword)
}

#[test]
fn vpermilpd_46() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPD, operand1: Some(Direct(YMM26)), operand2: Some(IndirectScaledIndexed(RDI, RSI, Eight, Some(OperandSize::Qword), None)), operand3: Some(Literal8(70)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 99, 253, 188, 5, 20, 247, 70], OperandSize::Qword)
}

#[test]
fn vpermilpd_47() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPD, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM4)), operand3: Some(Literal8(90)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 243, 253, 207, 5, 236, 90], OperandSize::Dword)
}

#[test]
fn vpermilpd_48() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPD, operand1: Some(Direct(ZMM4)), operand2: Some(IndirectDisplaced(EDI, 1558677340, Some(OperandSize::Zmmword), None)), operand3: Some(Literal8(57)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 243, 253, 201, 5, 167, 92, 135, 231, 92, 57], OperandSize::Dword)
}

#[test]
fn vpermilpd_49() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPD, operand1: Some(Direct(ZMM6)), operand2: Some(IndirectScaledIndexedDisplaced(EBX, EDI, Two, 24878822, Some(OperandSize::Qword), None)), operand3: Some(Literal8(109)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 243, 253, 217, 5, 180, 123, 230, 158, 123, 1, 109], OperandSize::Dword)
}

#[test]
fn vpermilpd_50() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPD, operand1: Some(Direct(ZMM22)), operand2: Some(Direct(ZMM13)), operand3: Some(Literal8(89)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 195, 253, 207, 5, 245, 89], OperandSize::Qword)
}

#[test]
fn vpermilpd_51() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPD, operand1: Some(Direct(ZMM16)), operand2: Some(Indirect(RDX, Some(OperandSize::Zmmword), None)), operand3: Some(Literal8(96)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 227, 253, 202, 5, 2, 96], OperandSize::Qword)
}

#[test]
fn vpermilpd_52() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPD, operand1: Some(Direct(ZMM16)), operand2: Some(IndirectScaledIndexedDisplaced(RAX, RBX, Eight, 1625820865, Some(OperandSize::Qword), None)), operand3: Some(Literal8(44)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 227, 253, 220, 5, 132, 216, 193, 14, 232, 96, 44], OperandSize::Qword)
}

