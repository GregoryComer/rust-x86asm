use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vsubss_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VSUBSS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 218, 92, 194], OperandSize::Dword)
}

#[test]
fn vsubss_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VSUBSS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledIndexed(EDI, ECX, Two, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 250, 92, 12, 79], OperandSize::Dword)
}

#[test]
fn vsubss_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VSUBSS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 202, 92, 244], OperandSize::Qword)
}

#[test]
fn vsubss_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VSUBSS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM5)), operand3: Some(Indirect(RDX, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 210, 92, 2], OperandSize::Qword)
}

#[test]
fn vsubss_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VSUBSS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Down), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 78, 186, 92, 237], OperandSize::Dword)
}

#[test]
fn vsubss_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VSUBSS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledIndexed(ECX, EDX, Eight, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 126, 138, 92, 44, 209], OperandSize::Dword)
}

#[test]
fn vsubss_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VSUBSS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM17)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Nearest), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 177, 126, 156, 92, 201], OperandSize::Qword)
}

#[test]
fn vsubss_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VSUBSS, operand1: Some(Direct(XMM14)), operand2: Some(Direct(XMM13)), operand3: Some(IndirectScaledIndexedDisplaced(RBX, RCX, Two, 1418488563, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 113, 22, 138, 92, 180, 75, 243, 106, 140, 84], OperandSize::Qword)
}

