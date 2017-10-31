use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vsqrtss_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VSQRTSS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 202, 81, 236], OperandSize::Dword)
}

#[test]
fn vsqrtss_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VSQRTSS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM7)), operand3: Some(Indirect(EDX, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 194, 81, 42], OperandSize::Dword)
}

#[test]
fn vsqrtss_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VSQRTSS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 226, 81, 203], OperandSize::Qword)
}

#[test]
fn vsqrtss_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VSQRTSS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledIndexedDisplaced(RCX, RSI, Eight, 169778597, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 210, 81, 164, 241, 165, 157, 30, 10], OperandSize::Qword)
}

#[test]
fn vsqrtss_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VSQRTSS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Up), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 94, 222, 81, 211], OperandSize::Dword)
}

#[test]
fn vsqrtss_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VSQRTSS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledDisplaced(EDI, Four, 670902541, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 70, 142, 81, 4, 189, 13, 41, 253, 39], OperandSize::Dword)
}

#[test]
fn vsqrtss_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VSQRTSS, operand1: Some(Direct(XMM29)), operand2: Some(Direct(XMM15)), operand3: Some(Direct(XMM9)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Up), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 65, 6, 219, 81, 233], OperandSize::Qword)
}

#[test]
fn vsqrtss_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VSQRTSS, operand1: Some(Direct(XMM22)), operand2: Some(Direct(XMM23)), operand3: Some(Indirect(RAX, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 225, 70, 130, 81, 48], OperandSize::Qword)
}

