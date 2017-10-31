use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpermilps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 105, 12, 233], OperandSize::Dword)
}

#[test]
fn vpermilps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledDisplaced(EAX, Four, 1701294474, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 97, 12, 28, 133, 138, 177, 103, 101], OperandSize::Dword)
}

#[test]
fn vpermilps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 73, 12, 229], OperandSize::Qword)
}

#[test]
fn vpermilps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledIndexed(RDX, RAX, Two, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 65, 12, 20, 66], OperandSize::Qword)
}

#[test]
fn vpermilps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPS, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(YMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 85, 12, 212], OperandSize::Dword)
}

#[test]
fn vpermilps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPS, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectDisplaced(EAX, 1833778787, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 12, 176, 99, 62, 77, 109], OperandSize::Dword)
}

#[test]
fn vpermilps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPS, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(YMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 101, 12, 217], OperandSize::Qword)
}

#[test]
fn vpermilps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPS, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectScaledDisplaced(RDX, Two, 717795399, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 77, 12, 12, 85, 71, 176, 200, 42], OperandSize::Qword)
}

#[test]
fn vpermilps_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 125, 140, 12, 229], OperandSize::Dword)
}

#[test]
fn vpermilps_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectDisplaced(ESI, 889622765, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 117, 139, 12, 158, 237, 144, 6, 53], OperandSize::Dword)
}

#[test]
fn vpermilps_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledDisplaced(EBX, Four, 849384744, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 85, 155, 12, 52, 157, 40, 149, 160, 50], OperandSize::Dword)
}

#[test]
fn vpermilps_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPS, operand1: Some(Direct(XMM18)), operand2: Some(Direct(XMM11)), operand3: Some(Direct(XMM22)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 162, 37, 141, 12, 214], OperandSize::Qword)
}

#[test]
fn vpermilps_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM8)), operand3: Some(Indirect(RDX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 61, 143, 12, 26], OperandSize::Qword)
}

#[test]
fn vpermilps_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPS, operand1: Some(Direct(XMM24)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectScaledIndexedDisplaced(RDI, RCX, Eight, 1021507214, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 98, 77, 155, 12, 132, 207, 142, 246, 226, 60], OperandSize::Qword)
}

#[test]
fn vpermilps_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPS, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(YMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 77, 172, 12, 216], OperandSize::Dword)
}

#[test]
fn vpermilps_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPS, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectScaledDisplaced(EBX, Two, 1672436832, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 93, 172, 12, 12, 93, 96, 92, 175, 99], OperandSize::Dword)
}

#[test]
fn vpermilps_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPS, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectDisplaced(EDX, 1841749834, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 101, 191, 12, 138, 74, 223, 198, 109], OperandSize::Dword)
}

#[test]
fn vpermilps_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPS, operand1: Some(Direct(YMM31)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM11)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 66, 117, 172, 12, 251], OperandSize::Qword)
}

#[test]
fn vpermilps_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPS, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM17)), operand3: Some(IndirectScaledDisplaced(RCX, Four, 88701433, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 117, 161, 12, 12, 141, 249, 121, 73, 5], OperandSize::Qword)
}

#[test]
fn vpermilps_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPS, operand1: Some(Direct(YMM17)), operand2: Some(Direct(YMM27)), operand3: Some(Indirect(RCX, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 226, 37, 179, 12, 9], OperandSize::Qword)
}

#[test]
fn vpermilps_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPS, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM1)), operand3: Some(Direct(ZMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 117, 204, 12, 236], OperandSize::Dword)
}

#[test]
fn vpermilps_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPS, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM6)), operand3: Some(IndirectScaledDisplaced(EDX, Four, 963249858, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 77, 204, 12, 4, 149, 194, 6, 106, 57], OperandSize::Dword)
}

#[test]
fn vpermilps_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPS, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM2)), operand3: Some(IndirectScaledIndexedDisplaced(EBX, ESI, Two, 1297175989, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 242, 109, 221, 12, 132, 115, 181, 85, 81, 77], OperandSize::Dword)
}

#[test]
fn vpermilps_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPS, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM1)), operand3: Some(Direct(ZMM10)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 210, 117, 204, 12, 194], OperandSize::Qword)
}

#[test]
fn vpermilps_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPS, operand1: Some(Direct(ZMM14)), operand2: Some(Direct(ZMM20)), operand3: Some(IndirectScaledIndexedDisplaced(RBX, RAX, Four, 773034948, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 114, 93, 198, 12, 180, 131, 196, 147, 19, 46], OperandSize::Qword)
}

#[test]
fn vpermilps_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPS, operand1: Some(Direct(ZMM10)), operand2: Some(Direct(ZMM1)), operand3: Some(Indirect(RBX, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 114, 117, 218, 12, 19], OperandSize::Qword)
}

#[test]
fn vpermilps_27() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM1)), operand3: Some(Literal8(75)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 121, 4, 241, 75], OperandSize::Dword)
}

#[test]
fn vpermilps_28() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPS, operand1: Some(Direct(XMM6)), operand2: Some(IndirectScaledIndexedDisplaced(ECX, EAX, Eight, 902322051, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(59)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 121, 4, 180, 193, 131, 87, 200, 53, 59], OperandSize::Dword)
}

#[test]
fn vpermilps_29() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM0)), operand3: Some(Literal8(41)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 121, 4, 248, 41], OperandSize::Qword)
}

#[test]
fn vpermilps_30() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPS, operand1: Some(Direct(XMM7)), operand2: Some(Indirect(RDI, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 121, 4, 63, 2], OperandSize::Qword)
}

#[test]
fn vpermilps_31() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPS, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM0)), operand3: Some(Literal8(50)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 125, 4, 216, 50], OperandSize::Dword)
}

