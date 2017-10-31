use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpmovdb_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVDB, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 126, 138, 49, 213], OperandSize::Dword)
}

#[test]
fn vpmovdb_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVDB, operand1: Some(IndirectScaledIndexed(EDI, ECX, Four, Some(OperandSize::Dword), None)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 8, 49, 28, 143], OperandSize::Dword)
}

#[test]
fn vpmovdb_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVDB, operand1: Some(Direct(XMM23)), operand2: Some(Direct(XMM15)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 50, 126, 137, 49, 255], OperandSize::Qword)
}

#[test]
fn vpmovdb_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVDB, operand1: Some(IndirectScaledIndexedDisplaced(RCX, RDI, Eight, 154374301, Some(OperandSize::Dword), None)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 8, 49, 156, 249, 157, 144, 51, 9], OperandSize::Qword)
}

#[test]
fn vpmovdb_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVDB, operand1: Some(Direct(XMM0)), operand2: Some(Direct(YMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 126, 171, 49, 216], OperandSize::Dword)
}

#[test]
fn vpmovdb_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVDB, operand1: Some(Indirect(ECX, Some(OperandSize::Qword), None)), operand2: Some(Direct(YMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 40, 49, 41], OperandSize::Dword)
}

#[test]
fn vpmovdb_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVDB, operand1: Some(Direct(XMM11)), operand2: Some(Direct(YMM31)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 66, 126, 169, 49, 251], OperandSize::Qword)
}

#[test]
fn vpmovdb_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVDB, operand1: Some(Indirect(RDI, Some(OperandSize::Qword), None)), operand2: Some(Direct(YMM11)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 114, 126, 40, 49, 31], OperandSize::Qword)
}

#[test]
fn vpmovdb_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVDB, operand1: Some(Direct(XMM2)), operand2: Some(Direct(ZMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 126, 202, 49, 218], OperandSize::Dword)
}

#[test]
fn vpmovdb_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVDB, operand1: Some(Indirect(ESI, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(ZMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 72, 49, 62], OperandSize::Dword)
}

#[test]
fn vpmovdb_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVDB, operand1: Some(Direct(XMM29)), operand2: Some(Direct(ZMM12)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 18, 126, 207, 49, 229], OperandSize::Qword)
}

#[test]
fn vpmovdb_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVDB, operand1: Some(IndirectScaledIndexedDisplaced(RBX, RAX, Two, 1210837113, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(ZMM30)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 98, 126, 72, 49, 180, 67, 121, 232, 43, 72], OperandSize::Qword)
}

