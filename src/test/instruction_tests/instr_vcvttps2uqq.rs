use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vcvttps2uqq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPS2UQQ, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 125, 143, 120, 252], OperandSize::Dword)
}

#[test]
fn vcvttps2uqq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPS2UQQ, operand1: Some(Direct(XMM0)), operand2: Some(IndirectDisplaced(ECX, 740263529, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 125, 143, 120, 129, 105, 134, 31, 44], OperandSize::Dword)
}

#[test]
fn vcvttps2uqq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPS2UQQ, operand1: Some(Direct(XMM27)), operand2: Some(Direct(XMM16)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 33, 125, 138, 120, 216], OperandSize::Qword)
}

#[test]
fn vcvttps2uqq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPS2UQQ, operand1: Some(Direct(XMM8)), operand2: Some(Indirect(RBX, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 113, 125, 140, 120, 3], OperandSize::Qword)
}

#[test]
fn vcvttps2uqq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPS2UQQ, operand1: Some(Direct(YMM1)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 125, 172, 120, 202], OperandSize::Dword)
}

#[test]
fn vcvttps2uqq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPS2UQQ, operand1: Some(Direct(YMM1)), operand2: Some(IndirectScaledIndexedDisplaced(EDX, EDI, Eight, 450334036, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 125, 172, 120, 140, 250, 84, 141, 215, 26], OperandSize::Dword)
}

#[test]
fn vcvttps2uqq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPS2UQQ, operand1: Some(Direct(YMM17)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 225, 125, 170, 120, 200], OperandSize::Qword)
}

#[test]
fn vcvttps2uqq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPS2UQQ, operand1: Some(Direct(YMM19)), operand2: Some(IndirectDisplaced(RDX, 1695524052, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 225, 125, 170, 120, 154, 212, 164, 15, 101], OperandSize::Qword)
}

#[test]
fn vcvttps2uqq_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPS2UQQ, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(YMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 125, 155, 120, 221], OperandSize::Dword)
}

#[test]
fn vcvttps2uqq_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPS2UQQ, operand1: Some(Direct(ZMM6)), operand2: Some(IndirectScaledIndexed(EDX, EBX, Eight, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 125, 202, 120, 52, 218], OperandSize::Dword)
}

#[test]
fn vcvttps2uqq_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPS2UQQ, operand1: Some(Direct(ZMM10)), operand2: Some(Direct(YMM27)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K3), broadcast: None }, &[98, 17, 125, 155, 120, 211], OperandSize::Qword)
}

#[test]
fn vcvttps2uqq_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPS2UQQ, operand1: Some(Direct(ZMM22)), operand2: Some(IndirectScaledIndexedDisplaced(RDI, RDI, Eight, 2127264706, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 225, 125, 205, 120, 180, 255, 194, 123, 203, 126], OperandSize::Qword)
}

