use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vsubss_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VSUBSS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 218, 92, 217], OperandSize::Dword)
}

#[test]
fn vsubss_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VSUBSS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectScaledDisplaced(EDX, Four, 749661036, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 242, 92, 52, 149, 108, 235, 174, 44], OperandSize::Dword)
}

#[test]
fn vsubss_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VSUBSS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 218, 92, 255], OperandSize::Qword)
}

#[test]
fn vsubss_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VSUBSS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM5)), operand3: Some(Indirect(RAX, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 210, 92, 0], OperandSize::Qword)
}

#[test]
fn vsubss_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VSUBSS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Down), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 126, 187, 92, 232], OperandSize::Dword)
}

#[test]
fn vsubss_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VSUBSS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectScaledIndexed(ECX, EDI, Two, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 78, 139, 92, 36, 121], OperandSize::Dword)
}

#[test]
fn vsubss_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VSUBSS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM28)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Up), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 30, 214, 92, 236], OperandSize::Qword)
}

#[test]
fn vsubss_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VSUBSS, operand1: Some(Direct(XMM29)), operand2: Some(Direct(XMM30)), operand3: Some(IndirectScaledIndexed(RCX, RCX, Two, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 97, 14, 133, 92, 44, 73], OperandSize::Qword)
}

