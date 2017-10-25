use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpsraq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAQ, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM0)), operand3: Some(Literal8(24)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 245, 138, 114, 224, 24], OperandSize::Dword)
}

#[test]
fn vpsraq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAQ, operand1: Some(Direct(XMM3)), operand2: Some(IndirectScaledDisplaced(EDX, Two, 41467064, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(94)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 229, 139, 114, 36, 85, 184, 188, 120, 2, 94], OperandSize::Dword)
}

#[test]
fn vpsraq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAQ, operand1: Some(Direct(XMM2)), operand2: Some(IndirectScaledIndexedDisplaced(ESI, ESI, Four, 1118788957, Some(OperandSize::Qword), None)), operand3: Some(Literal8(97)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 241, 237, 157, 114, 164, 182, 93, 93, 175, 66, 97], OperandSize::Dword)
}

#[test]
fn vpsraq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAQ, operand1: Some(Direct(XMM20)), operand2: Some(Direct(XMM0)), operand3: Some(Literal8(56)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 221, 135, 114, 224, 56], OperandSize::Qword)
}

#[test]
fn vpsraq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAQ, operand1: Some(Direct(XMM9)), operand2: Some(Indirect(RBX, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(75)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 181, 137, 114, 35, 75], OperandSize::Qword)
}

#[test]
fn vpsraq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAQ, operand1: Some(Direct(XMM9)), operand2: Some(Indirect(RSI, Some(OperandSize::Qword), None)), operand3: Some(Literal8(89)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 241, 181, 153, 114, 38, 89], OperandSize::Qword)
}

#[test]
fn vpsraq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAQ, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM1)), operand3: Some(Literal8(43)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 213, 169, 114, 225, 43], OperandSize::Dword)
}

#[test]
fn vpsraq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAQ, operand1: Some(Direct(YMM4)), operand2: Some(IndirectScaledIndexed(EDI, EAX, Two, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(79)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 221, 169, 114, 36, 71, 79], OperandSize::Dword)
}

#[test]
fn vpsraq_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAQ, operand1: Some(Direct(YMM1)), operand2: Some(IndirectScaledIndexedDisplaced(EBX, ECX, Two, 1066638140, Some(OperandSize::Qword), None)), operand3: Some(Literal8(78)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 245, 191, 114, 164, 75, 60, 155, 147, 63, 78], OperandSize::Dword)
}

#[test]
fn vpsraq_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAQ, operand1: Some(Direct(YMM29)), operand2: Some(Direct(YMM28)), operand3: Some(Literal8(67)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 145, 149, 166, 114, 228, 67], OperandSize::Qword)
}

#[test]
fn vpsraq_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAQ, operand1: Some(Direct(YMM14)), operand2: Some(IndirectScaledDisplaced(RSI, Two, 1656607929, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 141, 172, 114, 36, 117, 185, 212, 189, 98, 6], OperandSize::Qword)
}

#[test]
fn vpsraq_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAQ, operand1: Some(Direct(YMM22)), operand2: Some(IndirectScaledIndexedDisplaced(RCX, RDI, Two, 1136415540, Some(OperandSize::Qword), None)), operand3: Some(Literal8(4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 205, 178, 114, 164, 121, 52, 83, 188, 67, 4], OperandSize::Qword)
}

#[test]
fn vpsraq_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAQ, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM4)), operand3: Some(Literal8(98)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 237, 201, 114, 228, 98], OperandSize::Dword)
}

#[test]
fn vpsraq_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAQ, operand1: Some(Direct(ZMM2)), operand2: Some(IndirectScaledIndexed(EBX, ECX, Four, Some(OperandSize::Zmmword), None)), operand3: Some(Literal8(14)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 237, 201, 114, 36, 139, 14], OperandSize::Dword)
}

#[test]
fn vpsraq_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAQ, operand1: Some(Direct(ZMM6)), operand2: Some(IndirectScaledDisplaced(ESI, Four, 2113703696, Some(OperandSize::Qword), None)), operand3: Some(Literal8(45)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 205, 223, 114, 36, 181, 16, 143, 252, 125, 45], OperandSize::Dword)
}

#[test]
fn vpsraq_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAQ, operand1: Some(Direct(ZMM21)), operand2: Some(Direct(ZMM15)), operand3: Some(Literal8(47)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 209, 213, 198, 114, 231, 47], OperandSize::Qword)
}

#[test]
fn vpsraq_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAQ, operand1: Some(Direct(ZMM25)), operand2: Some(IndirectScaledIndexedDisplaced(RSI, RAX, Eight, 836786687, Some(OperandSize::Zmmword), None)), operand3: Some(Literal8(119)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 181, 193, 114, 164, 198, 255, 89, 224, 49, 119], OperandSize::Qword)
}

#[test]
fn vpsraq_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAQ, operand1: Some(Direct(ZMM2)), operand2: Some(IndirectDisplaced(RSI, 980186219, Some(OperandSize::Qword), None)), operand3: Some(Literal8(31)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 237, 217, 114, 166, 107, 116, 108, 58, 31], OperandSize::Qword)
}

#[test]
fn vpsraq_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAQ, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 229, 140, 226, 226], OperandSize::Dword)
}

#[test]
fn vpsraq_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAQ, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledIndexed(EAX, EDI, Two, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 237, 138, 226, 60, 120], OperandSize::Dword)
}

#[test]
fn vpsraq_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAQ, operand1: Some(Direct(XMM10)), operand2: Some(Direct(XMM29)), operand3: Some(Direct(XMM19)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 49, 149, 129, 226, 211], OperandSize::Qword)
}

#[test]
fn vpsraq_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAQ, operand1: Some(Direct(XMM24)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledIndexedDisplaced(RBX, RDX, Two, 507791950, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 97, 237, 137, 226, 132, 83, 78, 74, 68, 30], OperandSize::Qword)
}

#[test]
fn vpsraq_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAQ, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 197, 171, 226, 219], OperandSize::Dword)
}

#[test]
fn vpsraq_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAQ, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectScaledIndexedDisplaced(EDI, ECX, Eight, 2005839612, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 253, 171, 226, 132, 207, 252, 174, 142, 119], OperandSize::Dword)
}

#[test]
fn vpsraq_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAQ, operand1: Some(Direct(YMM11)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 113, 213, 169, 226, 218], OperandSize::Qword)
}

#[test]
fn vpsraq_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAQ, operand1: Some(Direct(YMM11)), operand2: Some(Direct(YMM11)), operand3: Some(IndirectDisplaced(RBX, 338482843, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 113, 165, 172, 226, 155, 155, 214, 44, 20], OperandSize::Qword)
}

#[test]
fn vpsraq_27() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAQ, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM7)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 197, 202, 226, 224], OperandSize::Dword)
}

#[test]
fn vpsraq_28() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAQ, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM5)), operand3: Some(IndirectScaledIndexedDisplaced(ESI, ESI, Two, 1677870364, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 213, 206, 226, 164, 118, 28, 69, 2, 100], OperandSize::Dword)
}

#[test]
fn vpsraq_29() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAQ, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM4)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 221, 204, 226, 234], OperandSize::Qword)
}

#[test]
fn vpsraq_30() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAQ, operand1: Some(Direct(ZMM19)), operand2: Some(Direct(ZMM29)), operand3: Some(IndirectScaledIndexedDisplaced(RBX, RSI, Two, 1938231283, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 225, 149, 197, 226, 156, 115, 243, 15, 135, 115], OperandSize::Qword)
}

