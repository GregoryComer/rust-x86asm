use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vminss_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VMINSS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 194, 93, 236], OperandSize::Dword)
}

#[test]
fn vminss_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VMINSS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM0)), operand3: Some(Indirect(EDX, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 250, 93, 50], OperandSize::Dword)
}

#[test]
fn vminss_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VMINSS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 210, 93, 202], OperandSize::Qword)
}

#[test]
fn vminss_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VMINSS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM0)), operand3: Some(Indirect(RAX, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 250, 93, 40], OperandSize::Qword)
}

#[test]
fn vminss_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VMINSS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 118, 156, 93, 228], OperandSize::Dword)
}

#[test]
fn vminss_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VMINSS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM0)), operand3: Some(Indirect(EAX, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 126, 137, 93, 0], OperandSize::Dword)
}

#[test]
fn vminss_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VMINSS, operand1: Some(Direct(XMM16)), operand2: Some(Direct(XMM16)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K6), broadcast: None }, &[98, 225, 126, 150, 93, 197], OperandSize::Qword)
}

#[test]
fn vminss_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VMINSS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM28)), operand3: Some(IndirectScaledIndexed(RDI, RBX, Eight, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 30, 131, 93, 60, 223], OperandSize::Qword)
}

