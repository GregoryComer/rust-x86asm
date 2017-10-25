use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpermilpd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 89, 13, 205], OperandSize::Dword)
}

#[test]
fn vpermilpd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledIndexedDisplaced(EBX, ESI, Four, 1354387993, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 89, 13, 156, 179, 25, 82, 186, 80], OperandSize::Dword)
}

#[test]
fn vpermilpd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 113, 13, 207], OperandSize::Qword)
}

#[test]
fn vpermilpd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM4)), operand3: Some(Indirect(RDI, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 89, 13, 63], OperandSize::Qword)
}

#[test]
fn vpermilpd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 117, 13, 236], OperandSize::Dword)
}

#[test]
fn vpermilpd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPD, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectScaledIndexedDisplaced(ESI, EBX, Four, 1526652471, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 101, 13, 156, 158, 55, 222, 254, 90], OperandSize::Dword)
}

#[test]
fn vpermilpd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPD, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 69, 13, 217], OperandSize::Qword)
}

#[test]
fn vpermilpd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectScaledIndexed(RDX, RSI, Two, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 13, 44, 114], OperandSize::Qword)
}

#[test]
fn vpermilpd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 229, 140, 13, 252], OperandSize::Dword)
}

#[test]
fn vpermilpd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM7)), operand3: Some(Indirect(EDX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 197, 137, 13, 2], OperandSize::Dword)
}

#[test]
fn vpermilpd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledIndexedDisplaced(EDI, EAX, Two, 760659501, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 242, 253, 155, 13, 188, 71, 45, 190, 86, 45], OperandSize::Dword)
}

#[test]
fn vpermilpd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPD, operand1: Some(Direct(XMM23)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM16)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 162, 197, 138, 13, 248], OperandSize::Qword)
}

#[test]
fn vpermilpd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPD, operand1: Some(Direct(XMM12)), operand2: Some(Direct(XMM28)), operand3: Some(IndirectScaledIndexedDisplaced(RDI, RAX, Four, 2069872759, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 114, 157, 135, 13, 164, 135, 119, 192, 95, 123], OperandSize::Qword)
}

#[test]
fn vpermilpd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPD, operand1: Some(Direct(XMM26)), operand2: Some(Direct(XMM14)), operand3: Some(Indirect(RBX, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 98, 141, 158, 13, 19], OperandSize::Qword)
}

#[test]
fn vpermilpd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPD, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(YMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 205, 170, 13, 230], OperandSize::Dword)
}

#[test]
fn vpermilpd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectScaledIndexed(ESI, EDX, Two, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 229, 174, 13, 60, 86], OperandSize::Dword)
}

#[test]
fn vpermilpd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPD, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectDisplaced(ECX, 1352502354, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 253, 185, 13, 145, 82, 140, 157, 80], OperandSize::Dword)
}

#[test]
fn vpermilpd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPD, operand1: Some(Direct(YMM8)), operand2: Some(Direct(YMM8)), operand3: Some(Direct(YMM20)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 50, 189, 174, 13, 196], OperandSize::Qword)
}

#[test]
fn vpermilpd_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPD, operand1: Some(Direct(YMM14)), operand2: Some(Direct(YMM4)), operand3: Some(Indirect(RDI, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 114, 221, 170, 13, 55], OperandSize::Qword)
}

#[test]
fn vpermilpd_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM15)), operand3: Some(IndirectDisplaced(RSI, 2041515254, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 133, 188, 13, 190, 246, 12, 175, 121], OperandSize::Qword)
}

#[test]
fn vpermilpd_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPD, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM2)), operand3: Some(Direct(ZMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 237, 201, 13, 236], OperandSize::Dword)
}

#[test]
fn vpermilpd_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPD, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM7)), operand3: Some(Indirect(EDX, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 197, 202, 13, 58], OperandSize::Dword)
}

#[test]
fn vpermilpd_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPD, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM0)), operand3: Some(IndirectScaledIndexed(EDX, ECX, Eight, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 253, 221, 13, 28, 202], OperandSize::Dword)
}

#[test]
fn vpermilpd_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPD, operand1: Some(Direct(ZMM15)), operand2: Some(Direct(ZMM9)), operand3: Some(Direct(ZMM25)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 18, 181, 206, 13, 249], OperandSize::Qword)
}

#[test]
fn vpermilpd_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPD, operand1: Some(Direct(ZMM11)), operand2: Some(Direct(ZMM14)), operand3: Some(Indirect(RCX, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 114, 141, 203, 13, 25], OperandSize::Qword)
}

#[test]
fn vpermilpd_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPD, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM6)), operand3: Some(Indirect(RCX, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 205, 221, 13, 33], OperandSize::Qword)
}

#[test]
fn vpermilpd_27() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM1)), operand3: Some(Literal8(7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 121, 5, 209, 7], OperandSize::Dword)
}

#[test]
fn vpermilpd_28() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPD, operand1: Some(Direct(XMM2)), operand2: Some(Indirect(EBX, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(81)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 121, 5, 19, 81], OperandSize::Dword)
}

#[test]
fn vpermilpd_29() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM7)), operand3: Some(Literal8(100)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 121, 5, 215, 100], OperandSize::Qword)
}

#[test]
fn vpermilpd_30() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPD, operand1: Some(Direct(XMM5)), operand2: Some(IndirectScaledIndexed(RCX, RDX, Eight, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(124)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 121, 5, 44, 209, 124], OperandSize::Qword)
}

#[test]
fn vpermilpd_31() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM2)), operand3: Some(Literal8(22)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 125, 5, 234, 22], OperandSize::Dword)
}

