use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpslld_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM0)), operand3: Some(Literal8(91)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 209, 114, 240, 91], OperandSize::Dword)
}

#[test]
fn vpslld_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM6)), operand3: Some(Literal8(46)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 225, 114, 246, 46], OperandSize::Qword)
}

#[test]
fn vpslld_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLD, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM3)), operand3: Some(Literal8(58)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 245, 114, 243, 58], OperandSize::Dword)
}

#[test]
fn vpslld_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLD, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM1)), operand3: Some(Literal8(10)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 237, 114, 241, 10], OperandSize::Qword)
}

#[test]
fn vpslld_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM7)), operand3: Some(Literal8(116)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 109, 139, 114, 247, 116], OperandSize::Dword)
}

#[test]
fn vpslld_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLD, operand1: Some(Direct(XMM3)), operand2: Some(IndirectScaledIndexed(ESI, EDI, Four, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(42)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 101, 140, 114, 52, 190, 42], OperandSize::Dword)
}

#[test]
fn vpslld_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLD, operand1: Some(Direct(XMM7)), operand2: Some(IndirectScaledIndexedDisplaced(ESI, EDI, Eight, 824977050, Some(OperandSize::Dword), None)), operand3: Some(Literal8(12)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 69, 157, 114, 180, 254, 154, 38, 44, 49, 12], OperandSize::Dword)
}

#[test]
fn vpslld_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLD, operand1: Some(Direct(XMM27)), operand2: Some(Direct(XMM3)), operand3: Some(Literal8(119)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 37, 134, 114, 243, 119], OperandSize::Qword)
}

#[test]
fn vpslld_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLD, operand1: Some(Direct(XMM6)), operand2: Some(IndirectScaledIndexed(RAX, RDI, Four, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(74)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 77, 141, 114, 52, 184, 74], OperandSize::Qword)
}

#[test]
fn vpslld_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLD, operand1: Some(Direct(XMM14)), operand2: Some(IndirectDisplaced(RDI, 486684296, Some(OperandSize::Dword), None)), operand3: Some(Literal8(0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 13, 158, 114, 183, 136, 54, 2, 29, 0], OperandSize::Qword)
}

#[test]
fn vpslld_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLD, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM6)), operand3: Some(Literal8(5)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 77, 169, 114, 246, 5], OperandSize::Dword)
}

#[test]
fn vpslld_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLD, operand1: Some(Direct(YMM6)), operand2: Some(IndirectScaledDisplaced(EBX, Eight, 1444392426, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(34)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 77, 173, 114, 52, 221, 234, 173, 23, 86, 34], OperandSize::Dword)
}

#[test]
fn vpslld_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLD, operand1: Some(Direct(YMM0)), operand2: Some(IndirectScaledIndexedDisplaced(ECX, EDI, Four, 872172026, Some(OperandSize::Dword), None)), operand3: Some(Literal8(58)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 125, 190, 114, 180, 185, 250, 73, 252, 51, 58], OperandSize::Dword)
}

#[test]
fn vpslld_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM8)), operand3: Some(Literal8(14)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 209, 85, 173, 114, 240, 14], OperandSize::Qword)
}

#[test]
fn vpslld_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLD, operand1: Some(Direct(YMM8)), operand2: Some(Indirect(RSI, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(99)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 61, 174, 114, 54, 99], OperandSize::Qword)
}

#[test]
fn vpslld_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLD, operand1: Some(Direct(YMM30)), operand2: Some(IndirectDisplaced(RDX, 1608869040, Some(OperandSize::Dword), None)), operand3: Some(Literal8(23)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 13, 182, 114, 178, 176, 100, 229, 95, 23], OperandSize::Qword)
}

#[test]
fn vpslld_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLD, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM7)), operand3: Some(Literal8(80)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 117, 206, 114, 247, 80], OperandSize::Dword)
}

#[test]
fn vpslld_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLD, operand1: Some(Direct(ZMM4)), operand2: Some(IndirectDisplaced(ESI, 1264720771, Some(OperandSize::Zmmword), None)), operand3: Some(Literal8(120)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 93, 204, 114, 182, 131, 27, 98, 75, 120], OperandSize::Dword)
}

#[test]
fn vpslld_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLD, operand1: Some(Direct(ZMM5)), operand2: Some(Indirect(EDX, Some(OperandSize::Dword), None)), operand3: Some(Literal8(54)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 241, 85, 217, 114, 50, 54], OperandSize::Dword)
}

