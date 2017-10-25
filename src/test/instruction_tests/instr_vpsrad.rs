use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpsrad_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM7)), operand3: Some(Literal8(50)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 114, 231, 50], OperandSize::Dword)
}

#[test]
fn vpsrad_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM5)), operand3: Some(Literal8(45)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 201, 114, 229, 45], OperandSize::Qword)
}

#[test]
fn vpsrad_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAD, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM2)), operand3: Some(Literal8(119)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 221, 114, 226, 119], OperandSize::Dword)
}

#[test]
fn vpsrad_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAD, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM6)), operand3: Some(Literal8(5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 114, 230, 5], OperandSize::Qword)
}

#[test]
fn vpsrad_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM2)), operand3: Some(Literal8(14)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 117, 142, 114, 226, 14], OperandSize::Dword)
}

#[test]
fn vpsrad_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAD, operand1: Some(Direct(XMM7)), operand2: Some(IndirectScaledIndexedDisplaced(EBX, EDX, Eight, 1632238316, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(114)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 69, 138, 114, 164, 211, 236, 250, 73, 97, 114], OperandSize::Dword)
}

#[test]
fn vpsrad_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAD, operand1: Some(Direct(XMM3)), operand2: Some(Indirect(EDX, Some(OperandSize::Dword), None)), operand3: Some(Literal8(109)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 101, 153, 114, 34, 109], OperandSize::Dword)
}

#[test]
fn vpsrad_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM0)), operand3: Some(Literal8(39)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 125, 141, 114, 224, 39], OperandSize::Qword)
}

#[test]
fn vpsrad_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAD, operand1: Some(Direct(XMM5)), operand2: Some(IndirectScaledIndexed(RSI, RDX, Eight, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(86)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 85, 139, 114, 36, 214, 86], OperandSize::Qword)
}

#[test]
fn vpsrad_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAD, operand1: Some(Direct(XMM2)), operand2: Some(IndirectScaledIndexed(RAX, RBX, Eight, Some(OperandSize::Dword), None)), operand3: Some(Literal8(120)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 109, 157, 114, 36, 216, 120], OperandSize::Qword)
}

#[test]
fn vpsrad_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM3)), operand3: Some(Literal8(4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 69, 173, 114, 227, 4], OperandSize::Dword)
}

#[test]
fn vpsrad_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAD, operand1: Some(Direct(YMM4)), operand2: Some(IndirectDisplaced(EDI, 752034718, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(69)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 93, 169, 114, 167, 158, 35, 211, 44, 69], OperandSize::Dword)
}

#[test]
fn vpsrad_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAD, operand1: Some(Direct(YMM2)), operand2: Some(IndirectScaledIndexed(ESI, EAX, Two, Some(OperandSize::Dword), None)), operand3: Some(Literal8(117)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 109, 191, 114, 36, 70, 117], OperandSize::Dword)
}

#[test]
fn vpsrad_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAD, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM15)), operand3: Some(Literal8(108)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 209, 117, 172, 114, 231, 108], OperandSize::Qword)
}

#[test]
fn vpsrad_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAD, operand1: Some(Direct(YMM6)), operand2: Some(Indirect(RDI, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(107)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 77, 170, 114, 39, 107], OperandSize::Qword)
}

#[test]
fn vpsrad_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAD, operand1: Some(Direct(YMM4)), operand2: Some(IndirectScaledIndexed(RCX, RAX, Eight, Some(OperandSize::Dword), None)), operand3: Some(Literal8(97)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 93, 190, 114, 36, 193, 97], OperandSize::Qword)
}

#[test]
fn vpsrad_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAD, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM7)), operand3: Some(Literal8(124)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 77, 202, 114, 231, 124], OperandSize::Dword)
}

#[test]
fn vpsrad_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAD, operand1: Some(Direct(ZMM3)), operand2: Some(IndirectScaledDisplaced(EAX, Four, 268879336, Some(OperandSize::Zmmword), None)), operand3: Some(Literal8(98)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 101, 204, 114, 36, 133, 232, 197, 6, 16, 98], OperandSize::Dword)
}

#[test]
fn vpsrad_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAD, operand1: Some(Direct(ZMM5)), operand2: Some(IndirectScaledDisplaced(EAX, Two, 1198797636, Some(OperandSize::Dword), None)), operand3: Some(Literal8(92)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 241, 85, 218, 114, 36, 69, 68, 51, 116, 71, 92], OperandSize::Dword)
}

