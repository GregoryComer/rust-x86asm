use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpmulhw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULHW, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 229, 221], OperandSize::Dword)
}

#[test]
fn vpmulhw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULHW, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM6)), operand3: Some(Indirect(EDX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 201, 229, 34], OperandSize::Dword)
}

#[test]
fn vpmulhw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULHW, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 225, 229, 205], OperandSize::Qword)
}

#[test]
fn vpmulhw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULHW, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM3)), operand3: Some(Indirect(RCX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 225, 229, 49], OperandSize::Qword)
}

#[test]
fn vpmulhw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULHW, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 197, 229, 234], OperandSize::Dword)
}

#[test]
fn vpmulhw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULHW, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectScaledIndexed(EDI, ESI, Eight, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 205, 229, 4, 247], OperandSize::Dword)
}

#[test]
fn vpmulhw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULHW, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(YMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 213, 229, 198], OperandSize::Qword)
}

#[test]
fn vpmulhw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULHW, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectScaledDisplaced(RAX, Two, 1088091819, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 245, 229, 52, 69, 171, 246, 218, 64], OperandSize::Qword)
}

#[test]
fn vpmulhw_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULHW, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 109, 138, 229, 213], OperandSize::Dword)
}

#[test]
fn vpmulhw_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULHW, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM6)), operand3: Some(Indirect(ESI, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 77, 137, 229, 6], OperandSize::Dword)
}

#[test]
fn vpmulhw_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULHW, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM20)), operand3: Some(Direct(XMM16)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 177, 93, 130, 229, 192], OperandSize::Qword)
}

#[test]
fn vpmulhw_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULHW, operand1: Some(Direct(XMM11)), operand2: Some(Direct(XMM2)), operand3: Some(Indirect(RAX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 113, 109, 141, 229, 24], OperandSize::Qword)
}

#[test]
fn vpmulhw_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULHW, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(YMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 77, 173, 229, 248], OperandSize::Dword)
}

#[test]
fn vpmulhw_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULHW, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectScaledIndexed(EAX, EDI, Eight, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 117, 170, 229, 36, 248], OperandSize::Dword)
}

#[test]
fn vpmulhw_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULHW, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(YMM23)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 177, 101, 169, 229, 215], OperandSize::Qword)
}

#[test]
fn vpmulhw_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULHW, operand1: Some(Direct(YMM8)), operand2: Some(Direct(YMM25)), operand3: Some(IndirectScaledIndexed(RAX, RDX, Four, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 113, 53, 164, 229, 4, 144], OperandSize::Qword)
}

#[test]
fn vpmulhw_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULHW, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM7)), operand3: Some(Direct(ZMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 69, 203, 229, 193], OperandSize::Dword)
}

#[test]
fn vpmulhw_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULHW, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM7)), operand3: Some(Indirect(ESI, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 69, 205, 229, 62], OperandSize::Dword)
}

#[test]
fn vpmulhw_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULHW, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM28)), operand3: Some(Direct(ZMM20)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 177, 29, 195, 229, 228], OperandSize::Qword)
}

#[test]
fn vpmulhw_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULHW, operand1: Some(Direct(ZMM10)), operand2: Some(Direct(ZMM16)), operand3: Some(IndirectScaledIndexedDisplaced(RAX, RSI, Eight, 827535648, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 113, 125, 197, 229, 148, 240, 32, 49, 83, 49], OperandSize::Qword)
}

