use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpmovusdb_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSDB, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 126, 141, 17, 231], OperandSize::Dword)
}

#[test]
fn vpmovusdb_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSDB, operand1: Some(Indirect(ESI, Some(OperandSize::Dword), None)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 8, 17, 14], OperandSize::Dword)
}

#[test]
fn vpmovusdb_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSDB, operand1: Some(Direct(XMM21)), operand2: Some(Direct(XMM16)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 162, 126, 142, 17, 197], OperandSize::Qword)
}

#[test]
fn vpmovusdb_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSDB, operand1: Some(IndirectScaledDisplaced(RAX, Eight, 1802205798, Some(OperandSize::Dword), None)), operand2: Some(Direct(XMM31)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 98, 126, 8, 17, 60, 197, 102, 122, 107, 107], OperandSize::Qword)
}

#[test]
fn vpmovusdb_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSDB, operand1: Some(Direct(XMM0)), operand2: Some(Direct(YMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 126, 169, 17, 200], OperandSize::Dword)
}

#[test]
fn vpmovusdb_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSDB, operand1: Some(IndirectDisplaced(ECX, 1291145204, Some(OperandSize::Qword), None)), operand2: Some(Direct(YMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 40, 17, 185, 244, 79, 245, 76], OperandSize::Dword)
}

#[test]
fn vpmovusdb_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSDB, operand1: Some(Direct(XMM1)), operand2: Some(Direct(YMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 126, 174, 17, 193], OperandSize::Qword)
}

#[test]
fn vpmovusdb_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSDB, operand1: Some(IndirectScaledIndexedDisplaced(RCX, RAX, Four, 1590166911, Some(OperandSize::Qword), None)), operand2: Some(Direct(YMM27)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 98, 126, 40, 17, 156, 129, 127, 5, 200, 94], OperandSize::Qword)
}

#[test]
fn vpmovusdb_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSDB, operand1: Some(Direct(XMM0)), operand2: Some(Direct(ZMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 126, 206, 17, 200], OperandSize::Dword)
}

#[test]
fn vpmovusdb_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSDB, operand1: Some(Indirect(ESI, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(ZMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 72, 17, 46], OperandSize::Dword)
}

#[test]
fn vpmovusdb_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSDB, operand1: Some(Direct(XMM30)), operand2: Some(Direct(ZMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 146, 126, 204, 17, 206], OperandSize::Qword)
}

#[test]
fn vpmovusdb_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSDB, operand1: Some(IndirectScaledIndexedDisplaced(RBX, RCX, Two, 945033856, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(ZMM11)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 114, 126, 72, 17, 156, 75, 128, 18, 84, 56], OperandSize::Qword)
}

