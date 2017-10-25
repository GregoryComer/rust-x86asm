use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpmovdb_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVDB, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 126, 142, 49, 226], OperandSize::Dword)
}

#[test]
fn vpmovdb_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVDB, operand1: Some(Indirect(EDX, Some(OperandSize::Dword), None)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 8, 49, 26], OperandSize::Dword)
}

#[test]
fn vpmovdb_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVDB, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM20)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 226, 126, 143, 49, 231], OperandSize::Qword)
}

#[test]
fn vpmovdb_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVDB, operand1: Some(IndirectScaledIndexedDisplaced(RDX, RCX, Eight, 400056718, Some(OperandSize::Dword), None)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 8, 49, 180, 202, 142, 97, 216, 23], OperandSize::Qword)
}

#[test]
fn vpmovdb_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVDB, operand1: Some(Direct(XMM0)), operand2: Some(Direct(YMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 126, 169, 49, 200], OperandSize::Dword)
}

#[test]
fn vpmovdb_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVDB, operand1: Some(Indirect(EBX, Some(OperandSize::Qword), None)), operand2: Some(Direct(YMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 40, 49, 3], OperandSize::Dword)
}

#[test]
fn vpmovdb_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVDB, operand1: Some(Direct(XMM6)), operand2: Some(Direct(YMM23)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 226, 126, 173, 49, 254], OperandSize::Qword)
}

#[test]
fn vpmovdb_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVDB, operand1: Some(Indirect(RBX, Some(OperandSize::Qword), None)), operand2: Some(Direct(YMM17)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 226, 126, 40, 49, 11], OperandSize::Qword)
}

#[test]
fn vpmovdb_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVDB, operand1: Some(Direct(XMM5)), operand2: Some(Direct(ZMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 126, 206, 49, 229], OperandSize::Dword)
}

#[test]
fn vpmovdb_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVDB, operand1: Some(IndirectScaledDisplaced(EBX, Two, 923160220, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(ZMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 72, 49, 28, 93, 156, 78, 6, 55], OperandSize::Dword)
}

#[test]
fn vpmovdb_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVDB, operand1: Some(Direct(XMM17)), operand2: Some(Direct(ZMM30)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 34, 126, 203, 49, 241], OperandSize::Qword)
}

#[test]
fn vpmovdb_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVDB, operand1: Some(IndirectScaledIndexed(RBX, RDI, Four, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(ZMM20)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 226, 126, 72, 49, 36, 187], OperandSize::Qword)
}

