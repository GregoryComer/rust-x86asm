use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpmaddwd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMADDWD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 193, 245, 197], OperandSize::Dword)
}

#[test]
fn vpmaddwd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMADDWD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectDisplaced(ESI, 1879883748, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 201, 245, 150, 228, 191, 12, 112], OperandSize::Dword)
}

#[test]
fn vpmaddwd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMADDWD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 225, 245, 217], OperandSize::Qword)
}

#[test]
fn vpmaddwd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMADDWD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledDisplaced(RBX, Eight, 786531389, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 209, 245, 20, 221, 61, 132, 225, 46], OperandSize::Qword)
}

#[test]
fn vpmaddwd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMADDWD, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 221, 245, 243], OperandSize::Dword)
}

#[test]
fn vpmaddwd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMADDWD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectScaledIndexed(EDX, EBX, Two, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 221, 245, 60, 90], OperandSize::Dword)
}

#[test]
fn vpmaddwd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMADDWD, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 197, 245, 216], OperandSize::Qword)
}

#[test]
fn vpmaddwd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMADDWD, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectScaledIndexed(RCX, RDX, Eight, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 237, 245, 52, 209], OperandSize::Qword)
}

#[test]
fn vpmaddwd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMADDWD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 125, 140, 245, 230], OperandSize::Dword)
}

#[test]
fn vpmaddwd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMADDWD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledIndexedDisplaced(EDX, EAX, Two, 743597174, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 93, 139, 245, 148, 66, 118, 100, 82, 44], OperandSize::Dword)
}

#[test]
fn vpmaddwd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMADDWD, operand1: Some(Direct(XMM25)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM18)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 33, 125, 137, 245, 202], OperandSize::Qword)
}

#[test]
fn vpmaddwd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMADDWD, operand1: Some(Direct(XMM14)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectDisplaced(RDI, 286513165, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 113, 93, 137, 245, 183, 13, 216, 19, 17], OperandSize::Qword)
}

#[test]
fn vpmaddwd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMADDWD, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 117, 174, 245, 195], OperandSize::Dword)
}

#[test]
fn vpmaddwd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMADDWD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectDisplaced(ECX, 108339000, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 77, 175, 245, 185, 56, 31, 117, 6], OperandSize::Dword)
}

#[test]
fn vpmaddwd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMADDWD, operand1: Some(Direct(YMM23)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM29)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 129, 69, 170, 245, 253], OperandSize::Qword)
}

#[test]
fn vpmaddwd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMADDWD, operand1: Some(Direct(YMM25)), operand2: Some(Direct(YMM13)), operand3: Some(IndirectScaledIndexedDisplaced(RAX, RSI, Eight, 22125508, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 97, 21, 175, 245, 140, 240, 196, 155, 81, 1], OperandSize::Qword)
}

#[test]
fn vpmaddwd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMADDWD, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM1)), operand3: Some(Direct(ZMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 117, 202, 245, 255], OperandSize::Dword)
}

#[test]
fn vpmaddwd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMADDWD, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM2)), operand3: Some(IndirectScaledIndexedDisplaced(ECX, ECX, Four, 265691414, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 109, 201, 245, 180, 137, 22, 33, 214, 15], OperandSize::Dword)
}

#[test]
fn vpmaddwd_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMADDWD, operand1: Some(Direct(ZMM20)), operand2: Some(Direct(ZMM28)), operand3: Some(Direct(ZMM31)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 129, 29, 196, 245, 231], OperandSize::Qword)
}

#[test]
fn vpmaddwd_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMADDWD, operand1: Some(Direct(ZMM19)), operand2: Some(Direct(ZMM27)), operand3: Some(Indirect(RSI, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 225, 37, 199, 245, 30], OperandSize::Qword)
}