#[test]
fn vpslld_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLD, operand1: Some(Direct(ZMM23)), operand2: Some(Direct(ZMM13)), operand3: Some(Literal8(21)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 209, 69, 194, 114, 245, 21], OperandSize::Qword)
}

#[test]
fn vpslld_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLD, operand1: Some(Direct(ZMM9)), operand2: Some(IndirectScaledDisplaced(RSI, Eight, 567533569, Some(OperandSize::Zmmword), None)), operand3: Some(Literal8(124)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 53, 202, 114, 52, 245, 1, 224, 211, 33, 124], OperandSize::Qword)
}

#[test]
fn vpslld_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLD, operand1: Some(Direct(ZMM19)), operand2: Some(IndirectScaledDisplaced(RCX, Four, 2005416795, Some(OperandSize::Dword), None)), operand3: Some(Literal8(22)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 241, 101, 215, 114, 52, 141, 91, 59, 136, 119, 22], OperandSize::Qword)
}

#[test]
fn vpslld_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 209, 242, 221], OperandSize::Dword)
}

#[test]
fn vpslld_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledIndexed(ESI, EAX, Two, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 193, 242, 28, 70], OperandSize::Dword)
}

#[test]
fn vpslld_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 225, 242, 200], OperandSize::Qword)
}

#[test]
fn vpslld_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledIndexedDisplaced(RCX, RSI, Eight, 1424536043, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 193, 242, 172, 241, 235, 177, 232, 84], OperandSize::Qword)
}

#[test]
fn vpslld_27() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLD, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 213, 242, 196], OperandSize::Dword)
}

#[test]
fn vpslld_28() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLD, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectDisplaced(EBX, 1878896110, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 242, 163, 238, 173, 253, 111], OperandSize::Dword)
}

#[test]
fn vpslld_29() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLD, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 229, 242, 244], OperandSize::Qword)
}

#[test]
fn vpslld_30() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLD, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectScaledIndexed(RAX, RBX, Four, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 213, 242, 36, 152], OperandSize::Qword)
}

#[test]
fn vpslld_31() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 117, 141, 242, 247], OperandSize::Dword)
}

#[test]
fn vpslld_32() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledIndexed(EBX, ESI, Eight, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 109, 137, 242, 60, 243], OperandSize::Dword)
}

#[test]
fn vpslld_33() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLD, operand1: Some(Direct(XMM8)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM31)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 17, 77, 143, 242, 199], OperandSize::Qword)
}

#[test]
fn vpslld_34() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLD, operand1: Some(Direct(XMM18)), operand2: Some(Direct(XMM27)), operand3: Some(IndirectScaledIndexed(RCX, RAX, Four, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 225, 37, 135, 242, 20, 129], OperandSize::Qword)
}

#[test]
fn vpslld_35() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 101, 169, 242, 232], OperandSize::Dword)
}

#[test]
fn vpslld_36() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLD, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM3)), operand3: Some(Indirect(EBX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 101, 169, 242, 19], OperandSize::Dword)
}

#[test]
fn vpslld_37() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLD, operand1: Some(Direct(YMM21)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(XMM24)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 129, 77, 172, 242, 232], OperandSize::Qword)
}

#[test]
fn vpslld_38() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLD, operand1: Some(Direct(YMM25)), operand2: Some(Direct(YMM15)), operand3: Some(IndirectScaledIndexedDisplaced(RAX, RSI, Eight, 1761780850, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 97, 5, 169, 242, 140, 240, 114, 164, 2, 105], OperandSize::Qword)
}

#[test]
fn vpslld_39() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLD, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM4)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 93, 207, 242, 209], OperandSize::Dword)
}

#[test]
fn vpslld_40() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLD, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM6)), operand3: Some(IndirectScaledDisplaced(ECX, Eight, 222587193, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 77, 204, 242, 12, 205, 57, 105, 68, 13], OperandSize::Dword)
}

#[test]
fn vpslld_41() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLD, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM22)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 77, 193, 242, 241], OperandSize::Qword)
}

#[test]
fn vpslld_42() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLD, operand1: Some(Direct(ZMM15)), operand2: Some(Direct(ZMM18)), operand3: Some(IndirectScaledIndexedDisplaced(RBX, RSI, Eight, 313685801, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 113, 109, 198, 242, 188, 243, 41, 119, 178, 18], OperandSize::Qword)
}

