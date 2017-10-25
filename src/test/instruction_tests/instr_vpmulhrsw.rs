use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpmulhrsw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULHRSW, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 113, 11, 199], OperandSize::Dword)
}

#[test]
fn vpmulhrsw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULHRSW, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectDisplaced(ESI, 471610804, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 73, 11, 190, 180, 53, 28, 28], OperandSize::Dword)
}

#[test]
fn vpmulhrsw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULHRSW, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 97, 11, 203], OperandSize::Qword)
}

#[test]
fn vpmulhrsw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULHRSW, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectDisplaced(RBX, 1814922674, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 105, 11, 155, 178, 133, 45, 108], OperandSize::Qword)
}

#[test]
fn vpmulhrsw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULHRSW, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 93, 11, 208], OperandSize::Dword)
}

#[test]
fn vpmulhrsw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULHRSW, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectScaledIndexed(EDX, ESI, Two, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 85, 11, 60, 114], OperandSize::Dword)
}

#[test]
fn vpmulhrsw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULHRSW, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(YMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 85, 11, 240], OperandSize::Qword)
}

#[test]
fn vpmulhrsw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULHRSW, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM6)), operand3: Some(Indirect(RAX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 77, 11, 48], OperandSize::Qword)
}

#[test]
fn vpmulhrsw_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULHRSW, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 69, 141, 11, 232], OperandSize::Dword)
}

#[test]
fn vpmulhrsw_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULHRSW, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledIndexed(EDI, EDI, Eight, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 101, 140, 11, 60, 255], OperandSize::Dword)
}

#[test]
fn vpmulhrsw_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULHRSW, operand1: Some(Direct(XMM16)), operand2: Some(Direct(XMM17)), operand3: Some(Direct(XMM13)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 194, 117, 129, 11, 197], OperandSize::Qword)
}

#[test]
fn vpmulhrsw_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULHRSW, operand1: Some(Direct(XMM18)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectScaledIndexed(RDX, RBX, Four, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 226, 77, 143, 11, 20, 154], OperandSize::Qword)
}

#[test]
fn vpmulhrsw_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULHRSW, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 109, 169, 11, 236], OperandSize::Dword)
}

#[test]
fn vpmulhrsw_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULHRSW, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectScaledIndexedDisplaced(EDI, EDI, Two, 2033543883, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 85, 174, 11, 180, 127, 203, 106, 53, 121], OperandSize::Dword)
}

#[test]
fn vpmulhrsw_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULHRSW, operand1: Some(Direct(YMM23)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM15)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 194, 93, 169, 11, 255], OperandSize::Qword)
}

#[test]
fn vpmulhrsw_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULHRSW, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectScaledIndexedDisplaced(RAX, RBX, Eight, 1431142005, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 125, 175, 11, 180, 216, 117, 126, 77, 85], OperandSize::Qword)
}

#[test]
fn vpmulhrsw_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULHRSW, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM2)), operand3: Some(Direct(ZMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 109, 205, 11, 249], OperandSize::Dword)
}

#[test]
fn vpmulhrsw_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULHRSW, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM0)), operand3: Some(IndirectDisplaced(EAX, 28724998, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 125, 207, 11, 128, 6, 79, 182, 1], OperandSize::Dword)
}

#[test]
fn vpmulhrsw_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULHRSW, operand1: Some(Direct(ZMM14)), operand2: Some(Direct(ZMM17)), operand3: Some(Direct(ZMM8)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 82, 117, 199, 11, 240], OperandSize::Qword)
}

#[test]
fn vpmulhrsw_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULHRSW, operand1: Some(Direct(ZMM23)), operand2: Some(Direct(ZMM2)), operand3: Some(IndirectDisplaced(RAX, 447048512, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 226, 109, 205, 11, 184, 64, 107, 165, 26], OperandSize::Qword)
}

