use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpsraq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAQ, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM5)), operand3: Some(Literal8(47)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 221, 139, 114, 229, 47], OperandSize::Dword)
}

#[test]
fn vpsraq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAQ, operand1: Some(Direct(XMM5)), operand2: Some(IndirectScaledIndexed(ESI, EDI, Eight, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(29)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 213, 141, 114, 36, 254, 29], OperandSize::Dword)
}

#[test]
fn vpsraq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAQ, operand1: Some(Direct(XMM5)), operand2: Some(IndirectDisplaced(ESI, 1474275717, Some(OperandSize::Qword), None)), operand3: Some(Literal8(6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 241, 213, 159, 114, 166, 133, 169, 223, 87, 6], OperandSize::Dword)
}

#[test]
fn vpsraq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAQ, operand1: Some(Direct(XMM27)), operand2: Some(Direct(XMM21)), operand3: Some(Literal8(53)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 177, 165, 133, 114, 229, 53], OperandSize::Qword)
}

#[test]
fn vpsraq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAQ, operand1: Some(Direct(XMM27)), operand2: Some(IndirectScaledDisplaced(RDI, Four, 622952536, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(64)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 165, 134, 114, 36, 189, 88, 128, 33, 37, 64], OperandSize::Qword)
}

#[test]
fn vpsraq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAQ, operand1: Some(Direct(XMM7)), operand2: Some(IndirectDisplaced(RCX, 1049695466, Some(OperandSize::Qword), None)), operand3: Some(Literal8(55)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 241, 197, 159, 114, 161, 234, 20, 145, 62, 55], OperandSize::Qword)
}

#[test]
fn vpsraq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAQ, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM5)), operand3: Some(Literal8(27)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 253, 175, 114, 229, 27], OperandSize::Dword)
}

#[test]
fn vpsraq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAQ, operand1: Some(Direct(YMM6)), operand2: Some(IndirectScaledIndexedDisplaced(EBX, EAX, Two, 734963169, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(92)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 205, 173, 114, 164, 67, 225, 165, 206, 43, 92], OperandSize::Dword)
}

#[test]
fn vpsraq_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAQ, operand1: Some(Direct(YMM0)), operand2: Some(IndirectScaledIndexedDisplaced(EDI, ECX, Eight, 584253461, Some(OperandSize::Qword), None)), operand3: Some(Literal8(83)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 253, 187, 114, 164, 207, 21, 0, 211, 34, 83], OperandSize::Dword)
}

#[test]
fn vpsraq_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAQ, operand1: Some(Direct(YMM9)), operand2: Some(Direct(YMM21)), operand3: Some(Literal8(92)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 177, 181, 174, 114, 229, 92], OperandSize::Qword)
}

#[test]
fn vpsraq_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAQ, operand1: Some(Direct(YMM21)), operand2: Some(IndirectScaledDisplaced(RCX, Two, 93869921, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(56)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 213, 164, 114, 36, 77, 97, 87, 152, 5, 56], OperandSize::Qword)
}

#[test]
fn vpsraq_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAQ, operand1: Some(Direct(YMM10)), operand2: Some(IndirectScaledIndexedDisplaced(RDI, RDX, Eight, 2056926539, Some(OperandSize::Qword), None)), operand3: Some(Literal8(90)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 173, 186, 114, 164, 215, 75, 53, 154, 122, 90], OperandSize::Qword)
}

#[test]
fn vpsraq_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAQ, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM6)), operand3: Some(Literal8(79)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 229, 205, 114, 230, 79], OperandSize::Dword)
}

#[test]
fn vpsraq_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAQ, operand1: Some(Direct(ZMM5)), operand2: Some(IndirectScaledDisplaced(ECX, Eight, 1965464237, Some(OperandSize::Zmmword), None)), operand3: Some(Literal8(7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 213, 207, 114, 36, 205, 173, 154, 38, 117, 7], OperandSize::Dword)
}

#[test]
fn vpsraq_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAQ, operand1: Some(Direct(ZMM3)), operand2: Some(IndirectScaledDisplaced(EBX, Four, 146606606, Some(OperandSize::Qword), None)), operand3: Some(Literal8(26)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 229, 220, 114, 36, 157, 14, 10, 189, 8, 26], OperandSize::Dword)
}

#[test]
fn vpsraq_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAQ, operand1: Some(Direct(ZMM12)), operand2: Some(Direct(ZMM27)), operand3: Some(Literal8(108)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 145, 157, 205, 114, 227, 108], OperandSize::Qword)
}

#[test]
fn vpsraq_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAQ, operand1: Some(Direct(ZMM2)), operand2: Some(IndirectScaledIndexed(RDI, RSI, Four, Some(OperandSize::Zmmword), None)), operand3: Some(Literal8(36)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 237, 206, 114, 36, 183, 36], OperandSize::Qword)
}

#[test]
fn vpsraq_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAQ, operand1: Some(Direct(ZMM18)), operand2: Some(IndirectDisplaced(RCX, 55164438, Some(OperandSize::Qword), None)), operand3: Some(Literal8(33)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 237, 215, 114, 161, 22, 190, 73, 3, 33], OperandSize::Qword)
}

#[test]
fn vpsraq_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAQ, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 229, 142, 226, 252], OperandSize::Dword)
}

#[test]
fn vpsraq_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAQ, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectScaledIndexed(EDX, EBX, Two, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 205, 142, 226, 4, 90], OperandSize::Dword)
}

#[test]
fn vpsraq_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAQ, operand1: Some(Direct(XMM20)), operand2: Some(Direct(XMM25)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 225, 181, 135, 226, 226], OperandSize::Qword)
}

#[test]
fn vpsraq_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAQ, operand1: Some(Direct(XMM29)), operand2: Some(Direct(XMM4)), operand3: Some(Indirect(RDX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 97, 221, 142, 226, 42], OperandSize::Qword)
}

#[test]
fn vpsraq_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAQ, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 245, 169, 226, 223], OperandSize::Dword)
}

#[test]
fn vpsraq_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAQ, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectScaledIndexedDisplaced(ECX, ECX, Two, 185743688, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 213, 174, 226, 188, 73, 72, 57, 18, 11], OperandSize::Dword)
}

#[test]
fn vpsraq_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAQ, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM8)), operand3: Some(Direct(XMM14)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 209, 189, 174, 226, 254], OperandSize::Qword)
}

#[test]
fn vpsraq_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAQ, operand1: Some(Direct(YMM11)), operand2: Some(Direct(YMM10)), operand3: Some(IndirectScaledIndexedDisplaced(RSI, RDI, Two, 942005740, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 113, 173, 169, 226, 156, 126, 236, 221, 37, 56], OperandSize::Qword)
}

#[test]
fn vpsraq_27() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAQ, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM3)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 229, 201, 226, 247], OperandSize::Dword)
}

#[test]
fn vpsraq_28() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAQ, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM1)), operand3: Some(IndirectScaledDisplaced(EDI, Eight, 520092596, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 245, 207, 226, 28, 253, 180, 251, 255, 30], OperandSize::Dword)
}

#[test]
fn vpsraq_29() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAQ, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM10)), operand3: Some(Direct(XMM12)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 209, 173, 206, 226, 228], OperandSize::Qword)
}

#[test]
fn vpsraq_30() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAQ, operand1: Some(Direct(ZMM9)), operand2: Some(Direct(ZMM11)), operand3: Some(IndirectDisplaced(RAX, 1333995402, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 113, 165, 205, 226, 136, 138, 39, 131, 79], OperandSize::Qword)
}

