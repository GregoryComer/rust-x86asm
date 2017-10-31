use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpermilpd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 65, 13, 227], OperandSize::Dword)
}

#[test]
fn vpermilpd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM3)), operand3: Some(Indirect(EBX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 97, 13, 27], OperandSize::Dword)
}

#[test]
fn vpermilpd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 65, 13, 228], OperandSize::Qword)
}

#[test]
fn vpermilpd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledIndexed(RDI, RCX, Two, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 65, 13, 44, 79], OperandSize::Qword)
}

#[test]
fn vpermilpd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPD, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 69, 13, 228], OperandSize::Dword)
}

#[test]
fn vpermilpd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPD, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM5)), operand3: Some(Indirect(ECX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 85, 13, 33], OperandSize::Dword)
}

#[test]
fn vpermilpd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPD, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 117, 13, 205], OperandSize::Qword)
}

#[test]
fn vpermilpd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPD, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectScaledIndexed(RAX, RDX, Eight, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 85, 13, 4, 208], OperandSize::Qword)
}

#[test]
fn vpermilpd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 213, 140, 13, 244], OperandSize::Dword)
}

#[test]
fn vpermilpd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectDisplaced(EAX, 1474953262, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 253, 139, 13, 144, 46, 0, 234, 87], OperandSize::Dword)
}

#[test]
fn vpermilpd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM4)), operand3: Some(Indirect(EBX, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 242, 221, 156, 13, 51], OperandSize::Dword)
}

#[test]
fn vpermilpd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPD, operand1: Some(Direct(XMM14)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM31)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 18, 229, 141, 13, 247], OperandSize::Qword)
}

#[test]
fn vpermilpd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM24)), operand3: Some(Indirect(RDI, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 189, 129, 13, 31], OperandSize::Qword)
}

#[test]
fn vpermilpd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPD, operand1: Some(Direct(XMM19)), operand2: Some(Direct(XMM21)), operand3: Some(IndirectDisplaced(RAX, 214614598, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 226, 213, 151, 13, 152, 70, 194, 202, 12], OperandSize::Qword)
}

#[test]
fn vpermilpd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPD, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 197, 172, 13, 231], OperandSize::Dword)
}

#[test]
fn vpermilpd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPD, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectScaledIndexedDisplaced(ESI, ESI, Four, 1511462092, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 221, 170, 13, 148, 182, 204, 20, 23, 90], OperandSize::Dword)
}

#[test]
fn vpermilpd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectDisplaced(EDX, 895806678, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 213, 186, 13, 170, 214, 236, 100, 53], OperandSize::Dword)
}

#[test]
fn vpermilpd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM25)), operand3: Some(Direct(YMM30)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 146, 181, 162, 13, 254], OperandSize::Qword)
}

#[test]
fn vpermilpd_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPD, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM12)), operand3: Some(Indirect(RAX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 157, 174, 13, 24], OperandSize::Qword)
}

#[test]
fn vpermilpd_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPD, operand1: Some(Direct(YMM19)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectDisplaced(RSI, 1546588976, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 226, 213, 191, 13, 158, 48, 19, 47, 92], OperandSize::Qword)
}

#[test]
fn vpermilpd_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPD, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM1)), operand3: Some(Direct(ZMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 245, 201, 13, 229], OperandSize::Dword)
}

#[test]
fn vpermilpd_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPD, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM5)), operand3: Some(IndirectScaledIndexed(EAX, EAX, Two, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 213, 205, 13, 36, 64], OperandSize::Dword)
}

#[test]
fn vpermilpd_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPD, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM0)), operand3: Some(Indirect(EDI, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 253, 217, 13, 15], OperandSize::Dword)
}

#[test]
fn vpermilpd_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPD, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM6)), operand3: Some(Direct(ZMM11)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 210, 205, 203, 13, 227], OperandSize::Qword)
}

#[test]
fn vpermilpd_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPD, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM6)), operand3: Some(IndirectDisplaced(RSI, 1973661905, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 205, 201, 13, 182, 209, 176, 163, 117], OperandSize::Qword)
}

#[test]
fn vpermilpd_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPD, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM26)), operand3: Some(IndirectScaledIndexedDisplaced(RDX, RCX, Eight, 1110385371, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 173, 215, 13, 140, 202, 219, 34, 47, 66], OperandSize::Qword)
}

#[test]
fn vpermilpd_27() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM6)), operand3: Some(Literal8(82)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 121, 5, 222, 82], OperandSize::Dword)
}

#[test]
fn vpermilpd_28() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPD, operand1: Some(Direct(XMM5)), operand2: Some(IndirectScaledIndexed(ECX, EAX, Eight, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(28)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 121, 5, 44, 193, 28], OperandSize::Dword)
}

#[test]
fn vpermilpd_29() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM2)), operand3: Some(Literal8(19)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 121, 5, 242, 19], OperandSize::Qword)
}

#[test]
fn vpermilpd_30() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPD, operand1: Some(Direct(XMM1)), operand2: Some(IndirectDisplaced(RSI, 197266747, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(45)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 121, 5, 142, 59, 13, 194, 11, 45], OperandSize::Qword)
}

#[test]
fn vpermilpd_31() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM6)), operand3: Some(Literal8(60)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 125, 5, 254, 60], OperandSize::Dword)
}

#[test]
fn vpermilpd_32() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPD, operand1: Some(Direct(YMM2)), operand2: Some(IndirectDisplaced(EDX, 1805458367, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(10)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 125, 5, 146, 191, 27, 157, 107, 10], OperandSize::Dword)
}

