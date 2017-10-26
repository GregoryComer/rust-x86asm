use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpsraq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAQ, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM6)), operand3: Some(Literal8(29)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 205, 142, 114, 230, 29], OperandSize::Dword)
}

#[test]
fn vpsraq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAQ, operand1: Some(Direct(XMM6)), operand2: Some(IndirectScaledDisplaced(EDX, Four, 1722990180, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(94)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 205, 140, 114, 36, 149, 100, 190, 178, 102, 94], OperandSize::Dword)
}

#[test]
fn vpsraq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAQ, operand1: Some(Direct(XMM2)), operand2: Some(IndirectDisplaced(ECX, 1609240740, Some(OperandSize::Qword), None)), operand3: Some(Literal8(6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 241, 237, 155, 114, 161, 164, 16, 235, 95, 6], OperandSize::Dword)
}

#[test]
fn vpsraq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAQ, operand1: Some(Direct(XMM17)), operand2: Some(Direct(XMM4)), operand3: Some(Literal8(67)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 245, 130, 114, 228, 67], OperandSize::Qword)
}

#[test]
fn vpsraq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAQ, operand1: Some(Direct(XMM0)), operand2: Some(IndirectScaledIndexedDisplaced(RCX, RDX, Two, 1388920317, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(93)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 253, 139, 114, 164, 81, 253, 61, 201, 82, 93], OperandSize::Qword)
}

#[test]
fn vpsraq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAQ, operand1: Some(Direct(XMM26)), operand2: Some(IndirectScaledDisplaced(RSI, Two, 1176915004, Some(OperandSize::Qword), None)), operand3: Some(Literal8(76)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 241, 173, 146, 114, 36, 117, 60, 76, 38, 70, 76], OperandSize::Qword)
}

#[test]
fn vpsraq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAQ, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM3)), operand3: Some(Literal8(46)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 221, 169, 114, 227, 46], OperandSize::Dword)
}

#[test]
fn vpsraq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAQ, operand1: Some(Direct(YMM0)), operand2: Some(Indirect(EBX, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(126)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 253, 169, 114, 35, 126], OperandSize::Dword)
}

#[test]
fn vpsraq_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAQ, operand1: Some(Direct(YMM5)), operand2: Some(IndirectScaledDisplaced(EDI, Four, 470905832, Some(OperandSize::Qword), None)), operand3: Some(Literal8(15)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 213, 185, 114, 36, 189, 232, 115, 17, 28, 15], OperandSize::Dword)
}

#[test]
fn vpsraq_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAQ, operand1: Some(Direct(YMM12)), operand2: Some(Direct(YMM29)), operand3: Some(Literal8(79)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 145, 157, 169, 114, 229, 79], OperandSize::Qword)
}

#[test]
fn vpsraq_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAQ, operand1: Some(Direct(YMM3)), operand2: Some(IndirectScaledIndexedDisplaced(RDX, RAX, Two, 323508453, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(116)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 229, 174, 114, 164, 66, 229, 88, 72, 19, 116], OperandSize::Qword)
}

#[test]
fn vpsraq_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAQ, operand1: Some(Direct(YMM23)), operand2: Some(IndirectScaledIndexedDisplaced(RSI, RBX, Four, 867341099, Some(OperandSize::Qword), None)), operand3: Some(Literal8(60)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 197, 180, 114, 164, 158, 43, 147, 178, 51, 60], OperandSize::Qword)
}

#[test]
fn vpsraq_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAQ, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM6)), operand3: Some(Literal8(2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 205, 206, 114, 230, 2], OperandSize::Dword)
}

#[test]
fn vpsraq_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAQ, operand1: Some(Direct(ZMM4)), operand2: Some(IndirectScaledDisplaced(EAX, Four, 335788972, Some(OperandSize::Zmmword), None)), operand3: Some(Literal8(61)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 221, 206, 114, 36, 133, 172, 187, 3, 20, 61], OperandSize::Dword)
}

