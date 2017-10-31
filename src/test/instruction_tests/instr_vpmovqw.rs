use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpmovqw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVQW, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 126, 138, 52, 227], OperandSize::Dword)
}

#[test]
fn vpmovqw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVQW, operand1: Some(IndirectScaledIndexedDisplaced(ESI, EBX, Two, 276496900, Some(OperandSize::Dword), None)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 8, 52, 164, 94, 4, 2, 123, 16], OperandSize::Dword)
}

#[test]
fn vpmovqw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVQW, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM14)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 114, 126, 142, 52, 244], OperandSize::Qword)
}

#[test]
fn vpmovqw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVQW, operand1: Some(IndirectDisplaced(RDX, 1660991519, Some(OperandSize::Dword), None)), operand2: Some(Direct(XMM24)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 98, 126, 8, 52, 130, 31, 184, 0, 99], OperandSize::Qword)
}

#[test]
fn vpmovqw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVQW, operand1: Some(Direct(XMM5)), operand2: Some(Direct(YMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 126, 175, 52, 245], OperandSize::Dword)
}

#[test]
fn vpmovqw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVQW, operand1: Some(IndirectScaledIndexed(ESI, ESI, Four, Some(OperandSize::Qword), None)), operand2: Some(Direct(YMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 40, 52, 52, 182], OperandSize::Dword)
}

#[test]
fn vpmovqw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVQW, operand1: Some(Direct(XMM3)), operand2: Some(Direct(YMM23)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 226, 126, 172, 52, 251], OperandSize::Qword)
}

#[test]
fn vpmovqw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVQW, operand1: Some(Indirect(RCX, Some(OperandSize::Qword), None)), operand2: Some(Direct(YMM25)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 98, 126, 40, 52, 9], OperandSize::Qword)
}

#[test]
fn vpmovqw_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVQW, operand1: Some(Direct(XMM3)), operand2: Some(Direct(ZMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 126, 203, 52, 211], OperandSize::Dword)
}

#[test]
fn vpmovqw_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVQW, operand1: Some(IndirectDisplaced(ESI, 217000964, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(ZMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 72, 52, 158, 4, 44, 239, 12], OperandSize::Dword)
}

#[test]
fn vpmovqw_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVQW, operand1: Some(Direct(XMM18)), operand2: Some(Direct(ZMM19)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 162, 126, 202, 52, 218], OperandSize::Qword)
}

#[test]
fn vpmovqw_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVQW, operand1: Some(IndirectDisplaced(RBX, 1702614808, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(ZMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 72, 52, 163, 24, 215, 123, 101], OperandSize::Qword)
}

