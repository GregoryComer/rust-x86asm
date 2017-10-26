use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpmulhrsw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULHRSW, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 105, 11, 205], OperandSize::Dword)
}

#[test]
fn vpmulhrsw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULHRSW, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM3)), operand3: Some(Indirect(ECX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 97, 11, 57], OperandSize::Dword)
}

#[test]
fn vpmulhrsw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULHRSW, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 65, 11, 248], OperandSize::Qword)
}

#[test]
fn vpmulhrsw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULHRSW, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectScaledIndexed(RCX, RDX, Eight, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 113, 11, 36, 209], OperandSize::Qword)
}

#[test]
fn vpmulhrsw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULHRSW, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 69, 11, 205], OperandSize::Dword)
}

#[test]
fn vpmulhrsw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULHRSW, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM7)), operand3: Some(Indirect(EDX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 69, 11, 34], OperandSize::Dword)
}

#[test]
fn vpmulhrsw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULHRSW, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(YMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 77, 11, 197], OperandSize::Qword)
}

#[test]
fn vpmulhrsw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULHRSW, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectScaledDisplaced(RBX, Two, 1718116620, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 101, 11, 52, 93, 12, 97, 104, 102], OperandSize::Qword)
}

#[test]
fn vpmulhrsw_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULHRSW, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 117, 140, 11, 207], OperandSize::Dword)
}

#[test]
fn vpmulhrsw_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULHRSW, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectDisplaced(ECX, 1121551369, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 117, 138, 11, 177, 9, 132, 217, 66], OperandSize::Dword)
}

#[test]
fn vpmulhrsw_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULHRSW, operand1: Some(Direct(XMM23)), operand2: Some(Direct(XMM9)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 226, 53, 138, 11, 255], OperandSize::Qword)
}

#[test]
fn vpmulhrsw_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULHRSW, operand1: Some(Direct(XMM17)), operand2: Some(Direct(XMM21)), operand3: Some(Indirect(RDX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 226, 85, 132, 11, 10], OperandSize::Qword)
}

#[test]
fn vpmulhrsw_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULHRSW, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(YMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 125, 174, 11, 198], OperandSize::Dword)
}

#[test]
fn vpmulhrsw_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULHRSW, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectDisplaced(EDI, 232614265, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 117, 173, 11, 175, 121, 105, 221, 13], OperandSize::Dword)
}

#[test]
fn vpmulhrsw_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULHRSW, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM18)), operand3: Some(Direct(YMM25)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 146, 109, 164, 11, 201], OperandSize::Qword)
}

#[test]
fn vpmulhrsw_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULHRSW, operand1: Some(Direct(YMM19)), operand2: Some(Direct(YMM8)), operand3: Some(IndirectScaledDisplaced(RCX, Four, 910320053, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 226, 61, 175, 11, 28, 141, 181, 97, 66, 54], OperandSize::Qword)
}

#[test]
fn vpmulhrsw_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULHRSW, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM7)), operand3: Some(Direct(ZMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 69, 201, 11, 251], OperandSize::Dword)
}

#[test]
fn vpmulhrsw_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULHRSW, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM5)), operand3: Some(IndirectScaledIndexed(ESI, ESI, Two, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 85, 201, 11, 60, 118], OperandSize::Dword)
}

#[test]
fn vpmulhrsw_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULHRSW, operand1: Some(Direct(ZMM25)), operand2: Some(Direct(ZMM9)), operand3: Some(Direct(ZMM27)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 2, 53, 207, 11, 203], OperandSize::Qword)
}

#[test]
fn vpmulhrsw_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULHRSW, operand1: Some(Direct(ZMM11)), operand2: Some(Direct(ZMM14)), operand3: Some(IndirectScaledIndexed(RDX, RBX, Four, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 114, 13, 201, 11, 28, 154], OperandSize::Qword)
}

