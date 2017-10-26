use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vsqrtss_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VSQRTSS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 242, 81, 249], OperandSize::Dword)
}

#[test]
fn vsqrtss_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VSQRTSS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledDisplaced(EDI, Two, 767721386, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 210, 81, 4, 125, 170, 127, 194, 45], OperandSize::Dword)
}

#[test]
fn vsqrtss_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VSQRTSS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 202, 81, 212], OperandSize::Qword)
}

#[test]
fn vsqrtss_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VSQRTSS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectDisplaced(RBX, 219861871, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 210, 81, 171, 111, 211, 26, 13], OperandSize::Qword)
}

#[test]
fn vsqrtss_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VSQRTSS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Down), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 110, 190, 81, 205], OperandSize::Dword)
}

#[test]
fn vsqrtss_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VSQRTSS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectDisplaced(EDX, 546401738, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 126, 142, 81, 186, 202, 109, 145, 32], OperandSize::Dword)
}

#[test]
fn vsqrtss_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VSQRTSS, operand1: Some(Direct(XMM13)), operand2: Some(Direct(XMM13)), operand3: Some(Direct(XMM16)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Nearest), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 49, 22, 158, 81, 232], OperandSize::Qword)
}

#[test]
fn vsqrtss_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VSQRTSS, operand1: Some(Direct(XMM28)), operand2: Some(Direct(XMM27)), operand3: Some(Indirect(RDI, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 97, 38, 134, 81, 39], OperandSize::Qword)
}

