use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vmovd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(EDI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 110, 255], OperandSize::Dword)
}

#[test]
fn vmovd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVD, operand1: Some(Direct(XMM2)), operand2: Some(IndirectScaledIndexed(EBX, EBX, Two, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 110, 20, 91], OperandSize::Dword)
}

#[test]
fn vmovd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(EBX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 110, 219], OperandSize::Qword)
}

#[test]
fn vmovd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVD, operand1: Some(Direct(XMM4)), operand2: Some(IndirectScaledIndexed(RSI, RAX, Four, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 110, 36, 134], OperandSize::Qword)
}

#[test]
fn vmovd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(EBP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 110, 213], OperandSize::Dword)
}

#[test]
fn vmovd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVD, operand1: Some(Direct(XMM3)), operand2: Some(Indirect(ECX, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 110, 25], OperandSize::Dword)
}

#[test]
fn vmovd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVD, operand1: Some(Direct(XMM14)), operand2: Some(Direct(EDI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 121, 110, 247], OperandSize::Qword)
}

#[test]
fn vmovd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVD, operand1: Some(Direct(XMM28)), operand2: Some(IndirectScaledIndexed(RDI, RCX, Eight, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 97, 125, 8, 110, 36, 207], OperandSize::Qword)
}

#[test]
fn vmovd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVD, operand1: Some(Direct(EBX)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 126, 219], OperandSize::Dword)
}

#[test]
fn vmovd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVD, operand1: Some(IndirectDisplaced(ECX, 1221394613, Some(OperandSize::Dword), None)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 126, 169, 181, 0, 205, 72], OperandSize::Dword)
}

#[test]
fn vmovd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVD, operand1: Some(Direct(ECX)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 126, 241], OperandSize::Qword)
}

#[test]
fn vmovd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVD, operand1: Some(IndirectScaledIndexedDisplaced(RAX, RDI, Two, 365396582, Some(OperandSize::Dword), None)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 126, 164, 120, 102, 130, 199, 21], OperandSize::Qword)
}

#[test]
fn vmovd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVD, operand1: Some(Direct(EDX)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 126, 218], OperandSize::Dword)
}

#[test]
fn vmovd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVD, operand1: Some(Indirect(ECX, Some(OperandSize::Dword), None)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 126, 1], OperandSize::Dword)
}

#[test]
fn vmovd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVD, operand1: Some(Direct(ESI)), operand2: Some(Direct(XMM28)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 97, 125, 8, 126, 230], OperandSize::Qword)
}

#[test]
fn vmovd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVD, operand1: Some(Indirect(RAX, Some(OperandSize::Dword), None)), operand2: Some(Direct(XMM29)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 97, 125, 8, 126, 40], OperandSize::Qword)
}

