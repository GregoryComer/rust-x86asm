use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpmovusdb_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSDB, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 126, 138, 17, 205], OperandSize::Dword)
}

#[test]
fn vpmovusdb_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSDB, operand1: Some(IndirectDisplaced(EDI, 1047972841, Some(OperandSize::Dword), None)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 8, 17, 183, 233, 203, 118, 62], OperandSize::Dword)
}

#[test]
fn vpmovusdb_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSDB, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM13)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 114, 126, 141, 17, 234], OperandSize::Qword)
}

#[test]
fn vpmovusdb_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSDB, operand1: Some(IndirectScaledIndexed(RSI, RSI, Four, Some(OperandSize::Dword), None)), operand2: Some(Direct(XMM14)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 114, 126, 8, 17, 52, 182], OperandSize::Qword)
}

#[test]
fn vpmovusdb_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSDB, operand1: Some(Direct(XMM5)), operand2: Some(Direct(YMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 126, 171, 17, 237], OperandSize::Dword)
}

#[test]
fn vpmovusdb_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSDB, operand1: Some(IndirectScaledDisplaced(EBX, Eight, 1240449283, Some(OperandSize::Qword), None)), operand2: Some(Direct(YMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 40, 17, 12, 221, 3, 193, 239, 73], OperandSize::Dword)
}

#[test]
fn vpmovusdb_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSDB, operand1: Some(Direct(XMM2)), operand2: Some(Direct(YMM8)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 114, 126, 174, 17, 194], OperandSize::Qword)
}

#[test]
fn vpmovusdb_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSDB, operand1: Some(Indirect(RAX, Some(OperandSize::Qword), None)), operand2: Some(Direct(YMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 40, 17, 24], OperandSize::Qword)
}

#[test]
fn vpmovusdb_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSDB, operand1: Some(Direct(XMM6)), operand2: Some(Direct(ZMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 126, 202, 17, 238], OperandSize::Dword)
}

#[test]
fn vpmovusdb_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSDB, operand1: Some(IndirectScaledIndexed(ECX, EDX, Four, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(ZMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 72, 17, 60, 145], OperandSize::Dword)
}

#[test]
fn vpmovusdb_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSDB, operand1: Some(Direct(XMM17)), operand2: Some(Direct(ZMM18)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 162, 126, 205, 17, 209], OperandSize::Qword)
}

#[test]
fn vpmovusdb_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSDB, operand1: Some(IndirectScaledDisplaced(RAX, Two, 1660985262, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(ZMM26)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 98, 126, 72, 17, 20, 69, 174, 159, 0, 99], OperandSize::Qword)
}

