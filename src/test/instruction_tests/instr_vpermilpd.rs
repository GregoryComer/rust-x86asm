use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpermilpd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 105, 13, 220], OperandSize::Dword)
}

#[test]
fn vpermilpd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledDisplaced(EDI, Four, 482503670, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 89, 13, 44, 189, 246, 107, 194, 28], OperandSize::Dword)
}

#[test]
fn vpermilpd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 73, 13, 233], OperandSize::Qword)
}

#[test]
fn vpermilpd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM0)), operand3: Some(Indirect(RDX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 13, 34], OperandSize::Qword)
}

#[test]
fn vpermilpd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPD, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(YMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 13, 220], OperandSize::Dword)
}

#[test]
fn vpermilpd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPD, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM7)), operand3: Some(Indirect(EAX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 69, 13, 24], OperandSize::Dword)
}

#[test]
fn vpermilpd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(YMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 101, 13, 249], OperandSize::Qword)
}

#[test]
fn vpermilpd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectScaledDisplaced(RDX, Four, 1243049306, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 13, 44, 149, 90, 109, 23, 74], OperandSize::Qword)
}

#[test]
fn vpermilpd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 197, 139, 13, 219], OperandSize::Dword)
}

#[test]
fn vpermilpd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM7)), operand3: Some(Indirect(ESI, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 197, 143, 13, 6], OperandSize::Dword)
}

#[test]
fn vpermilpd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledIndexed(ECX, EDX, Two, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 242, 197, 158, 13, 4, 81], OperandSize::Dword)
}

#[test]
fn vpermilpd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPD, operand1: Some(Direct(XMM26)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM21)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 34, 221, 143, 13, 213], OperandSize::Qword)
}

#[test]
fn vpermilpd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPD, operand1: Some(Direct(XMM25)), operand2: Some(Direct(XMM10)), operand3: Some(IndirectScaledIndexedDisplaced(RCX, RSI, Four, 1768878841, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 98, 173, 142, 13, 140, 177, 249, 242, 110, 105], OperandSize::Qword)
}

#[test]
fn vpermilpd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPD, operand1: Some(Direct(XMM16)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledIndexed(RDI, RSI, Eight, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 226, 229, 155, 13, 4, 247], OperandSize::Qword)
}

#[test]
fn vpermilpd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPD, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(YMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 205, 169, 13, 204], OperandSize::Dword)
}

#[test]
fn vpermilpd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPD, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectScaledDisplaced(EBX, Eight, 1945059382, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 237, 170, 13, 52, 221, 54, 64, 239, 115], OperandSize::Dword)
}

#[test]
fn vpermilpd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectDisplaced(ECX, 930016510, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 229, 187, 13, 185, 254, 236, 110, 55], OperandSize::Dword)
}

#[test]
fn vpermilpd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM29)), operand3: Some(Direct(YMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 149, 165, 13, 236], OperandSize::Qword)
}

#[test]
fn vpermilpd_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPD, operand1: Some(Direct(YMM10)), operand2: Some(Direct(YMM13)), operand3: Some(IndirectDisplaced(RDX, 1377341902, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 114, 149, 170, 13, 146, 206, 145, 24, 82], OperandSize::Qword)
}

#[test]
fn vpermilpd_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPD, operand1: Some(Direct(YMM8)), operand2: Some(Direct(YMM26)), operand3: Some(IndirectScaledDisplaced(RBX, Four, 1255476530, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 114, 173, 180, 13, 4, 157, 50, 13, 213, 74], OperandSize::Qword)
}

#[test]
fn vpermilpd_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPD, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM2)), operand3: Some(Direct(ZMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 237, 207, 13, 222], OperandSize::Dword)
}

#[test]
fn vpermilpd_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPD, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM3)), operand3: Some(Indirect(EDI, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 229, 205, 13, 23], OperandSize::Dword)
}

#[test]
fn vpermilpd_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPD, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM6)), operand3: Some(IndirectScaledDisplaced(ESI, Two, 1668215010, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 205, 221, 13, 12, 117, 226, 240, 110, 99], OperandSize::Dword)
}

#[test]
fn vpermilpd_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPD, operand1: Some(Direct(ZMM19)), operand2: Some(Direct(ZMM8)), operand3: Some(Direct(ZMM8)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 194, 189, 207, 13, 216], OperandSize::Qword)
}

#[test]
fn vpermilpd_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPD, operand1: Some(Direct(ZMM13)), operand2: Some(Direct(ZMM15)), operand3: Some(Indirect(RDI, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 114, 133, 201, 13, 47], OperandSize::Qword)
}

#[test]
fn vpermilpd_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPD, operand1: Some(Direct(ZMM26)), operand2: Some(Direct(ZMM8)), operand3: Some(IndirectDisplaced(RAX, 781557700, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 98, 189, 218, 13, 144, 196, 159, 149, 46], OperandSize::Qword)
}

#[test]
fn vpermilpd_27() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM6)), operand3: Some(Literal8(88)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 121, 5, 246, 88], OperandSize::Dword)
}

#[test]
fn vpermilpd_28() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPD, operand1: Some(Direct(XMM2)), operand2: Some(IndirectScaledDisplaced(ECX, Eight, 898798005, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(84)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 121, 5, 20, 205, 181, 145, 146, 53, 84], OperandSize::Dword)
}

#[test]
fn vpermilpd_29() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM7)), operand3: Some(Literal8(64)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 121, 5, 207, 64], OperandSize::Qword)
}

#[test]
fn vpermilpd_30() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPD, operand1: Some(Direct(XMM0)), operand2: Some(IndirectScaledIndexed(RCX, RDX, Eight, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(43)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 121, 5, 4, 209, 43], OperandSize::Qword)
}

