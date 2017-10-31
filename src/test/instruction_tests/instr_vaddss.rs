use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vaddss_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDSS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 234, 88, 209], OperandSize::Dword)
}

#[test]
fn vaddss_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDSS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledIndexedDisplaced(EDX, ESI, Four, 1444840468, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 194, 88, 164, 178, 20, 132, 30, 86], OperandSize::Dword)
}

#[test]
fn vaddss_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDSS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 226, 88, 240], OperandSize::Qword)
}

#[test]
fn vaddss_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDSS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledIndexedDisplaced(RAX, RSI, Two, 1366643259, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 194, 88, 180, 112, 59, 82, 117, 81], OperandSize::Qword)
}

#[test]
fn vaddss_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDSS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Zero), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 102, 255, 88, 242], OperandSize::Dword)
}

#[test]
fn vaddss_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDSS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM2)), operand3: Some(Indirect(EAX, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 110, 141, 88, 56], OperandSize::Dword)
}

#[test]
fn vaddss_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDSS, operand1: Some(Direct(XMM23)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM28)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Down), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 129, 110, 187, 88, 252], OperandSize::Qword)
}

#[test]
fn vaddss_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDSS, operand1: Some(Direct(XMM31)), operand2: Some(Direct(XMM15)), operand3: Some(IndirectScaledDisplaced(RDX, Four, 1250502579, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 97, 6, 141, 88, 60, 149, 179, 39, 137, 74], OperandSize::Qword)
}

