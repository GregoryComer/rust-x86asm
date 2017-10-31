use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vmulss_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VMULSS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 250, 89, 252], OperandSize::Dword)
}

#[test]
fn vmulss_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VMULSS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledDisplaced(ESI, Four, 38966295, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 250, 89, 20, 181, 23, 148, 82, 2], OperandSize::Dword)
}

#[test]
fn vmulss_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VMULSS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 234, 89, 233], OperandSize::Qword)
}

#[test]
fn vmulss_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VMULSS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM7)), operand3: Some(Indirect(RSI, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 194, 89, 6], OperandSize::Qword)
}

#[test]
fn vmulss_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VMULSS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Nearest), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 126, 157, 89, 203], OperandSize::Dword)
}

#[test]
fn vmulss_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VMULSS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledIndexed(EDX, EAX, Two, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 94, 141, 89, 36, 66], OperandSize::Dword)
}

#[test]
fn vmulss_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VMULSS, operand1: Some(Direct(XMM20)), operand2: Some(Direct(XMM10)), operand3: Some(Direct(XMM19)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Up), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 161, 46, 217, 89, 227], OperandSize::Qword)
}

#[test]
fn vmulss_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VMULSS, operand1: Some(Direct(XMM24)), operand2: Some(Direct(XMM8)), operand3: Some(IndirectScaledIndexedDisplaced(RDX, RCX, Four, 1215678784, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 97, 62, 140, 89, 132, 138, 64, 201, 117, 72], OperandSize::Qword)
}

