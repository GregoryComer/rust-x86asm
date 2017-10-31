use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpsraq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAQ, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM0)), operand3: Some(Literal8(127)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 253, 142, 114, 224, 127], OperandSize::Dword)
}

#[test]
fn vpsraq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAQ, operand1: Some(Direct(XMM7)), operand2: Some(IndirectScaledIndexedDisplaced(EBX, EDI, Four, 236059692, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(24)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 197, 140, 114, 164, 187, 44, 252, 17, 14, 24], OperandSize::Dword)
}

#[test]
fn vpsraq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAQ, operand1: Some(Direct(XMM0)), operand2: Some(Indirect(EDX, Some(OperandSize::Qword), None)), operand3: Some(Literal8(19)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 241, 253, 153, 114, 34, 19], OperandSize::Dword)
}

#[test]
fn vpsraq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAQ, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM15)), operand3: Some(Literal8(37)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 209, 245, 138, 114, 231, 37], OperandSize::Qword)
}

#[test]
fn vpsraq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAQ, operand1: Some(Direct(XMM2)), operand2: Some(IndirectDisplaced(RSI, 1985333530, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 237, 138, 114, 166, 26, 201, 85, 118, 4], OperandSize::Qword)
}

#[test]
fn vpsraq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAQ, operand1: Some(Direct(XMM20)), operand2: Some(IndirectDisplaced(RDX, 150193390, Some(OperandSize::Qword), None)), operand3: Some(Literal8(110)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 241, 221, 145, 114, 162, 238, 196, 243, 8, 110], OperandSize::Qword)
}

#[test]
fn vpsraq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAQ, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM3)), operand3: Some(Literal8(113)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 229, 172, 114, 227, 113], OperandSize::Dword)
}

#[test]
fn vpsraq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAQ, operand1: Some(Direct(YMM1)), operand2: Some(IndirectScaledIndexedDisplaced(ESI, EAX, Four, 2137196404, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(30)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 245, 173, 114, 164, 134, 116, 7, 99, 127, 30], OperandSize::Dword)
}

#[test]
fn vpsraq_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAQ, operand1: Some(Direct(YMM5)), operand2: Some(IndirectScaledDisplaced(EDI, Two, 945291295, Some(OperandSize::Qword), None)), operand3: Some(Literal8(41)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 213, 185, 114, 36, 125, 31, 0, 88, 56, 41], OperandSize::Dword)
}

#[test]
fn vpsraq_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAQ, operand1: Some(Direct(YMM9)), operand2: Some(Direct(YMM26)), operand3: Some(Literal8(122)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 145, 181, 172, 114, 226, 122], OperandSize::Qword)
}

#[test]
fn vpsraq_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAQ, operand1: Some(Direct(YMM5)), operand2: Some(IndirectScaledDisplaced(RSI, Four, 861462620, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(40)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 213, 174, 114, 36, 181, 92, 224, 88, 51, 40], OperandSize::Qword)
}

#[test]
fn vpsraq_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAQ, operand1: Some(Direct(YMM11)), operand2: Some(IndirectScaledIndexed(RBX, RSI, Eight, Some(OperandSize::Qword), None)), operand3: Some(Literal8(65)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 165, 190, 114, 36, 243, 65], OperandSize::Qword)
}

#[test]
fn vpsraq_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAQ, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM1)), operand3: Some(Literal8(35)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 237, 206, 114, 225, 35], OperandSize::Dword)
}

#[test]
fn vpsraq_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAQ, operand1: Some(Direct(ZMM7)), operand2: Some(IndirectScaledIndexed(EDI, EDI, Eight, Some(OperandSize::Zmmword), None)), operand3: Some(Literal8(46)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 197, 207, 114, 36, 255, 46], OperandSize::Dword)
}

#[test]
fn vpsraq_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAQ, operand1: Some(Direct(ZMM7)), operand2: Some(IndirectDisplaced(EBX, 789808269, Some(OperandSize::Qword), None)), operand3: Some(Literal8(122)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 197, 223, 114, 163, 141, 132, 19, 47, 122], OperandSize::Dword)
}

#[test]
fn vpsraq_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAQ, operand1: Some(Direct(ZMM25)), operand2: Some(Direct(ZMM13)), operand3: Some(Literal8(15)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 209, 181, 194, 114, 229, 15], OperandSize::Qword)
}

#[test]
fn vpsraq_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAQ, operand1: Some(Direct(ZMM31)), operand2: Some(Indirect(RCX, Some(OperandSize::Zmmword), None)), operand3: Some(Literal8(101)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 133, 199, 114, 33, 101], OperandSize::Qword)
}

#[test]
fn vpsraq_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAQ, operand1: Some(Direct(ZMM23)), operand2: Some(IndirectDisplaced(RDI, 1144191467, Some(OperandSize::Qword), None)), operand3: Some(Literal8(9)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 197, 215, 114, 167, 235, 249, 50, 68, 9], OperandSize::Qword)
}

#[test]
fn vpsraq_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAQ, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 197, 140, 226, 192], OperandSize::Dword)
}

#[test]
fn vpsraq_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAQ, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledIndexedDisplaced(EBX, EAX, Eight, 1878493304, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 237, 140, 226, 140, 195, 120, 136, 247, 111], OperandSize::Dword)
}

#[test]
fn vpsraq_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAQ, operand1: Some(Direct(XMM13)), operand2: Some(Direct(XMM21)), operand3: Some(Direct(XMM23)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 49, 213, 130, 226, 239], OperandSize::Qword)
}

#[test]
fn vpsraq_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAQ, operand1: Some(Direct(XMM18)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectDisplaced(RSI, 1918526915, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 225, 221, 142, 226, 150, 195, 101, 90, 114], OperandSize::Qword)
}

#[test]
fn vpsraq_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAQ, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 253, 169, 226, 194], OperandSize::Dword)
}

#[test]
fn vpsraq_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAQ, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM7)), operand3: Some(Indirect(ECX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 197, 170, 226, 49], OperandSize::Dword)
}

#[test]
fn vpsraq_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAQ, operand1: Some(Direct(YMM23)), operand2: Some(Direct(YMM9)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 225, 181, 172, 226, 254], OperandSize::Qword)
}

#[test]
fn vpsraq_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAQ, operand1: Some(Direct(YMM30)), operand2: Some(Direct(YMM24)), operand3: Some(IndirectDisplaced(RDI, 111762161, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 97, 189, 167, 226, 183, 241, 90, 169, 6], OperandSize::Qword)
}

#[test]
fn vpsraq_27() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAQ, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM4)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 221, 204, 226, 212], OperandSize::Dword)
}

#[test]
fn vpsraq_28() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAQ, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM6)), operand3: Some(IndirectDisplaced(EDI, 1306663728, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 205, 205, 226, 191, 48, 27, 226, 77], OperandSize::Dword)
}

#[test]
fn vpsraq_29() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAQ, operand1: Some(Direct(ZMM8)), operand2: Some(Direct(ZMM15)), operand3: Some(Direct(XMM12)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 81, 133, 201, 226, 196], OperandSize::Qword)
}

#[test]
fn vpsraq_30() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAQ, operand1: Some(Direct(ZMM25)), operand2: Some(Direct(ZMM28)), operand3: Some(IndirectDisplaced(RBX, 16443481, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 97, 157, 196, 226, 139, 89, 232, 250, 0], OperandSize::Qword)
}

