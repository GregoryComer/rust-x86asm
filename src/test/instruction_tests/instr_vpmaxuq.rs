use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpmaxuq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUQ, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 253, 141, 63, 243], OperandSize::Dword)
}

#[test]
fn vpmaxuq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUQ, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledIndexedDisplaced(ESI, EBX, Four, 2104332895, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 237, 138, 63, 188, 158, 95, 146, 109, 125], OperandSize::Dword)
}

#[test]
fn vpmaxuq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUQ, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectScaledIndexedDisplaced(ECX, EAX, Four, 935838847, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 242, 205, 154, 63, 164, 129, 127, 196, 199, 55], OperandSize::Dword)
}

#[test]
fn vpmaxuq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUQ, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM10)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 210, 221, 137, 63, 250], OperandSize::Qword)
}

#[test]
fn vpmaxuq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUQ, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM25)), operand3: Some(IndirectScaledIndexedDisplaced(RCX, RAX, Eight, 1164114418, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 181, 129, 63, 132, 193, 242, 249, 98, 69], OperandSize::Qword)
}

#[test]
fn vpmaxuq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUQ, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM25)), operand3: Some(IndirectScaledIndexedDisplaced(RDX, RAX, Four, 1078665738, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 242, 181, 149, 63, 180, 130, 10, 34, 75, 64], OperandSize::Qword)
}

#[test]
fn vpmaxuq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUQ, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 237, 175, 63, 206], OperandSize::Dword)
}

#[test]
fn vpmaxuq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUQ, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectScaledDisplaced(ESI, Two, 1614648800, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 245, 169, 63, 12, 117, 224, 149, 61, 96], OperandSize::Dword)
}

#[test]
fn vpmaxuq_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUQ, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectScaledIndexedDisplaced(ESI, EDI, Four, 1136031624, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 213, 189, 63, 132, 190, 136, 119, 182, 67], OperandSize::Dword)
}

#[test]
fn vpmaxuq_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUQ, operand1: Some(Direct(YMM14)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(YMM16)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 50, 213, 171, 63, 240], OperandSize::Qword)
}

#[test]
fn vpmaxuq_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUQ, operand1: Some(Direct(YMM19)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectScaledIndexed(RBX, RBX, Two, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 226, 213, 172, 63, 28, 91], OperandSize::Qword)
}

#[test]
fn vpmaxuq_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUQ, operand1: Some(Direct(YMM17)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectScaledIndexedDisplaced(RAX, RDI, Eight, 1016091018, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 226, 245, 185, 63, 140, 248, 138, 81, 144, 60], OperandSize::Qword)
}

#[test]
fn vpmaxuq_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUQ, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM4)), operand3: Some(Direct(ZMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 221, 206, 63, 247], OperandSize::Dword)
}

#[test]
fn vpmaxuq_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUQ, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM1)), operand3: Some(IndirectScaledDisplaced(ESI, Eight, 1463018853, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 245, 204, 63, 60, 245, 101, 229, 51, 87], OperandSize::Dword)
}

#[test]
fn vpmaxuq_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUQ, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM2)), operand3: Some(IndirectDisplaced(EBX, 1214375693, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 237, 217, 63, 163, 13, 231, 97, 72], OperandSize::Dword)
}

#[test]
fn vpmaxuq_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUQ, operand1: Some(Direct(ZMM14)), operand2: Some(Direct(ZMM4)), operand3: Some(Direct(ZMM9)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 82, 221, 205, 63, 241], OperandSize::Qword)
}

#[test]
fn vpmaxuq_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUQ, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM8)), operand3: Some(IndirectScaledDisplaced(RDI, Eight, 1662347896, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 189, 207, 63, 28, 253, 120, 106, 21, 99], OperandSize::Qword)
}

#[test]
fn vpmaxuq_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUQ, operand1: Some(Direct(ZMM29)), operand2: Some(Direct(ZMM6)), operand3: Some(IndirectScaledIndexedDisplaced(RDI, RAX, Eight, 833836609, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 98, 205, 223, 63, 172, 199, 65, 86, 179, 49], OperandSize::Qword)
}

