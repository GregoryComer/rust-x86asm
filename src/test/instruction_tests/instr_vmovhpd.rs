use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vmovhpd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVHPD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledIndexed(ECX, EAX, Four, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 22, 60, 129], OperandSize::Dword)
}

#[test]
fn vmovhpd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVHPD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM0)), operand3: Some(Indirect(RBX, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 22, 27], OperandSize::Qword)
}

#[test]
fn vmovhpd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVHPD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectDisplaced(EDI, 2060866628, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 241, 22, 143, 68, 84, 214, 122], OperandSize::Dword)
}

#[test]
fn vmovhpd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVHPD, operand1: Some(Direct(XMM28)), operand2: Some(Direct(XMM22)), operand3: Some(IndirectDisplaced(RAX, 1276543614, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 97, 205, 0, 22, 160, 126, 130, 22, 76], OperandSize::Qword)
}

#[test]
fn vmovhpd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVHPD, operand1: Some(IndirectScaledIndexedDisplaced(EDI, EBX, Two, 1680087901, Some(OperandSize::Qword), None)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 23, 172, 95, 93, 27, 36, 100], OperandSize::Dword)
}

#[test]
fn vmovhpd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVHPD, operand1: Some(IndirectScaledDisplaced(RSI, Two, 2036361973, Some(OperandSize::Qword), None)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 23, 52, 117, 245, 106, 96, 121], OperandSize::Qword)
}

#[test]
fn vmovhpd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVHPD, operand1: Some(IndirectScaledIndexed(EBX, ECX, Eight, Some(OperandSize::Qword), None)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 23, 60, 203], OperandSize::Dword)
}

#[test]
fn vmovhpd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVHPD, operand1: Some(Indirect(RDX, Some(OperandSize::Qword), None)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 23, 42], OperandSize::Qword)
}

