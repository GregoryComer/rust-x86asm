use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpmaxuq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUQ, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 221, 137, 63, 205], OperandSize::Dword)
}

#[test]
fn vpmaxuq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUQ, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectDisplaced(EBX, 167780144, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 237, 140, 63, 147, 48, 31, 0, 10], OperandSize::Dword)
}

#[test]
fn vpmaxuq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUQ, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledIndexedDisplaced(ECX, ECX, Eight, 59782200, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 242, 197, 154, 63, 156, 201, 56, 52, 144, 3], OperandSize::Dword)
}

#[test]
fn vpmaxuq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUQ, operand1: Some(Direct(XMM8)), operand2: Some(Direct(XMM29)), operand3: Some(Direct(XMM10)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 82, 149, 135, 63, 194], OperandSize::Qword)
}

#[test]
fn vpmaxuq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUQ, operand1: Some(Direct(XMM29)), operand2: Some(Direct(XMM28)), operand3: Some(IndirectScaledIndexed(RAX, RDI, Eight, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 98, 157, 135, 63, 44, 248], OperandSize::Qword)
}

#[test]
fn vpmaxuq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUQ, operand1: Some(Direct(XMM14)), operand2: Some(Direct(XMM24)), operand3: Some(IndirectScaledDisplaced(RCX, Two, 1133948013, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 114, 189, 149, 63, 52, 77, 109, 172, 150, 67], OperandSize::Qword)
}

#[test]
fn vpmaxuq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUQ, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(YMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 213, 169, 63, 250], OperandSize::Dword)
}

#[test]
fn vpmaxuq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUQ, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectScaledIndexed(EDI, ESI, Four, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 221, 172, 63, 60, 183], OperandSize::Dword)
}

#[test]
fn vpmaxuq_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUQ, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectDisplaced(ESI, 1999358799, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 253, 188, 63, 182, 79, 203, 43, 119], OperandSize::Dword)
}

#[test]
fn vpmaxuq_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUQ, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM11)), operand3: Some(Direct(YMM18)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 178, 165, 170, 63, 210], OperandSize::Qword)
}

#[test]
fn vpmaxuq_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUQ, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectScaledDisplaced(RBX, Eight, 1229791412, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 245, 170, 63, 36, 221, 180, 32, 77, 73], OperandSize::Qword)
}

#[test]
fn vpmaxuq_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUQ, operand1: Some(Direct(YMM24)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectScaledDisplaced(RBX, Two, 1708215741, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 98, 221, 187, 63, 4, 93, 189, 77, 209, 101], OperandSize::Qword)
}

#[test]
fn vpmaxuq_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUQ, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM4)), operand3: Some(Direct(ZMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 221, 205, 63, 233], OperandSize::Dword)
}

#[test]
fn vpmaxuq_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUQ, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM6)), operand3: Some(IndirectScaledDisplaced(EAX, Two, 724809379, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 205, 207, 63, 44, 69, 163, 182, 51, 43], OperandSize::Dword)
}

#[test]
fn vpmaxuq_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUQ, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM4)), operand3: Some(IndirectScaledIndexedDisplaced(EDX, EAX, Two, 1010037324, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 221, 219, 63, 140, 66, 76, 242, 51, 60], OperandSize::Dword)
}

#[test]
fn vpmaxuq_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUQ, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM9)), operand3: Some(Direct(ZMM19)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 178, 181, 201, 63, 203], OperandSize::Qword)
}

#[test]
fn vpmaxuq_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUQ, operand1: Some(Direct(ZMM10)), operand2: Some(Direct(ZMM24)), operand3: Some(IndirectScaledDisplaced(RAX, Four, 1858939656, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 114, 189, 196, 63, 20, 133, 8, 43, 205, 110], OperandSize::Qword)
}

#[test]
fn vpmaxuq_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUQ, operand1: Some(Direct(ZMM20)), operand2: Some(Direct(ZMM22)), operand3: Some(IndirectScaledIndexed(RCX, RDI, Two, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 226, 205, 212, 63, 36, 121], OperandSize::Qword)
}

