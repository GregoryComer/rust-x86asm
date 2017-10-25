use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vaddss_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDSS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 250, 88, 210], OperandSize::Dword)
}

#[test]
fn vaddss_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDSS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledIndexed(EDX, EAX, Eight, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 234, 88, 20, 194], OperandSize::Dword)
}

#[test]
fn vaddss_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDSS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 234, 88, 194], OperandSize::Qword)
}

#[test]
fn vaddss_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDSS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledDisplaced(RCX, Eight, 1571273251, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 194, 88, 36, 205, 35, 186, 167, 93], OperandSize::Qword)
}

#[test]
fn vaddss_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDSS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Zero), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 86, 252, 88, 234], OperandSize::Dword)
}

#[test]
fn vaddss_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDSS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectDisplaced(EAX, 377502824, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 86, 140, 88, 128, 104, 60, 128, 22], OperandSize::Dword)
}

#[test]
fn vaddss_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDSS, operand1: Some(Direct(XMM25)), operand2: Some(Direct(XMM28)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Down), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 97, 30, 178, 88, 201], OperandSize::Qword)
}

#[test]
fn vaddss_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDSS, operand1: Some(Direct(XMM29)), operand2: Some(Direct(XMM28)), operand3: Some(IndirectScaledDisplaced(RSI, Two, 251746904, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 97, 30, 130, 88, 44, 117, 88, 90, 1, 15], OperandSize::Qword)
}

