use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vmovntpd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVNTPD, operand1: Some(Indirect(EAX, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 43, 48], OperandSize::Dword)
}

#[test]
fn vmovntpd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVNTPD, operand1: Some(IndirectScaledIndexed(RDX, RDI, Eight, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 43, 4, 250], OperandSize::Qword)
}

#[test]
fn vmovntpd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVNTPD, operand1: Some(IndirectDisplaced(ECX, 244811640, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(YMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 43, 161, 120, 135, 151, 14], OperandSize::Dword)
}

#[test]
fn vmovntpd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVNTPD, operand1: Some(IndirectScaledIndexed(RCX, RBX, Two, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(YMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 43, 52, 89], OperandSize::Qword)
}

#[test]
fn vmovntpd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVNTPD, operand1: Some(IndirectDisplaced(ECX, 1656305375, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 43, 169, 223, 54, 185, 98], OperandSize::Dword)
}

#[test]
fn vmovntpd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVNTPD, operand1: Some(Indirect(RBX, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM30)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 97, 253, 8, 43, 51], OperandSize::Qword)
}

#[test]
fn vmovntpd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVNTPD, operand1: Some(IndirectScaledIndexed(EAX, EDI, Two, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(YMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 43, 28, 120], OperandSize::Dword)
}

#[test]
fn vmovntpd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVNTPD, operand1: Some(IndirectDisplaced(RDI, 629683974, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(YMM27)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 97, 253, 40, 43, 159, 6, 55, 136, 37], OperandSize::Qword)
}

#[test]
fn vmovntpd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVNTPD, operand1: Some(IndirectScaledIndexed(ECX, ECX, Eight, Some(OperandSize::Zmmword), None)), operand2: Some(Direct(ZMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 253, 72, 43, 44, 201], OperandSize::Dword)
}

#[test]
fn vmovntpd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVNTPD, operand1: Some(Indirect(RDX, Some(OperandSize::Zmmword), None)), operand2: Some(Direct(ZMM27)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 97, 253, 72, 43, 26], OperandSize::Qword)
}

