use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpmovuswb_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSWB, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 126, 140, 16, 220], OperandSize::Dword)
}

#[test]
fn vpmovuswb_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSWB, operand1: Some(Indirect(ECX, Some(OperandSize::Qword), None)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 8, 16, 57], OperandSize::Dword)
}

#[test]
fn vpmovuswb_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSWB, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM13)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 114, 126, 137, 16, 235], OperandSize::Qword)
}

#[test]
fn vpmovuswb_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSWB, operand1: Some(Indirect(RAX, Some(OperandSize::Qword), None)), operand2: Some(Direct(XMM11)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 114, 126, 8, 16, 24], OperandSize::Qword)
}

#[test]
fn vpmovuswb_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSWB, operand1: Some(Direct(XMM2)), operand2: Some(Direct(YMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 126, 174, 16, 202], OperandSize::Dword)
}

#[test]
fn vpmovuswb_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSWB, operand1: Some(IndirectScaledDisplaced(EAX, Two, 1515580749, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(YMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 40, 16, 60, 69, 77, 237, 85, 90], OperandSize::Dword)
}

#[test]
fn vpmovuswb_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSWB, operand1: Some(Direct(XMM14)), operand2: Some(Direct(YMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 210, 126, 170, 16, 230], OperandSize::Qword)
}

#[test]
fn vpmovuswb_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSWB, operand1: Some(Indirect(RBX, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(YMM18)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 226, 126, 40, 16, 19], OperandSize::Qword)
}

#[test]
fn vpmovuswb_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSWB, operand1: Some(Direct(YMM2)), operand2: Some(Direct(ZMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 126, 203, 16, 218], OperandSize::Dword)
}

#[test]
fn vpmovuswb_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSWB, operand1: Some(IndirectScaledIndexed(EBX, EDX, Two, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(ZMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 72, 16, 28, 83], OperandSize::Dword)
}

#[test]
fn vpmovuswb_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSWB, operand1: Some(Direct(YMM25)), operand2: Some(Direct(ZMM12)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 18, 126, 205, 16, 225], OperandSize::Qword)
}

#[test]
fn vpmovuswb_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSWB, operand1: Some(IndirectScaledIndexedDisplaced(RDX, RBX, Four, 1058338155, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(ZMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 72, 16, 180, 154, 107, 245, 20, 63], OperandSize::Qword)
}

