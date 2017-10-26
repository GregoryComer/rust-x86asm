use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpsllvq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLVQ, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 217, 71, 240], OperandSize::Dword)
}

#[test]
fn vpsllvq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLVQ, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectDisplaced(ESI, 100542133, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 193, 71, 158, 181, 38, 254, 5], OperandSize::Dword)
}

#[test]
fn vpsllvq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLVQ, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 225, 71, 247], OperandSize::Qword)
}

#[test]
fn vpsllvq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLVQ, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM3)), operand3: Some(Indirect(RSI, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 225, 71, 54], OperandSize::Qword)
}

#[test]
fn vpsllvq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLVQ, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 237, 71, 233], OperandSize::Dword)
}

#[test]
fn vpsllvq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLVQ, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectDisplaced(EDX, 1880761407, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 213, 71, 178, 63, 36, 26, 112], OperandSize::Dword)
}

#[test]
fn vpsllvq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLVQ, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 197, 71, 242], OperandSize::Qword)
}

#[test]
fn vpsllvq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLVQ, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectScaledIndexed(RSI, RBX, Eight, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 237, 71, 52, 222], OperandSize::Qword)
}

#[test]
fn vpsllvq_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLVQ, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 197, 139, 71, 202], OperandSize::Dword)
}

#[test]
fn vpsllvq_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLVQ, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledIndexedDisplaced(EDI, ECX, Two, 255736532, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 221, 142, 71, 132, 79, 212, 58, 62, 15], OperandSize::Dword)
}

#[test]
fn vpsllvq_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLVQ, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledIndexed(ECX, ESI, Two, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 242, 229, 158, 71, 36, 113], OperandSize::Dword)
}

#[test]
fn vpsllvq_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLVQ, operand1: Some(Direct(XMM14)), operand2: Some(Direct(XMM30)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 114, 141, 133, 71, 243], OperandSize::Qword)
}

#[test]
fn vpsllvq_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLVQ, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM10)), operand3: Some(IndirectScaledIndexedDisplaced(RDX, RAX, Two, 1758012050, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 173, 141, 71, 132, 66, 146, 34, 201, 104], OperandSize::Qword)
}

#[test]
fn vpsllvq_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLVQ, operand1: Some(Direct(XMM17)), operand2: Some(Direct(XMM29)), operand3: Some(IndirectScaledIndexedDisplaced(RDX, RCX, Two, 550714210, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 226, 149, 145, 71, 140, 74, 98, 59, 211, 32], OperandSize::Qword)
}

#[test]
fn vpsllvq_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLVQ, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 197, 175, 71, 206], OperandSize::Dword)
}

#[test]
fn vpsllvq_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLVQ, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectScaledIndexedDisplaced(EAX, EAX, Four, 437491997, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 197, 169, 71, 132, 128, 29, 153, 19, 26], OperandSize::Dword)
}

#[test]
fn vpsllvq_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLVQ, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectScaledIndexed(EDX, ECX, Two, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 237, 187, 71, 44, 74], OperandSize::Dword)
}

#[test]
fn vpsllvq_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLVQ, operand1: Some(Direct(YMM11)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM10)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 82, 221, 171, 71, 218], OperandSize::Qword)
}

#[test]
fn vpsllvq_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLVQ, operand1: Some(Direct(YMM25)), operand2: Some(Direct(YMM27)), operand3: Some(IndirectDisplaced(RSI, 1942892158, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 98, 165, 165, 71, 142, 126, 46, 206, 115], OperandSize::Qword)
}

#[test]
fn vpsllvq_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLVQ, operand1: Some(Direct(YMM23)), operand2: Some(Direct(YMM28)), operand3: Some(IndirectScaledDisplaced(RAX, Eight, 1063689260, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 226, 157, 179, 71, 60, 197, 44, 156, 102, 63], OperandSize::Qword)
}

#[test]
fn vpsllvq_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLVQ, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM4)), operand3: Some(Direct(ZMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 221, 204, 71, 192], OperandSize::Dword)
}

#[test]
fn vpsllvq_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLVQ, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM0)), operand3: Some(IndirectScaledDisplaced(ESI, Eight, 221046583, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 253, 204, 71, 12, 245, 55, 231, 44, 13], OperandSize::Dword)
}

#[test]
fn vpsllvq_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLVQ, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM4)), operand3: Some(IndirectScaledIndexed(ECX, EDX, Two, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 221, 221, 71, 44, 81], OperandSize::Dword)
}

#[test]
fn vpsllvq_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLVQ, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM19)), operand3: Some(Direct(ZMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 229, 197, 71, 214], OperandSize::Qword)
}

#[test]
fn vpsllvq_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLVQ, operand1: Some(Direct(ZMM22)), operand2: Some(Direct(ZMM6)), operand3: Some(IndirectScaledDisplaced(RDX, Eight, 966879673, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 226, 205, 204, 71, 52, 213, 185, 105, 161, 57], OperandSize::Qword)
}

#[test]
fn vpsllvq_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLVQ, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM15)), operand3: Some(IndirectScaledDisplaced(RDX, Four, 208152660, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 133, 222, 71, 60, 149, 84, 40, 104, 12], OperandSize::Qword)
}