#[test]
fn vpermilps_32() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPS, operand1: Some(Direct(YMM3)), operand2: Some(IndirectDisplaced(EDI, 1250518652, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(77)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 125, 4, 159, 124, 102, 137, 74, 77], OperandSize::Dword)
}

#[test]
fn vpermilps_33() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPS, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM0)), operand3: Some(Literal8(90)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 125, 4, 216, 90], OperandSize::Qword)
}

#[test]
fn vpermilps_34() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPS, operand1: Some(Direct(YMM3)), operand2: Some(IndirectDisplaced(RAX, 2093057647, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(84)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 125, 4, 152, 111, 134, 193, 124, 84], OperandSize::Qword)
}

#[test]
fn vpermilps_35() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM5)), operand3: Some(Literal8(104)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 243, 125, 138, 4, 237, 104], OperandSize::Dword)
}

#[test]
fn vpermilps_36() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPS, operand1: Some(Direct(XMM3)), operand2: Some(IndirectDisplaced(EDX, 137975472, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(21)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 243, 125, 141, 4, 154, 176, 86, 57, 8, 21], OperandSize::Dword)
}

#[test]
fn vpermilps_37() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPS, operand1: Some(Direct(XMM4)), operand2: Some(IndirectScaledIndexed(ECX, EDI, Four, Some(OperandSize::Dword), None)), operand3: Some(Literal8(43)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 243, 125, 153, 4, 36, 185, 43], OperandSize::Dword)
}

#[test]
fn vpermilps_38() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPS, operand1: Some(Direct(XMM10)), operand2: Some(Direct(XMM20)), operand3: Some(Literal8(43)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 51, 125, 142, 4, 212, 43], OperandSize::Qword)
}

#[test]
fn vpermilps_39() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPS, operand1: Some(Direct(XMM25)), operand2: Some(IndirectScaledIndexed(RAX, RAX, Eight, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(122)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 99, 125, 141, 4, 12, 192, 122], OperandSize::Qword)
}

#[test]
fn vpermilps_40() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPS, operand1: Some(Direct(XMM17)), operand2: Some(IndirectScaledIndexedDisplaced(RDI, RDI, Eight, 521568113, Some(OperandSize::Dword), None)), operand3: Some(Literal8(63)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 227, 125, 157, 4, 140, 255, 113, 127, 22, 31, 63], OperandSize::Qword)
}

#[test]
fn vpermilps_41() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPS, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM0)), operand3: Some(Literal8(60)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 243, 125, 174, 4, 224, 60], OperandSize::Dword)
}

#[test]
fn vpermilps_42() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPS, operand1: Some(Direct(YMM7)), operand2: Some(IndirectDisplaced(EAX, 1438091210, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(82)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 243, 125, 174, 4, 184, 202, 135, 183, 85, 82], OperandSize::Dword)
}

#[test]
fn vpermilps_43() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPS, operand1: Some(Direct(YMM2)), operand2: Some(Indirect(EBX, Some(OperandSize::Dword), None)), operand3: Some(Literal8(96)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 243, 125, 191, 4, 19, 96], OperandSize::Dword)
}

#[test]
fn vpermilps_44() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPS, operand1: Some(Direct(YMM18)), operand2: Some(Direct(YMM13)), operand3: Some(Literal8(20)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 195, 125, 172, 4, 213, 20], OperandSize::Qword)
}

#[test]
fn vpermilps_45() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPS, operand1: Some(Direct(YMM30)), operand2: Some(IndirectScaledIndexedDisplaced(RDI, RAX, Two, 517387649, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(50)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 99, 125, 171, 4, 180, 71, 129, 181, 214, 30, 50], OperandSize::Qword)
}

#[test]
fn vpermilps_46() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPS, operand1: Some(Direct(YMM7)), operand2: Some(IndirectDisplaced(RDI, 1110866970, Some(OperandSize::Dword), None)), operand3: Some(Literal8(97)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 243, 125, 190, 4, 191, 26, 124, 54, 66, 97], OperandSize::Qword)
}

#[test]
fn vpermilps_47() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPS, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM5)), operand3: Some(Literal8(73)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 243, 125, 203, 4, 221, 73], OperandSize::Dword)
}

#[test]
fn vpermilps_48() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPS, operand1: Some(Direct(ZMM4)), operand2: Some(Indirect(ESI, Some(OperandSize::Zmmword), None)), operand3: Some(Literal8(42)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 243, 125, 201, 4, 38, 42], OperandSize::Dword)
}

#[test]
fn vpermilps_49() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPS, operand1: Some(Direct(ZMM1)), operand2: Some(IndirectScaledDisplaced(ESI, Two, 1920316964, Some(OperandSize::Dword), None)), operand3: Some(Literal8(37)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 243, 125, 220, 4, 12, 117, 36, 182, 117, 114, 37], OperandSize::Dword)
}

#[test]
fn vpermilps_50() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPS, operand1: Some(Direct(ZMM10)), operand2: Some(Direct(ZMM26)), operand3: Some(Literal8(115)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 19, 125, 202, 4, 210, 115], OperandSize::Qword)
}

#[test]
fn vpermilps_51() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPS, operand1: Some(Direct(ZMM29)), operand2: Some(Indirect(RDI, Some(OperandSize::Zmmword), None)), operand3: Some(Literal8(96)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 99, 125, 205, 4, 47, 96], OperandSize::Qword)
}

#[test]
fn vpermilps_52() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPS, operand1: Some(Direct(ZMM28)), operand2: Some(IndirectScaledDisplaced(RAX, Four, 55830062, Some(OperandSize::Dword), None)), operand3: Some(Literal8(59)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 99, 125, 217, 4, 36, 133, 46, 230, 83, 3, 59], OperandSize::Qword)
}

