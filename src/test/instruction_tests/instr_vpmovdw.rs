use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpmovdw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVDW, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 126, 143, 51, 225], OperandSize::Dword)
}

#[test]
fn vpmovdw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVDW, operand1: Some(IndirectScaledIndexed(EBX, EDI, Four, Some(OperandSize::Qword), None)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 8, 51, 4, 187], OperandSize::Dword)
}

#[test]
fn vpmovdw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVDW, operand1: Some(Direct(XMM20)), operand2: Some(Direct(XMM16)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 162, 126, 141, 51, 196], OperandSize::Qword)
}

#[test]
fn vpmovdw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVDW, operand1: Some(Indirect(RBX, Some(OperandSize::Qword), None)), operand2: Some(Direct(XMM22)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 226, 126, 8, 51, 51], OperandSize::Qword)
}

#[test]
fn vpmovdw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVDW, operand1: Some(Direct(XMM1)), operand2: Some(Direct(YMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 126, 172, 51, 233], OperandSize::Dword)
}

#[test]
fn vpmovdw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVDW, operand1: Some(IndirectScaledDisplaced(EDI, Eight, 747448587, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(YMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 40, 51, 44, 253, 11, 41, 141, 44], OperandSize::Dword)
}

#[test]
fn vpmovdw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVDW, operand1: Some(Direct(XMM9)), operand2: Some(Direct(YMM19)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 194, 126, 174, 51, 217], OperandSize::Qword)
}

#[test]
fn vpmovdw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVDW, operand1: Some(IndirectScaledIndexedDisplaced(RDX, RBX, Two, 1213338410, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(YMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 40, 51, 164, 90, 42, 19, 82, 72], OperandSize::Qword)
}

#[test]
fn vpmovdw_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVDW, operand1: Some(Direct(YMM3)), operand2: Some(Direct(ZMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 126, 205, 51, 235], OperandSize::Dword)
}

#[test]
fn vpmovdw_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVDW, operand1: Some(IndirectScaledIndexed(EDX, ESI, Four, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(ZMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 72, 51, 20, 178], OperandSize::Dword)
}

#[test]
fn vpmovdw_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVDW, operand1: Some(Direct(YMM2)), operand2: Some(Direct(ZMM12)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 114, 126, 201, 51, 226], OperandSize::Qword)
}

#[test]
fn vpmovdw_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVDW, operand1: Some(Indirect(RBX, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(ZMM28)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 98, 126, 72, 51, 35], OperandSize::Qword)
}

