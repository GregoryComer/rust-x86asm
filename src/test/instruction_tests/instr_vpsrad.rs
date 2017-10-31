use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpsrad_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM0)), operand3: Some(Literal8(74)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 241, 114, 224, 74], OperandSize::Dword)
}

#[test]
fn vpsrad_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM5)), operand3: Some(Literal8(114)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 209, 114, 229, 114], OperandSize::Qword)
}

#[test]
fn vpsrad_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM3)), operand3: Some(Literal8(109)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 197, 114, 227, 109], OperandSize::Dword)
}

#[test]
fn vpsrad_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAD, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM6)), operand3: Some(Literal8(57)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 205, 114, 230, 57], OperandSize::Qword)
}

#[test]
fn vpsrad_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM7)), operand3: Some(Literal8(39)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 109, 138, 114, 231, 39], OperandSize::Dword)
}

#[test]
fn vpsrad_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAD, operand1: Some(Direct(XMM5)), operand2: Some(IndirectScaledIndexed(ESI, EDX, Eight, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(39)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 85, 138, 114, 36, 214, 39], OperandSize::Dword)
}

#[test]
fn vpsrad_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAD, operand1: Some(Direct(XMM7)), operand2: Some(IndirectDisplaced(EAX, 4959309, Some(OperandSize::Dword), None)), operand3: Some(Literal8(36)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 69, 155, 114, 160, 77, 172, 75, 0, 36], OperandSize::Dword)
}

#[test]
fn vpsrad_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAD, operand1: Some(Direct(XMM11)), operand2: Some(Direct(XMM7)), operand3: Some(Literal8(94)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 37, 143, 114, 231, 94], OperandSize::Qword)
}

#[test]
fn vpsrad_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAD, operand1: Some(Direct(XMM23)), operand2: Some(IndirectDisplaced(RSI, 937813044, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(81)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 69, 135, 114, 166, 52, 228, 229, 55, 81], OperandSize::Qword)
}

#[test]
fn vpsrad_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAD, operand1: Some(Direct(XMM22)), operand2: Some(IndirectDisplaced(RDI, 1543202164, Some(OperandSize::Dword), None)), operand3: Some(Literal8(33)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 77, 150, 114, 167, 116, 101, 251, 91, 33], OperandSize::Qword)
}

#[test]
fn vpsrad_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM3)), operand3: Some(Literal8(121)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 69, 175, 114, 227, 121], OperandSize::Dword)
}

#[test]
fn vpsrad_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAD, operand1: Some(Direct(YMM7)), operand2: Some(IndirectScaledDisplaced(EAX, Eight, 1620915170, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(124)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 69, 175, 114, 36, 197, 226, 51, 157, 96, 124], OperandSize::Dword)
}

#[test]
fn vpsrad_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAD, operand1: Some(Direct(YMM6)), operand2: Some(IndirectDisplaced(ESI, 2034038083, Some(OperandSize::Dword), None)), operand3: Some(Literal8(9)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 77, 185, 114, 166, 67, 245, 60, 121, 9], OperandSize::Dword)
}

#[test]
fn vpsrad_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAD, operand1: Some(Direct(YMM24)), operand2: Some(Direct(YMM11)), operand3: Some(Literal8(105)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 209, 61, 162, 114, 227, 105], OperandSize::Qword)
}

#[test]
fn vpsrad_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAD, operand1: Some(Direct(YMM15)), operand2: Some(Indirect(RDI, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(65)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 5, 173, 114, 39, 65], OperandSize::Qword)
}

#[test]
fn vpsrad_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAD, operand1: Some(Direct(YMM15)), operand2: Some(Indirect(RBX, Some(OperandSize::Dword), None)), operand3: Some(Literal8(94)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 5, 186, 114, 35, 94], OperandSize::Qword)
}

#[test]
fn vpsrad_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAD, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM4)), operand3: Some(Literal8(71)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 85, 204, 114, 228, 71], OperandSize::Dword)
}

#[test]
fn vpsrad_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAD, operand1: Some(Direct(ZMM2)), operand2: Some(Indirect(ECX, Some(OperandSize::Zmmword), None)), operand3: Some(Literal8(5)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 109, 204, 114, 33, 5], OperandSize::Dword)
}

