use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpmulhrsw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULHRSW, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 89, 11, 236], OperandSize::Dword)
}

#[test]
fn vpmulhrsw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULHRSW, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledDisplaced(EDI, Two, 665645253, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 81, 11, 52, 125, 197, 240, 172, 39], OperandSize::Dword)
}

#[test]
fn vpmulhrsw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULHRSW, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 97, 11, 237], OperandSize::Qword)
}

#[test]
fn vpmulhrsw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULHRSW, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM5)), operand3: Some(Indirect(RDX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 81, 11, 26], OperandSize::Qword)
}

#[test]
fn vpmulhrsw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULHRSW, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(YMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 85, 11, 233], OperandSize::Dword)
}

#[test]
fn vpmulhrsw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULHRSW, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM2)), operand3: Some(Indirect(ESI, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 109, 11, 22], OperandSize::Dword)
}

#[test]
fn vpmulhrsw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULHRSW, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(YMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 101, 11, 243], OperandSize::Qword)
}

#[test]
fn vpmulhrsw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULHRSW, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectScaledIndexed(RSI, RBX, Two, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 101, 11, 60, 94], OperandSize::Qword)
}

#[test]
fn vpmulhrsw_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULHRSW, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 101, 138, 11, 225], OperandSize::Dword)
}

#[test]
fn vpmulhrsw_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULHRSW, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectScaledDisplaced(EBX, Four, 64201786, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 77, 141, 11, 44, 157, 58, 164, 211, 3], OperandSize::Dword)
}

#[test]
fn vpmulhrsw_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULHRSW, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM18)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 109, 133, 11, 204], OperandSize::Qword)
}

#[test]
fn vpmulhrsw_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULHRSW, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM25)), operand3: Some(IndirectScaledIndexed(RDI, RCX, Four, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 53, 132, 11, 20, 143], OperandSize::Qword)
}

#[test]
fn vpmulhrsw_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULHRSW, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 93, 175, 11, 237], OperandSize::Dword)
}

#[test]
fn vpmulhrsw_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULHRSW, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectScaledDisplaced(EAX, Four, 72241770, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 101, 173, 11, 12, 133, 106, 82, 78, 4], OperandSize::Dword)
}

#[test]
fn vpmulhrsw_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULHRSW, operand1: Some(Direct(YMM16)), operand2: Some(Direct(YMM17)), operand3: Some(Direct(YMM14)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 194, 117, 163, 11, 198], OperandSize::Qword)
}

#[test]
fn vpmulhrsw_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULHRSW, operand1: Some(Direct(YMM8)), operand2: Some(Direct(YMM2)), operand3: Some(Indirect(RAX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 114, 109, 172, 11, 0], OperandSize::Qword)
}

#[test]
fn vpmulhrsw_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULHRSW, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM4)), operand3: Some(Direct(ZMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 93, 205, 11, 225], OperandSize::Dword)
}

#[test]
fn vpmulhrsw_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULHRSW, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM6)), operand3: Some(IndirectScaledIndexed(EDX, ECX, Eight, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 77, 201, 11, 20, 202], OperandSize::Dword)
}

#[test]
fn vpmulhrsw_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULHRSW, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM6)), operand3: Some(Direct(ZMM24)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 146, 77, 205, 11, 232], OperandSize::Qword)
}

#[test]
fn vpmulhrsw_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULHRSW, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM3)), operand3: Some(IndirectScaledIndexedDisplaced(RDX, RDI, Four, 499990280, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 101, 203, 11, 180, 186, 8, 63, 205, 29], OperandSize::Qword)
}

