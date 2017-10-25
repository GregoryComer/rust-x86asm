use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpmovusdb_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSDB, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 126, 141, 17, 230], OperandSize::Dword)
}

#[test]
fn vpmovusdb_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSDB, operand1: Some(IndirectScaledDisplaced(EDI, Four, 327611042, Some(OperandSize::Dword), None)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 8, 17, 44, 189, 162, 242, 134, 19], OperandSize::Dword)
}

#[test]
fn vpmovusdb_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSDB, operand1: Some(Direct(XMM15)), operand2: Some(Direct(XMM14)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 82, 126, 138, 17, 247], OperandSize::Qword)
}

#[test]
fn vpmovusdb_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSDB, operand1: Some(Indirect(RBX, Some(OperandSize::Dword), None)), operand2: Some(Direct(XMM14)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 114, 126, 8, 17, 51], OperandSize::Qword)
}

#[test]
fn vpmovusdb_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSDB, operand1: Some(Direct(XMM7)), operand2: Some(Direct(YMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 126, 173, 17, 207], OperandSize::Dword)
}

#[test]
fn vpmovusdb_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSDB, operand1: Some(IndirectDisplaced(EAX, 1836366719, Some(OperandSize::Qword), None)), operand2: Some(Direct(YMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 40, 17, 176, 127, 187, 116, 109], OperandSize::Dword)
}

#[test]
fn vpmovusdb_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSDB, operand1: Some(Direct(XMM0)), operand2: Some(Direct(YMM24)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 98, 126, 173, 17, 192], OperandSize::Qword)
}

#[test]
fn vpmovusdb_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSDB, operand1: Some(IndirectScaledIndexedDisplaced(RDX, RDI, Eight, 528273456, Some(OperandSize::Qword), None)), operand2: Some(Direct(YMM23)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 226, 126, 40, 17, 188, 250, 48, 208, 124, 31], OperandSize::Qword)
}

#[test]
fn vpmovusdb_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSDB, operand1: Some(Direct(XMM6)), operand2: Some(Direct(ZMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 126, 205, 17, 254], OperandSize::Dword)
}

#[test]
fn vpmovusdb_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSDB, operand1: Some(Indirect(EDI, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(ZMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 72, 17, 23], OperandSize::Dword)
}

#[test]
fn vpmovusdb_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSDB, operand1: Some(Direct(XMM7)), operand2: Some(Direct(ZMM15)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 114, 126, 203, 17, 255], OperandSize::Qword)
}

#[test]
fn vpmovusdb_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSDB, operand1: Some(Indirect(RDX, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(ZMM8)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 114, 126, 72, 17, 2], OperandSize::Qword)
}

