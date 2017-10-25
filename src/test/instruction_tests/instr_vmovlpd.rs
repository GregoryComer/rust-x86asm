use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vmovlpd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVLPD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledIndexedDisplaced(ESI, ECX, Two, 247160231, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 209, 18, 188, 78, 167, 93, 187, 14], OperandSize::Dword)
}

#[test]
fn vmovlpd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVLPD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM1)), operand3: Some(Indirect(RDI, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 241, 18, 47], OperandSize::Qword)
}

#[test]
fn vmovlpd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVLPD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM6)), operand3: Some(Indirect(ECX, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 201, 18, 9], OperandSize::Dword)
}

#[test]
fn vmovlpd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVLPD, operand1: Some(Direct(XMM28)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledDisplaced(RCX, Four, 385436456, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 97, 197, 8, 18, 36, 141, 40, 75, 249, 22], OperandSize::Qword)
}

#[test]
fn vmovlpd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVLPD, operand1: Some(IndirectDisplaced(EDI, 417570571, Some(OperandSize::Qword), None)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 19, 183, 11, 159, 227, 24], OperandSize::Dword)
}

#[test]
fn vmovlpd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVLPD, operand1: Some(IndirectScaledIndexed(RSI, RAX, Two, Some(OperandSize::Qword), None)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 19, 44, 70], OperandSize::Qword)
}

#[test]
fn vmovlpd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVLPD, operand1: Some(IndirectScaledIndexed(EAX, EDI, Two, Some(OperandSize::Qword), None)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 19, 36, 120], OperandSize::Dword)
}

#[test]
fn vmovlpd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVLPD, operand1: Some(IndirectDisplaced(RSI, 382848103, Some(OperandSize::Qword), None)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 19, 150, 103, 204, 209, 22], OperandSize::Qword)
}

