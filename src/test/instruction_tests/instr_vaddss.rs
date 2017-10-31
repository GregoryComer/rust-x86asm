use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vaddss_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDSS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 250, 88, 232], OperandSize::Dword)
}

#[test]
fn vaddss_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDSS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledDisplaced(EBX, Four, 296363448, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 194, 88, 28, 157, 184, 37, 170, 17], OperandSize::Dword)
}

#[test]
fn vaddss_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDSS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 210, 88, 205], OperandSize::Qword)
}

#[test]
fn vaddss_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDSS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledIndexed(RSI, RSI, Four, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 234, 88, 36, 182], OperandSize::Qword)
}

#[test]
fn vaddss_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDSS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Up), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 94, 223, 88, 193], OperandSize::Dword)
}

#[test]
fn vaddss_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDSS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectDisplaced(EAX, 1374532610, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 126, 140, 88, 168, 2, 180, 237, 81], OperandSize::Dword)
}

#[test]
fn vaddss_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDSS, operand1: Some(Direct(XMM15)), operand2: Some(Direct(XMM17)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Zero), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 113, 118, 242, 88, 248], OperandSize::Qword)
}

#[test]
fn vaddss_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDSS, operand1: Some(Direct(XMM24)), operand2: Some(Direct(XMM3)), operand3: Some(Indirect(RBX, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 97, 102, 137, 88, 3], OperandSize::Qword)
}

