use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpmovqw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVQW, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 126, 138, 52, 228], OperandSize::Dword)
}

#[test]
fn vpmovqw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVQW, operand1: Some(IndirectScaledIndexed(EDI, EAX, Eight, Some(OperandSize::Dword), None)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 8, 52, 4, 199], OperandSize::Dword)
}

#[test]
fn vpmovqw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVQW, operand1: Some(Direct(XMM15)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 210, 126, 141, 52, 231], OperandSize::Qword)
}

#[test]
fn vpmovqw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVQW, operand1: Some(Indirect(RCX, Some(OperandSize::Dword), None)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 8, 52, 41], OperandSize::Qword)
}

#[test]
fn vpmovqw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVQW, operand1: Some(Direct(XMM2)), operand2: Some(Direct(YMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 126, 173, 52, 218], OperandSize::Dword)
}

#[test]
fn vpmovqw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVQW, operand1: Some(Indirect(ECX, Some(OperandSize::Qword), None)), operand2: Some(Direct(YMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 40, 52, 25], OperandSize::Dword)
}

#[test]
fn vpmovqw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVQW, operand1: Some(Direct(XMM18)), operand2: Some(Direct(YMM16)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 162, 126, 170, 52, 194], OperandSize::Qword)
}

#[test]
fn vpmovqw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVQW, operand1: Some(Indirect(RAX, Some(OperandSize::Qword), None)), operand2: Some(Direct(YMM14)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 114, 126, 40, 52, 48], OperandSize::Qword)
}

#[test]
fn vpmovqw_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVQW, operand1: Some(Direct(XMM0)), operand2: Some(Direct(ZMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 126, 206, 52, 248], OperandSize::Dword)
}

#[test]
fn vpmovqw_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVQW, operand1: Some(IndirectScaledIndexedDisplaced(EDI, EDX, Eight, 1790561009, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(ZMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 72, 52, 140, 215, 241, 202, 185, 106], OperandSize::Dword)
}

#[test]
fn vpmovqw_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVQW, operand1: Some(Direct(XMM27)), operand2: Some(Direct(ZMM23)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 130, 126, 207, 52, 251], OperandSize::Qword)
}

#[test]
fn vpmovqw_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVQW, operand1: Some(IndirectScaledIndexedDisplaced(RDX, RDI, Four, 581779677, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(ZMM25)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 98, 126, 72, 52, 140, 186, 221, 64, 173, 34], OperandSize::Qword)
}

