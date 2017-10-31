use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vsqrtss_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VSQRTSS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 210, 81, 209], OperandSize::Dword)
}

#[test]
fn vsqrtss_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VSQRTSS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM7)), operand3: Some(Indirect(EBX, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 194, 81, 35], OperandSize::Dword)
}

#[test]
fn vsqrtss_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VSQRTSS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 226, 81, 235], OperandSize::Qword)
}

#[test]
fn vsqrtss_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VSQRTSS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectDisplaced(RCX, 1799748704, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 202, 81, 169, 96, 252, 69, 107], OperandSize::Qword)
}

#[test]
fn vsqrtss_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VSQRTSS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Down), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 126, 189, 81, 224], OperandSize::Dword)
}

#[test]
fn vsqrtss_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VSQRTSS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledDisplaced(ECX, Four, 1040968219, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 86, 141, 81, 28, 141, 27, 234, 11, 62], OperandSize::Dword)
}

#[test]
fn vsqrtss_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VSQRTSS, operand1: Some(Direct(XMM26)), operand2: Some(Direct(XMM20)), operand3: Some(Direct(XMM9)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Down), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 65, 94, 183, 81, 209], OperandSize::Qword)
}

#[test]
fn vsqrtss_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VSQRTSS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM18)), operand3: Some(Indirect(RAX, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 110, 130, 81, 16], OperandSize::Qword)
}