#[test]
fn vpsrad_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAD, operand1: Some(Direct(ZMM16)), operand2: Some(Direct(ZMM6)), operand3: Some(Literal8(118)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 125, 193, 114, 230, 118], OperandSize::Qword)
}

#[test]
fn vpsrad_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAD, operand1: Some(Direct(ZMM19)), operand2: Some(IndirectScaledIndexed(RDI, RCX, Four, Some(OperandSize::Zmmword), None)), operand3: Some(Literal8(6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 101, 193, 114, 36, 143, 6], OperandSize::Qword)
}

#[test]
fn vpsrad_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAD, operand1: Some(Direct(ZMM23)), operand2: Some(IndirectScaledIndexed(RCX, RCX, Four, Some(OperandSize::Dword), None)), operand3: Some(Literal8(48)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 241, 69, 210, 114, 36, 137, 48], OperandSize::Qword)
}

#[test]
fn vpsrad_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 217, 226, 219], OperandSize::Dword)
}

#[test]
fn vpsrad_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectScaledIndexed(EAX, ECX, Eight, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 241, 226, 44, 200], OperandSize::Dword)
}

#[test]
fn vpsrad_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 233, 226, 255], OperandSize::Qword)
}

#[test]
fn vpsrad_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectScaledIndexed(RAX, RBX, Four, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 241, 226, 28, 152], OperandSize::Qword)
}

#[test]
fn vpsrad_27() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAD, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 245, 226, 246], OperandSize::Dword)
}

#[test]
fn vpsrad_28() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAD, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectScaledIndexed(EDX, EDI, Two, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 229, 226, 12, 122], OperandSize::Dword)
}

#[test]
fn vpsrad_29() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAD, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 245, 226, 199], OperandSize::Qword)
}

#[test]
fn vpsrad_30() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAD, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectDisplaced(RSI, 258578497, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 245, 226, 150, 65, 152, 105, 15], OperandSize::Qword)
}

#[test]
fn vpsrad_31() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 77, 143, 226, 223], OperandSize::Dword)
}

#[test]
fn vpsrad_32() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledIndexed(EBX, ESI, Two, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 85, 138, 226, 12, 115], OperandSize::Dword)
}

#[test]
fn vpsrad_33() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAD, operand1: Some(Direct(XMM22)), operand2: Some(Direct(XMM12)), operand3: Some(Direct(XMM17)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 161, 29, 140, 226, 241], OperandSize::Qword)
}

#[test]
fn vpsrad_34() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAD, operand1: Some(Direct(XMM17)), operand2: Some(Direct(XMM13)), operand3: Some(IndirectScaledIndexedDisplaced(RAX, RAX, Eight, 261026055, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 225, 21, 138, 226, 140, 192, 7, 241, 142, 15], OperandSize::Qword)
}

#[test]
fn vpsrad_35() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 93, 173, 226, 250], OperandSize::Dword)
}

#[test]
fn vpsrad_36() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAD, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM5)), operand3: Some(Indirect(EDI, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 85, 171, 226, 7], OperandSize::Dword)
}

#[test]
fn vpsrad_37() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAD, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM16)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 125, 166, 226, 218], OperandSize::Qword)
}

#[test]
fn vpsrad_38() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAD, operand1: Some(Direct(YMM8)), operand2: Some(Direct(YMM26)), operand3: Some(IndirectScaledDisplaced(RDI, Four, 1591101632, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 113, 45, 166, 226, 4, 189, 192, 72, 214, 94], OperandSize::Qword)
}

#[test]
fn vpsrad_39() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAD, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM0)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 125, 206, 226, 231], OperandSize::Dword)
}

#[test]
fn vpsrad_40() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAD, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM2)), operand3: Some(IndirectScaledDisplaced(ECX, Four, 220045260, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 109, 207, 226, 4, 141, 204, 159, 29, 13], OperandSize::Dword)
}

#[test]
fn vpsrad_41() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAD, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM7)), operand3: Some(Direct(XMM18)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 177, 69, 202, 226, 218], OperandSize::Qword)
}

#[test]
fn vpsrad_42() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAD, operand1: Some(Direct(ZMM13)), operand2: Some(Direct(ZMM31)), operand3: Some(IndirectScaledIndexedDisplaced(RCX, RAX, Eight, 656221215, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 113, 5, 199, 226, 172, 193, 31, 36, 29, 39], OperandSize::Qword)
}