#[test]
fn vpermilpd_31() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPD, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM5)), operand3: Some(Literal8(77)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 125, 5, 197, 77], OperandSize::Dword)
}

#[test]
fn vpermilpd_32() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPD, operand1: Some(Direct(YMM3)), operand2: Some(IndirectScaledDisplaced(EDI, Eight, 1474998057, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(45)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 125, 5, 28, 253, 41, 175, 234, 87, 45], OperandSize::Dword)
}

#[test]
fn vpermilpd_33() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPD, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM3)), operand3: Some(Literal8(14)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 125, 5, 211, 14], OperandSize::Qword)
}

#[test]
fn vpermilpd_34() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPD, operand1: Some(Direct(YMM1)), operand2: Some(IndirectScaledIndexedDisplaced(RBX, RDX, Two, 1505620897, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(57)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 125, 5, 140, 83, 161, 243, 189, 89, 57], OperandSize::Qword)
}

#[test]
fn vpermilpd_35() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM5)), operand3: Some(Literal8(34)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 243, 253, 140, 5, 229, 34], OperandSize::Dword)
}

#[test]
fn vpermilpd_36() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPD, operand1: Some(Direct(XMM5)), operand2: Some(IndirectScaledIndexed(EDI, ESI, Two, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(10)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 243, 253, 139, 5, 44, 119, 10], OperandSize::Dword)
}

#[test]
fn vpermilpd_37() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPD, operand1: Some(Direct(XMM5)), operand2: Some(IndirectScaledIndexedDisplaced(EDX, ECX, Eight, 603252125, Some(OperandSize::Qword), None)), operand3: Some(Literal8(94)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 243, 253, 159, 5, 172, 202, 157, 229, 244, 35, 94], OperandSize::Dword)
}

#[test]
fn vpermilpd_38() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM23)), operand3: Some(Literal8(3)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 179, 253, 140, 5, 255, 3], OperandSize::Qword)
}

#[test]
fn vpermilpd_39() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPD, operand1: Some(Direct(XMM8)), operand2: Some(IndirectDisplaced(RCX, 964349003, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(42)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 115, 253, 137, 5, 129, 75, 204, 122, 57, 42], OperandSize::Qword)
}

#[test]
fn vpermilpd_40() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPD, operand1: Some(Direct(XMM30)), operand2: Some(IndirectDisplaced(RDI, 1376854523, Some(OperandSize::Qword), None)), operand3: Some(Literal8(87)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 99, 253, 156, 5, 183, 251, 33, 17, 82, 87], OperandSize::Qword)
}

#[test]
fn vpermilpd_41() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPD, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM0)), operand3: Some(Literal8(37)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 243, 253, 170, 5, 192, 37], OperandSize::Dword)
}

#[test]
fn vpermilpd_42() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPD, operand1: Some(Direct(YMM3)), operand2: Some(Indirect(ECX, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(61)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 243, 253, 175, 5, 25, 61], OperandSize::Dword)
}

#[test]
fn vpermilpd_43() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPD, operand1: Some(Direct(YMM6)), operand2: Some(IndirectDisplaced(EDI, 1603733893, Some(OperandSize::Qword), None)), operand3: Some(Literal8(112)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 243, 253, 188, 5, 183, 133, 9, 151, 95, 112], OperandSize::Dword)
}

#[test]
fn vpermilpd_44() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPD, operand1: Some(Direct(YMM22)), operand2: Some(Direct(YMM26)), operand3: Some(Literal8(79)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 131, 253, 171, 5, 242, 79], OperandSize::Qword)
}

#[test]
fn vpermilpd_45() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPD, operand1: Some(Direct(YMM6)), operand2: Some(Indirect(RAX, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(50)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 243, 253, 169, 5, 48, 50], OperandSize::Qword)
}

#[test]
fn vpermilpd_46() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPD, operand1: Some(Direct(YMM5)), operand2: Some(IndirectScaledIndexedDisplaced(RBX, RDX, Four, 612873138, Some(OperandSize::Qword), None)), operand3: Some(Literal8(23)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 243, 253, 191, 5, 172, 147, 178, 179, 135, 36, 23], OperandSize::Qword)
}

#[test]
fn vpermilpd_47() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPD, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM0)), operand3: Some(Literal8(32)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 243, 253, 205, 5, 224, 32], OperandSize::Dword)
}

#[test]
fn vpermilpd_48() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPD, operand1: Some(Direct(ZMM1)), operand2: Some(IndirectScaledIndexed(EAX, ESI, Eight, Some(OperandSize::Zmmword), None)), operand3: Some(Literal8(37)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 243, 253, 204, 5, 12, 240, 37], OperandSize::Dword)
}

#[test]
fn vpermilpd_49() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPD, operand1: Some(Direct(ZMM4)), operand2: Some(IndirectScaledIndexed(ECX, EBX, Two, Some(OperandSize::Qword), None)), operand3: Some(Literal8(21)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 243, 253, 217, 5, 36, 89, 21], OperandSize::Dword)
}

#[test]
fn vpermilpd_50() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPD, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM16)), operand3: Some(Literal8(85)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 179, 253, 204, 5, 200, 85], OperandSize::Qword)
}

#[test]
fn vpermilpd_51() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPD, operand1: Some(Direct(ZMM20)), operand2: Some(IndirectScaledIndexed(RDI, RCX, Four, Some(OperandSize::Zmmword), None)), operand3: Some(Literal8(60)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 227, 253, 207, 5, 36, 143, 60], OperandSize::Qword)
}

#[test]
fn vpermilpd_52() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPD, operand1: Some(Direct(ZMM2)), operand2: Some(Indirect(RSI, Some(OperandSize::Qword), None)), operand3: Some(Literal8(107)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 243, 253, 220, 5, 22, 107], OperandSize::Qword)
}

