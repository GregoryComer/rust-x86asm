use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpmovdb_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVDB, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 126, 140, 49, 245], OperandSize::Dword)
}

#[test]
fn vpmovdb_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVDB, operand1: Some(Indirect(EBX, Some(OperandSize::Dword), None)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 8, 49, 19], OperandSize::Dword)
}

#[test]
fn vpmovdb_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVDB, operand1: Some(Direct(XMM27)), operand2: Some(Direct(XMM14)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 18, 126, 141, 49, 243], OperandSize::Qword)
}

#[test]
fn vpmovdb_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVDB, operand1: Some(IndirectScaledDisplaced(RSI, Two, 2094269277, Some(OperandSize::Dword), None)), operand2: Some(Direct(XMM29)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 98, 126, 8, 49, 44, 117, 93, 3, 212, 124], OperandSize::Qword)
}

#[test]
fn vpmovdb_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVDB, operand1: Some(Direct(XMM7)), operand2: Some(Direct(YMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 126, 173, 49, 231], OperandSize::Dword)
}

#[test]
fn vpmovdb_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVDB, operand1: Some(IndirectScaledIndexedDisplaced(EBX, ESI, Two, 1097255986, Some(OperandSize::Qword), None)), operand2: Some(Direct(YMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 40, 49, 164, 115, 50, 204, 102, 65], OperandSize::Dword)
}

#[test]
fn vpmovdb_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVDB, operand1: Some(Direct(XMM0)), operand2: Some(Direct(YMM29)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 98, 126, 173, 49, 232], OperandSize::Qword)
}

#[test]
fn vpmovdb_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVDB, operand1: Some(IndirectScaledDisplaced(RDI, Four, 1928527624, Some(OperandSize::Qword), None)), operand2: Some(Direct(YMM10)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 114, 126, 40, 49, 20, 189, 8, 255, 242, 114], OperandSize::Qword)
}

#[test]
fn vpmovdb_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVDB, operand1: Some(Direct(XMM6)), operand2: Some(Direct(ZMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 126, 205, 49, 238], OperandSize::Dword)
}

#[test]
fn vpmovdb_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVDB, operand1: Some(IndirectDisplaced(ESI, 567197046, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(ZMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 72, 49, 190, 118, 189, 206, 33], OperandSize::Dword)
}

#[test]
fn vpmovdb_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVDB, operand1: Some(Direct(XMM5)), operand2: Some(Direct(ZMM12)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 114, 126, 202, 49, 229], OperandSize::Qword)
}

#[test]
fn vpmovdb_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVDB, operand1: Some(IndirectScaledIndexedDisplaced(RDI, RAX, Two, 1862829300, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(ZMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 72, 49, 156, 71, 244, 132, 8, 111], OperandSize::Qword)
}

