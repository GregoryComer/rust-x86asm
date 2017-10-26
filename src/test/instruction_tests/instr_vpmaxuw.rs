use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpmaxuw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUW, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 89, 62, 227], OperandSize::Dword)
}

#[test]
fn vpmaxuw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUW, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledIndexedDisplaced(EBX, ECX, Eight, 733629275, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 97, 62, 188, 203, 91, 75, 186, 43], OperandSize::Dword)
}

#[test]
fn vpmaxuw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUW, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 113, 62, 197], OperandSize::Qword)
}

#[test]
fn vpmaxuw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUW, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledIndexed(RAX, RBX, Four, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 105, 62, 52, 152], OperandSize::Qword)
}

#[test]
fn vpmaxuw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUW, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 109, 62, 240], OperandSize::Dword)
}

#[test]
fn vpmaxuw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUW, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectScaledDisplaced(EBX, Two, 63743586, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 77, 62, 60, 93, 98, 166, 204, 3], OperandSize::Dword)
}

#[test]
fn vpmaxuw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUW, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(YMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 101, 62, 194], OperandSize::Qword)
}

#[test]
fn vpmaxuw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUW, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM5)), operand3: Some(Indirect(RDI, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 85, 62, 15], OperandSize::Qword)
}

#[test]
fn vpmaxuw_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUW, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 117, 143, 62, 211], OperandSize::Dword)
}

#[test]
fn vpmaxuw_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUW, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM5)), operand3: Some(Indirect(ESI, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 85, 140, 62, 38], OperandSize::Dword)
}

#[test]
fn vpmaxuw_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUW, operand1: Some(Direct(XMM23)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM13)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 194, 101, 141, 62, 253], OperandSize::Qword)
}

#[test]
fn vpmaxuw_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUW, operand1: Some(Direct(XMM10)), operand2: Some(Direct(XMM20)), operand3: Some(IndirectScaledIndexed(RBX, RDX, Four, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 114, 93, 133, 62, 20, 147], OperandSize::Qword)
}

#[test]
fn vpmaxuw_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUW, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(YMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 77, 172, 62, 230], OperandSize::Dword)
}

#[test]
fn vpmaxuw_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUW, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectScaledDisplaced(ECX, Two, 394015121, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 69, 169, 62, 52, 77, 145, 49, 124, 23], OperandSize::Dword)
}

#[test]
fn vpmaxuw_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUW, operand1: Some(Direct(YMM25)), operand2: Some(Direct(YMM14)), operand3: Some(Direct(YMM21)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 34, 13, 170, 62, 205], OperandSize::Qword)
}

#[test]
fn vpmaxuw_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUW, operand1: Some(Direct(YMM18)), operand2: Some(Direct(YMM11)), operand3: Some(IndirectScaledDisplaced(RDX, Two, 1588552774, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 226, 37, 169, 62, 20, 85, 70, 100, 175, 94], OperandSize::Qword)
}

#[test]
fn vpmaxuw_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUW, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM0)), operand3: Some(Direct(ZMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 125, 204, 62, 214], OperandSize::Dword)
}

#[test]
fn vpmaxuw_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUW, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM1)), operand3: Some(IndirectScaledDisplaced(EBX, Eight, 481796964, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 117, 201, 62, 44, 221, 100, 163, 183, 28], OperandSize::Dword)
}

#[test]
fn vpmaxuw_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUW, operand1: Some(Direct(ZMM20)), operand2: Some(Direct(ZMM15)), operand3: Some(Direct(ZMM10)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 194, 5, 206, 62, 226], OperandSize::Qword)
}

#[test]
fn vpmaxuw_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUW, operand1: Some(Direct(ZMM9)), operand2: Some(Direct(ZMM9)), operand3: Some(IndirectScaledDisplaced(RCX, Four, 342286610, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 114, 53, 202, 62, 12, 141, 18, 225, 102, 20], OperandSize::Qword)
}

