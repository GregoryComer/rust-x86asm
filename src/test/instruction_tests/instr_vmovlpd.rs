use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vmovlpd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVLPD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectScaledDisplaced(ECX, Two, 1224112237, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 241, 18, 4, 77, 109, 120, 246, 72], OperandSize::Dword)
}

#[test]
fn vmovlpd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVLPD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectScaledIndexed(RBX, RAX, Two, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 241, 18, 44, 67], OperandSize::Qword)
}

#[test]
fn vmovlpd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVLPD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledDisplaced(ESI, Two, 1551991882, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 209, 18, 20, 117, 74, 132, 129, 92], OperandSize::Dword)
}

#[test]
fn vmovlpd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVLPD, operand1: Some(Direct(XMM26)), operand2: Some(Direct(XMM16)), operand3: Some(IndirectDisplaced(RBX, 1035024165, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 97, 253, 0, 18, 147, 37, 55, 177, 61], OperandSize::Qword)
}

#[test]
fn vmovlpd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVLPD, operand1: Some(IndirectDisplaced(EAX, 1809137764, Some(OperandSize::Qword), None)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 19, 184, 100, 64, 213, 107], OperandSize::Dword)
}

#[test]
fn vmovlpd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVLPD, operand1: Some(IndirectScaledIndexedDisplaced(RSI, RDX, Four, 470345258, Some(OperandSize::Qword), None)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 19, 140, 150, 42, 230, 8, 28], OperandSize::Qword)
}

#[test]
fn vmovlpd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVLPD, operand1: Some(Indirect(ESI, Some(OperandSize::Qword), None)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 19, 46], OperandSize::Dword)
}

#[test]
fn vmovlpd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVLPD, operand1: Some(IndirectScaledIndexed(RAX, RCX, Eight, Some(OperandSize::Qword), None)), operand2: Some(Direct(XMM10)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 121, 19, 20, 200], OperandSize::Qword)
}

