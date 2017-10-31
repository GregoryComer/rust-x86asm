use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpmovusdb_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSDB, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 126, 141, 17, 195], OperandSize::Dword)
}

#[test]
fn vpmovusdb_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSDB, operand1: Some(Indirect(EAX, Some(OperandSize::Dword), None)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 8, 17, 24], OperandSize::Dword)
}

#[test]
fn vpmovusdb_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSDB, operand1: Some(Direct(XMM18)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 178, 126, 143, 17, 226], OperandSize::Qword)
}

#[test]
fn vpmovusdb_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSDB, operand1: Some(Indirect(RDI, Some(OperandSize::Dword), None)), operand2: Some(Direct(XMM15)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 114, 126, 8, 17, 63], OperandSize::Qword)
}

#[test]
fn vpmovusdb_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSDB, operand1: Some(Direct(XMM5)), operand2: Some(Direct(YMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 126, 171, 17, 237], OperandSize::Dword)
}

#[test]
fn vpmovusdb_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSDB, operand1: Some(IndirectScaledDisplaced(EAX, Two, 1015071688, Some(OperandSize::Qword), None)), operand2: Some(Direct(YMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 40, 17, 36, 69, 200, 195, 128, 60], OperandSize::Dword)
}

#[test]
fn vpmovusdb_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSDB, operand1: Some(Direct(XMM15)), operand2: Some(Direct(YMM24)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 66, 126, 170, 17, 199], OperandSize::Qword)
}

#[test]
fn vpmovusdb_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSDB, operand1: Some(IndirectDisplaced(RSI, 124031911, Some(OperandSize::Qword), None)), operand2: Some(Direct(YMM8)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 114, 126, 40, 17, 134, 167, 147, 100, 7], OperandSize::Qword)
}

#[test]
fn vpmovusdb_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSDB, operand1: Some(Direct(XMM6)), operand2: Some(Direct(ZMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 126, 206, 17, 254], OperandSize::Dword)
}

#[test]
fn vpmovusdb_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSDB, operand1: Some(IndirectScaledDisplaced(ECX, Four, 246319991, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(ZMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 72, 17, 36, 141, 119, 139, 174, 14], OperandSize::Dword)
}

#[test]
fn vpmovusdb_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSDB, operand1: Some(Direct(XMM29)), operand2: Some(Direct(ZMM20)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 130, 126, 203, 17, 229], OperandSize::Qword)
}

#[test]
fn vpmovusdb_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSDB, operand1: Some(IndirectScaledIndexed(RDX, RBX, Two, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(ZMM25)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 98, 126, 72, 17, 12, 90], OperandSize::Qword)
}