#[test]
fn vpermilpd_33() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPD, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM1)), operand3: Some(Literal8(18)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 125, 5, 225, 18], OperandSize::Qword)
}

#[test]
fn vpermilpd_34() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPD, operand1: Some(Direct(YMM2)), operand2: Some(IndirectScaledDisplaced(RCX, Eight, 1397159309, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(69)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 125, 5, 20, 205, 141, 245, 70, 83, 69], OperandSize::Qword)
}

#[test]
fn vpermilpd_35() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM6)), operand3: Some(Literal8(83)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 243, 253, 143, 5, 214, 83], OperandSize::Dword)
}

#[test]
fn vpermilpd_36() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPD, operand1: Some(Direct(XMM5)), operand2: Some(IndirectScaledIndexed(EAX, EDX, Four, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(83)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 243, 253, 140, 5, 44, 144, 83], OperandSize::Dword)
}

#[test]
fn vpermilpd_37() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPD, operand1: Some(Direct(XMM5)), operand2: Some(IndirectScaledDisplaced(ESI, Two, 1882347322, Some(OperandSize::Qword), None)), operand3: Some(Literal8(36)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 243, 253, 157, 5, 44, 117, 58, 87, 50, 112, 36], OperandSize::Dword)
}

#[test]
fn vpermilpd_38() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPD, operand1: Some(Direct(XMM23)), operand2: Some(Direct(XMM27)), operand3: Some(Literal8(99)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 131, 253, 143, 5, 251, 99], OperandSize::Qword)
}

#[test]
fn vpermilpd_39() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPD, operand1: Some(Direct(XMM7)), operand2: Some(Indirect(RSI, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(73)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 243, 253, 140, 5, 62, 73], OperandSize::Qword)
}

#[test]
fn vpermilpd_40() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPD, operand1: Some(Direct(XMM11)), operand2: Some(IndirectDisplaced(RSI, 14974793, Some(OperandSize::Qword), None)), operand3: Some(Literal8(55)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 115, 253, 155, 5, 158, 73, 127, 228, 0, 55], OperandSize::Qword)
}

#[test]
fn vpermilpd_41() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPD, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM6)), operand3: Some(Literal8(107)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 243, 253, 170, 5, 222, 107], OperandSize::Dword)
}

#[test]
fn vpermilpd_42() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPD, operand1: Some(Direct(YMM0)), operand2: Some(IndirectScaledIndexedDisplaced(EDX, ECX, Eight, 1441314341, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(16)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 243, 253, 171, 5, 132, 202, 37, 182, 232, 85, 16], OperandSize::Dword)
}

#[test]
fn vpermilpd_43() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPD, operand1: Some(Direct(YMM3)), operand2: Some(IndirectDisplaced(ECX, 359730729, Some(OperandSize::Qword), None)), operand3: Some(Literal8(83)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 243, 253, 191, 5, 153, 41, 14, 113, 21, 83], OperandSize::Dword)
}

#[test]
fn vpermilpd_44() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPD, operand1: Some(Direct(YMM28)), operand2: Some(Direct(YMM12)), operand3: Some(Literal8(101)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 67, 253, 171, 5, 228, 101], OperandSize::Qword)
}

#[test]
fn vpermilpd_45() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPD, operand1: Some(Direct(YMM1)), operand2: Some(IndirectScaledDisplaced(RSI, Eight, 1810398994, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(117)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 243, 253, 175, 5, 12, 245, 18, 127, 232, 107, 117], OperandSize::Qword)
}

#[test]
fn vpermilpd_46() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPD, operand1: Some(Direct(YMM23)), operand2: Some(IndirectScaledIndexedDisplaced(RDX, RSI, Eight, 1257596240, Some(OperandSize::Qword), None)), operand3: Some(Literal8(120)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 227, 253, 187, 5, 188, 242, 80, 101, 245, 74, 120], OperandSize::Qword)
}

#[test]
fn vpermilpd_47() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPD, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM5)), operand3: Some(Literal8(60)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 243, 253, 202, 5, 229, 60], OperandSize::Dword)
}

#[test]
fn vpermilpd_48() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPD, operand1: Some(Direct(ZMM1)), operand2: Some(IndirectDisplaced(ECX, 498319192, Some(OperandSize::Zmmword), None)), operand3: Some(Literal8(12)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 243, 253, 202, 5, 137, 88, 191, 179, 29, 12], OperandSize::Dword)
}

#[test]
fn vpermilpd_49() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPD, operand1: Some(Direct(ZMM3)), operand2: Some(IndirectScaledIndexedDisplaced(EDI, ESI, Four, 1322473036, Some(OperandSize::Qword), None)), operand3: Some(Literal8(13)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 243, 253, 221, 5, 156, 183, 76, 86, 211, 78, 13], OperandSize::Dword)
}

#[test]
fn vpermilpd_50() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPD, operand1: Some(Direct(ZMM28)), operand2: Some(Direct(ZMM3)), operand3: Some(Literal8(51)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 99, 253, 205, 5, 227, 51], OperandSize::Qword)
}

#[test]
fn vpermilpd_51() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPD, operand1: Some(Direct(ZMM4)), operand2: Some(Indirect(RCX, Some(OperandSize::Zmmword), None)), operand3: Some(Literal8(71)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 243, 253, 201, 5, 33, 71], OperandSize::Qword)
}

#[test]
fn vpermilpd_52() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMILPD, operand1: Some(Direct(ZMM12)), operand2: Some(IndirectScaledIndexed(RSI, RSI, Two, Some(OperandSize::Qword), None)), operand3: Some(Literal8(11)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 115, 253, 221, 5, 36, 118, 11], OperandSize::Qword)
}

