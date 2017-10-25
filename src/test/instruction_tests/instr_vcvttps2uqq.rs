use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vcvttps2uqq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPS2UQQ, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 125, 142, 120, 224], OperandSize::Dword)
}

#[test]
fn vcvttps2uqq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPS2UQQ, operand1: Some(Direct(XMM3)), operand2: Some(IndirectScaledDisplaced(EBX, Four, 354262205, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 125, 140, 120, 28, 157, 189, 156, 29, 21], OperandSize::Dword)
}

#[test]
fn vcvttps2uqq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPS2UQQ, operand1: Some(Direct(XMM10)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 113, 125, 140, 120, 212], OperandSize::Qword)
}

#[test]
fn vcvttps2uqq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPS2UQQ, operand1: Some(Direct(XMM25)), operand2: Some(IndirectScaledIndexedDisplaced(RDX, RBX, Two, 2135834352, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 97, 125, 140, 120, 140, 90, 240, 62, 78, 127], OperandSize::Qword)
}

#[test]
fn vcvttps2uqq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPS2UQQ, operand1: Some(Direct(YMM5)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 125, 172, 120, 238], OperandSize::Dword)
}

#[test]
fn vcvttps2uqq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPS2UQQ, operand1: Some(Direct(YMM3)), operand2: Some(IndirectScaledIndexedDisplaced(ESI, EDX, Eight, 850472165, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 125, 173, 120, 156, 214, 229, 44, 177, 50], OperandSize::Dword)
}

#[test]
fn vcvttps2uqq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPS2UQQ, operand1: Some(Direct(YMM0)), operand2: Some(Direct(XMM26)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 145, 125, 173, 120, 194], OperandSize::Qword)
}

#[test]
fn vcvttps2uqq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPS2UQQ, operand1: Some(Direct(YMM14)), operand2: Some(IndirectDisplaced(RSI, 1694965105, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 113, 125, 175, 120, 182, 113, 29, 7, 101], OperandSize::Qword)
}

#[test]
fn vcvttps2uqq_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPS2UQQ, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(YMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 125, 153, 120, 200], OperandSize::Dword)
}

#[test]
fn vcvttps2uqq_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPS2UQQ, operand1: Some(Direct(ZMM1)), operand2: Some(IndirectDisplaced(ESI, 2050169832, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 125, 207, 120, 142, 232, 27, 51, 122], OperandSize::Dword)
}

#[test]
fn vcvttps2uqq_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPS2UQQ, operand1: Some(Direct(ZMM8)), operand2: Some(Direct(YMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K5), broadcast: None }, &[98, 113, 125, 157, 120, 197], OperandSize::Qword)
}

#[test]
fn vcvttps2uqq_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPS2UQQ, operand1: Some(Direct(ZMM14)), operand2: Some(IndirectScaledIndexedDisplaced(RBX, RAX, Eight, 295987443, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 113, 125, 207, 120, 180, 195, 243, 104, 164, 17], OperandSize::Qword)
}

