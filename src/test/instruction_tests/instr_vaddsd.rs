use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vaddsd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDSD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 251, 88, 196], OperandSize::Dword)
}

#[test]
fn vaddsd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDSD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectDisplaced(ESI, 504411809, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 195, 88, 150, 161, 182, 16, 30], OperandSize::Dword)
}

#[test]
fn vaddsd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDSD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 243, 88, 228], OperandSize::Qword)
}

#[test]
fn vaddsd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDSD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledDisplaced(RCX, Two, 1188295532, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 211, 88, 44, 77, 108, 243, 211, 70], OperandSize::Qword)
}

#[test]
fn vaddsd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDSD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Nearest), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 207, 159, 88, 232], OperandSize::Dword)
}

#[test]
fn vaddsd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDSD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledDisplaced(EBX, Two, 1922151305, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 223, 140, 88, 12, 93, 137, 179, 145, 114], OperandSize::Dword)
}

#[test]
fn vaddsd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDSD, operand1: Some(Direct(XMM14)), operand2: Some(Direct(XMM29)), operand3: Some(Direct(XMM25)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Zero), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 17, 151, 244, 88, 241], OperandSize::Qword)
}

#[test]
fn vaddsd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDSD, operand1: Some(Direct(XMM23)), operand2: Some(Direct(XMM12)), operand3: Some(IndirectScaledDisplaced(RDX, Four, 1936011175, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 225, 159, 141, 88, 60, 149, 167, 47, 101, 115], OperandSize::Qword)
}

