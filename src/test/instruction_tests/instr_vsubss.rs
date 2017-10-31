use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vsubss_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VSUBSS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 250, 92, 237], OperandSize::Dword)
}

#[test]
fn vsubss_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VSUBSS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM2)), operand3: Some(Indirect(EBX, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 234, 92, 35], OperandSize::Dword)
}

#[test]
fn vsubss_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VSUBSS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 250, 92, 193], OperandSize::Qword)
}

#[test]
fn vsubss_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VSUBSS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledIndexedDisplaced(RAX, RDX, Two, 1612383588, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 234, 92, 140, 80, 100, 5, 27, 96], OperandSize::Qword)
}

#[test]
fn vsubss_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VSUBSS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Up), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 118, 223, 92, 251], OperandSize::Dword)
}

#[test]
fn vsubss_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VSUBSS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectDisplaced(EDI, 1129633747, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 70, 137, 92, 151, 211, 215, 84, 67], OperandSize::Dword)
}

#[test]
fn vsubss_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VSUBSS, operand1: Some(Direct(XMM18)), operand2: Some(Direct(XMM20)), operand3: Some(Direct(XMM27)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Nearest), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 129, 94, 145, 92, 211], OperandSize::Qword)
}

#[test]
fn vsubss_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VSUBSS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM27)), operand3: Some(IndirectDisplaced(RAX, 1522637919, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 38, 132, 92, 128, 95, 156, 193, 90], OperandSize::Qword)
}

