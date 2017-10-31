use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpmovusqd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSQD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 126, 138, 21, 193], OperandSize::Dword)
}

#[test]
fn vpmovusqd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSQD, operand1: Some(IndirectScaledIndexed(ECX, EAX, Two, Some(OperandSize::Qword), None)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 8, 21, 52, 65], OperandSize::Dword)
}

#[test]
fn vpmovusqd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSQD, operand1: Some(Direct(XMM14)), operand2: Some(Direct(XMM9)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 82, 126, 141, 21, 206], OperandSize::Qword)
}

#[test]
fn vpmovusqd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSQD, operand1: Some(IndirectDisplaced(RDI, 1631188402, Some(OperandSize::Qword), None)), operand2: Some(Direct(XMM19)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 226, 126, 8, 21, 159, 178, 245, 57, 97], OperandSize::Qword)
}

#[test]
fn vpmovusqd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSQD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(YMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 126, 171, 21, 250], OperandSize::Dword)
}

#[test]
fn vpmovusqd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSQD, operand1: Some(IndirectScaledDisplaced(EBX, Eight, 1165191283, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(YMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 40, 21, 28, 221, 115, 104, 115, 69], OperandSize::Dword)
}

#[test]
fn vpmovusqd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSQD, operand1: Some(Direct(XMM26)), operand2: Some(Direct(YMM11)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 18, 126, 171, 21, 218], OperandSize::Qword)
}

#[test]
fn vpmovusqd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSQD, operand1: Some(IndirectScaledDisplaced(RDI, Four, 944600728, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(YMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 40, 21, 12, 189, 152, 118, 77, 56], OperandSize::Qword)
}

#[test]
fn vpmovusqd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSQD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(ZMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 126, 207, 21, 253], OperandSize::Dword)
}

#[test]
fn vpmovusqd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSQD, operand1: Some(IndirectScaledDisplaced(EDX, Two, 886521546, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(ZMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 72, 21, 60, 85, 202, 62, 215, 52], OperandSize::Dword)
}

#[test]
fn vpmovusqd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSQD, operand1: Some(Direct(YMM1)), operand2: Some(Direct(ZMM13)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 114, 126, 205, 21, 233], OperandSize::Qword)
}

#[test]
fn vpmovusqd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSQD, operand1: Some(Indirect(RDI, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(ZMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 72, 21, 39], OperandSize::Qword)
}