#[test]
fn vpsrad_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAD, operand1: Some(Direct(ZMM7)), operand2: Some(IndirectDisplaced(EDX, 1487398550, Some(OperandSize::Dword), None)), operand3: Some(Literal8(92)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 241, 69, 219, 114, 162, 150, 230, 167, 88, 92], OperandSize::Dword)
}

#[test]
fn vpsrad_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAD, operand1: Some(Direct(ZMM28)), operand2: Some(Direct(ZMM20)), operand3: Some(Literal8(47)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 177, 29, 199, 114, 228, 47], OperandSize::Qword)
}

#[test]
fn vpsrad_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAD, operand1: Some(Direct(ZMM3)), operand2: Some(Indirect(RSI, Some(OperandSize::Zmmword), None)), operand3: Some(Literal8(76)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 101, 203, 114, 38, 76], OperandSize::Qword)
}

#[test]
fn vpsrad_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAD, operand1: Some(Direct(ZMM14)), operand2: Some(IndirectScaledIndexedDisplaced(RBX, RSI, Eight, 2067224097, Some(OperandSize::Dword), None)), operand3: Some(Literal8(120)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 241, 13, 222, 114, 164, 243, 33, 86, 55, 123, 120], OperandSize::Qword)
}

#[test]
fn vpsrad_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 241, 226, 212], OperandSize::Dword)
}

#[test]
fn vpsrad_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledDisplaced(ESI, Four, 767634205, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 226, 44, 181, 29, 43, 193, 45], OperandSize::Dword)
}

#[test]
fn vpsrad_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 193, 226, 217], OperandSize::Qword)
}

#[test]
fn vpsrad_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledIndexed(RSI, RDX, Two, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 225, 226, 4, 86], OperandSize::Qword)
}

#[test]
fn vpsrad_27() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 237, 226, 239], OperandSize::Dword)
}

#[test]
fn vpsrad_28() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAD, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM3)), operand3: Some(Indirect(ESI, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 229, 226, 14], OperandSize::Dword)
}

#[test]
fn vpsrad_29() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 197, 226, 248], OperandSize::Qword)
}

#[test]
fn vpsrad_30() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAD, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectDisplaced(RDX, 1837681348, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 197, 226, 178, 196, 202, 136, 109], OperandSize::Qword)
}

#[test]
fn vpsrad_31() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 117, 137, 226, 208], OperandSize::Dword)
}

#[test]
fn vpsrad_32() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectDisplaced(EAX, 61129016, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 93, 141, 226, 128, 56, 193, 164, 3], OperandSize::Dword)
}

#[test]
fn vpsrad_33() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAD, operand1: Some(Direct(XMM28)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 97, 125, 143, 226, 225], OperandSize::Qword)
}

#[test]
fn vpsrad_34() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM30)), operand3: Some(Indirect(RDX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 13, 133, 226, 26], OperandSize::Qword)
}

#[test]
fn vpsrad_35() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAD, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 101, 175, 226, 197], OperandSize::Dword)
}

#[test]
fn vpsrad_36() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAD, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM3)), operand3: Some(Indirect(ECX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 101, 170, 226, 25], OperandSize::Dword)
}

#[test]
fn vpsrad_37() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAD, operand1: Some(Direct(YMM18)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(XMM21)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 161, 109, 172, 226, 213], OperandSize::Qword)
}

#[test]
fn vpsrad_38() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAD, operand1: Some(Direct(YMM24)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectScaledDisplaced(RCX, Eight, 376431801, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 97, 101, 175, 226, 4, 205, 185, 228, 111, 22], OperandSize::Qword)
}

#[test]
fn vpsrad_39() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAD, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM5)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 85, 203, 226, 236], OperandSize::Dword)
}

#[test]
fn vpsrad_40() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAD, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM1)), operand3: Some(IndirectScaledIndexedDisplaced(ESI, EBX, Two, 1892625417, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 117, 201, 226, 180, 94, 9, 44, 207, 112], OperandSize::Dword)
}

#[test]
fn vpsrad_41() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAD, operand1: Some(Direct(ZMM21)), operand2: Some(Direct(ZMM20)), operand3: Some(Direct(XMM28)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 129, 93, 195, 226, 236], OperandSize::Qword)
}

#[test]
fn vpsrad_42() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAD, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM9)), operand3: Some(IndirectDisplaced(RCX, 1610330156, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 53, 201, 226, 177, 44, 176, 251, 95], OperandSize::Qword)
}