#[test]
fn vpsraq_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAQ, operand1: Some(Direct(ZMM5)), operand2: Some(IndirectScaledIndexedDisplaced(ESI, EBX, Two, 1655979427, Some(OperandSize::Qword), None)), operand3: Some(Literal8(38)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 213, 217, 114, 164, 94, 163, 61, 180, 98, 38], OperandSize::Dword)
}

#[test]
fn vpsraq_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAQ, operand1: Some(Direct(ZMM17)), operand2: Some(Direct(ZMM29)), operand3: Some(Literal8(121)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 145, 245, 196, 114, 229, 121], OperandSize::Qword)
}

#[test]
fn vpsraq_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAQ, operand1: Some(Direct(ZMM26)), operand2: Some(IndirectScaledIndexedDisplaced(RDX, RCX, Two, 443524211, Some(OperandSize::Zmmword), None)), operand3: Some(Literal8(76)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 173, 195, 114, 164, 74, 115, 164, 111, 26, 76], OperandSize::Qword)
}

#[test]
fn vpsraq_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAQ, operand1: Some(Direct(ZMM30)), operand2: Some(IndirectDisplaced(RBX, 884138378, Some(OperandSize::Qword), None)), operand3: Some(Literal8(11)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 141, 213, 114, 163, 138, 225, 178, 52, 11], OperandSize::Qword)
}

#[test]
fn vpsraq_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAQ, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 205, 137, 226, 241], OperandSize::Dword)
}

#[test]
fn vpsraq_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAQ, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM1)), operand3: Some(Indirect(ECX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 245, 143, 226, 49], OperandSize::Dword)
}

#[test]
fn vpsraq_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAQ, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM18)), operand3: Some(Direct(XMM22)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 177, 237, 131, 226, 222], OperandSize::Qword)
}

#[test]
fn vpsraq_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAQ, operand1: Some(Direct(XMM19)), operand2: Some(Direct(XMM31)), operand3: Some(IndirectScaledDisplaced(RBX, Eight, 881623167, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 225, 133, 131, 226, 28, 221, 127, 128, 140, 52], OperandSize::Qword)
}

#[test]
fn vpsraq_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAQ, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 245, 173, 226, 210], OperandSize::Dword)
}

#[test]
fn vpsraq_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAQ, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectScaledIndexedDisplaced(EBX, ECX, Two, 1847198361, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 237, 172, 226, 180, 75, 153, 2, 26, 110], OperandSize::Dword)
}

#[test]
fn vpsraq_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAQ, operand1: Some(Direct(YMM10)), operand2: Some(Direct(YMM15)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 113, 133, 171, 226, 209], OperandSize::Qword)
}

#[test]
fn vpsraq_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAQ, operand1: Some(Direct(YMM18)), operand2: Some(Direct(YMM8)), operand3: Some(IndirectScaledIndexed(RDI, RBX, Four, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 225, 189, 171, 226, 20, 159], OperandSize::Qword)
}

#[test]
fn vpsraq_27() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAQ, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM4)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 221, 205, 226, 226], OperandSize::Dword)
}

#[test]
fn vpsraq_28() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAQ, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM6)), operand3: Some(IndirectScaledIndexedDisplaced(EDX, EBX, Eight, 701082540, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 205, 204, 226, 148, 218, 172, 171, 201, 41], OperandSize::Dword)
}

#[test]
fn vpsraq_29() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAQ, operand1: Some(Direct(ZMM21)), operand2: Some(Direct(ZMM17)), operand3: Some(Direct(XMM12)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 193, 245, 197, 226, 236], OperandSize::Qword)
}

#[test]
fn vpsraq_30() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAQ, operand1: Some(Direct(ZMM23)), operand2: Some(Direct(ZMM8)), operand3: Some(IndirectScaledIndexedDisplaced(RSI, RDX, Two, 277587180, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 225, 189, 205, 226, 188, 86, 236, 164, 139, 16], OperandSize::Qword)
}

