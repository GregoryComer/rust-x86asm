use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpmovsqd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSQD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 126, 141, 37, 234], OperandSize::Dword)
}

#[test]
fn vpmovsqd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSQD, operand1: Some(Indirect(ECX, Some(OperandSize::Qword), None)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 8, 37, 57], OperandSize::Dword)
}

#[test]
fn vpmovsqd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSQD, operand1: Some(Direct(XMM16)), operand2: Some(Direct(XMM19)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 162, 126, 140, 37, 216], OperandSize::Qword)
}

#[test]
fn vpmovsqd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSQD, operand1: Some(IndirectDisplaced(RDX, 2044301832, Some(OperandSize::Qword), None)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 8, 37, 154, 8, 146, 217, 121], OperandSize::Qword)
}

#[test]
fn vpmovsqd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSQD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(YMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 126, 169, 37, 203], OperandSize::Dword)
}

#[test]
fn vpmovsqd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSQD, operand1: Some(IndirectScaledDisplaced(ESI, Two, 406313327, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(YMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 40, 37, 4, 117, 111, 217, 55, 24], OperandSize::Dword)
}

#[test]
fn vpmovsqd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSQD, operand1: Some(Direct(XMM26)), operand2: Some(Direct(YMM14)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 18, 126, 172, 37, 242], OperandSize::Qword)
}

#[test]
fn vpmovsqd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSQD, operand1: Some(IndirectDisplaced(RDI, 1496926945, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(YMM16)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 226, 126, 40, 37, 135, 225, 74, 57, 89], OperandSize::Qword)
}

#[test]
fn vpmovsqd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSQD, operand1: Some(Direct(YMM1)), operand2: Some(Direct(ZMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 126, 206, 37, 225], OperandSize::Dword)
}

#[test]
fn vpmovsqd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSQD, operand1: Some(IndirectDisplaced(EBX, 2096559076, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(ZMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 72, 37, 155, 228, 243, 246, 124], OperandSize::Dword)
}

#[test]
fn vpmovsqd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSQD, operand1: Some(Direct(YMM2)), operand2: Some(Direct(ZMM13)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 114, 126, 205, 37, 234], OperandSize::Qword)
}

#[test]
fn vpmovsqd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSQD, operand1: Some(IndirectScaledIndexedDisplaced(RSI, RAX, Two, 1781809897, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(ZMM18)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 226, 126, 72, 37, 148, 70, 233, 66, 52, 106], OperandSize::Qword)
}

