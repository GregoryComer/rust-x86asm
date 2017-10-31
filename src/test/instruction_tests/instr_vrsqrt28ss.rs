use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vrsqrt28ss_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRT28SS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 125, 157, 205, 213], OperandSize::Dword)
}

#[test]
fn vrsqrt28ss_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRT28SS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledDisplaced(EBX, Two, 143872756, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 85, 141, 205, 60, 93, 244, 82, 147, 8], OperandSize::Dword)
}

#[test]
fn vrsqrt28ss_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRT28SS, operand1: Some(Direct(XMM19)), operand2: Some(Direct(XMM27)), operand3: Some(Direct(XMM29)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K5), broadcast: None }, &[98, 130, 37, 149, 205, 221], OperandSize::Qword)
}

#[test]
fn vrsqrt28ss_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRT28SS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM19)), operand3: Some(Indirect(RSI, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 101, 134, 205, 38], OperandSize::Qword)
}