#[test]
fn vpermilpd_32() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPD, operand1: Some(Direct(YMM0)), operand2: Some(IndirectScaledIndexed(EAX, ECX, Eight, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(40)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 125, 5, 4, 200, 40], OperandSize::Dword)
}

#[test]
fn vpermilpd_33() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM5)), operand3: Some(Literal8(103)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 125, 5, 253, 103], OperandSize::Qword)
}

#[test]
fn vpermilpd_34() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPD, operand1: Some(Direct(YMM0)), operand2: Some(IndirectScaledDisplaced(RBX, Two, 2030148741, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 125, 5, 4, 93, 133, 156, 1, 121, 3], OperandSize::Qword)
}

#[test]
fn vpermilpd_35() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM7)), operand3: Some(Literal8(71)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 243, 253, 139, 5, 239, 71], OperandSize::Dword)
}

#[test]
fn vpermilpd_36() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPD, operand1: Some(Direct(XMM2)), operand2: Some(IndirectScaledIndexedDisplaced(EBX, EDI, Eight, 247847014, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(38)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 243, 253, 141, 5, 148, 251, 102, 216, 197, 14, 38], OperandSize::Dword)
}

#[test]
fn vpermilpd_37() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPD, operand1: Some(Direct(XMM6)), operand2: Some(IndirectScaledIndexed(ECX, ESI, Eight, Some(OperandSize::Qword), None)), operand3: Some(Literal8(49)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 243, 253, 153, 5, 52, 241, 49], OperandSize::Dword)
}

#[test]
fn vpermilpd_38() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPD, operand1: Some(Direct(XMM15)), operand2: Some(Direct(XMM14)), operand3: Some(Literal8(104)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 83, 253, 138, 5, 254, 104], OperandSize::Qword)
}

#[test]
fn vpermilpd_39() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPD, operand1: Some(Direct(XMM1)), operand2: Some(IndirectScaledIndexedDisplaced(RDI, RDX, Four, 1246370657, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(71)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 243, 253, 140, 5, 140, 151, 97, 27, 74, 74, 71], OperandSize::Qword)
}

#[test]
fn vpermilpd_40() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPD, operand1: Some(Direct(XMM27)), operand2: Some(IndirectScaledIndexed(RAX, RAX, Eight, Some(OperandSize::Qword), None)), operand3: Some(Literal8(69)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 99, 253, 157, 5, 28, 192, 69], OperandSize::Qword)
}

#[test]
fn vpermilpd_41() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPD, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM6)), operand3: Some(Literal8(45)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 243, 253, 174, 5, 214, 45], OperandSize::Dword)
}

#[test]
fn vpermilpd_42() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPD, operand1: Some(Direct(YMM2)), operand2: Some(IndirectScaledIndexedDisplaced(ECX, EAX, Eight, 282037771, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(101)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 243, 253, 169, 5, 148, 193, 11, 142, 207, 16, 101], OperandSize::Dword)
}

#[test]
fn vpermilpd_43() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPD, operand1: Some(Direct(YMM1)), operand2: Some(IndirectDisplaced(EDI, 1249116903, Some(OperandSize::Qword), None)), operand3: Some(Literal8(91)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 243, 253, 185, 5, 143, 231, 2, 116, 74, 91], OperandSize::Dword)
}

#[test]
fn vpermilpd_44() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPD, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM24)), operand3: Some(Literal8(61)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 147, 253, 175, 5, 216, 61], OperandSize::Qword)
}

#[test]
fn vpermilpd_45() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPD, operand1: Some(Direct(YMM28)), operand2: Some(Indirect(RCX, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(75)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 99, 253, 170, 5, 33, 75], OperandSize::Qword)
}

#[test]
fn vpermilpd_46() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPD, operand1: Some(Direct(YMM27)), operand2: Some(IndirectScaledIndexedDisplaced(RBX, RCX, Eight, 1458109680, Some(OperandSize::Qword), None)), operand3: Some(Literal8(69)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 99, 253, 191, 5, 156, 203, 240, 252, 232, 86, 69], OperandSize::Qword)
}

#[test]
fn vpermilpd_47() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPD, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM3)), operand3: Some(Literal8(14)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 243, 253, 201, 5, 235, 14], OperandSize::Dword)
}

#[test]
fn vpermilpd_48() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPD, operand1: Some(Direct(ZMM3)), operand2: Some(IndirectDisplaced(ECX, 1259247872, Some(OperandSize::Zmmword), None)), operand3: Some(Literal8(124)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 243, 253, 202, 5, 153, 0, 153, 14, 75, 124], OperandSize::Dword)
}

#[test]
fn vpermilpd_49() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPD, operand1: Some(Direct(ZMM7)), operand2: Some(Indirect(EBX, Some(OperandSize::Qword), None)), operand3: Some(Literal8(24)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 243, 253, 223, 5, 59, 24], OperandSize::Dword)
}

#[test]
fn vpermilpd_50() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPD, operand1: Some(Direct(ZMM31)), operand2: Some(Direct(ZMM15)), operand3: Some(Literal8(68)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 67, 253, 204, 5, 255, 68], OperandSize::Qword)
}

#[test]
fn vpermilpd_51() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPD, operand1: Some(Direct(ZMM23)), operand2: Some(IndirectScaledIndexed(RSI, RBX, Four, Some(OperandSize::Zmmword), None)), operand3: Some(Literal8(124)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 227, 253, 203, 5, 60, 158, 124], OperandSize::Qword)
}

#[test]
fn vpermilpd_52() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPD, operand1: Some(Direct(ZMM23)), operand2: Some(IndirectScaledIndexed(RCX, RBX, Four, Some(OperandSize::Qword), None)), operand3: Some(Literal8(25)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 227, 253, 217, 5, 60, 153, 25], OperandSize::Qword)
}

