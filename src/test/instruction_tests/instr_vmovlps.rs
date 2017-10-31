use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vmovlps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVLPS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM5)), operand3: Some(Indirect(EDX, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 208, 18, 42], OperandSize::Dword)
}

#[test]
fn vmovlps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVLPS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledIndexed(RDI, RDX, Two, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 18, 28, 87], OperandSize::Qword)
}

#[test]
fn vmovlps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVLPS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledDisplaced(ECX, Four, 1096437799, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 18, 20, 141, 39, 80, 90, 65], OperandSize::Dword)
}

#[test]
fn vmovlps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVLPS, operand1: Some(Direct(XMM14)), operand2: Some(Direct(XMM14)), operand3: Some(IndirectScaledDisplaced(RDI, Eight, 1663067850, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 8, 18, 52, 253, 202, 102, 32, 99], OperandSize::Qword)
}

#[test]
fn vmovlps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVLPS, operand1: Some(IndirectScaledIndexed(ECX, EDI, Eight, Some(OperandSize::Qword), None)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 19, 52, 249], OperandSize::Dword)
}

#[test]
fn vmovlps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVLPS, operand1: Some(IndirectDisplaced(RDI, 539088751, Some(OperandSize::Qword), None)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 19, 191, 111, 215, 33, 32], OperandSize::Qword)
}

#[test]
fn vmovlps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVLPS, operand1: Some(IndirectScaledDisplaced(EAX, Eight, 499956697, Some(OperandSize::Qword), None)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 19, 12, 197, 217, 187, 204, 29], OperandSize::Dword)
}

#[test]
fn vmovlps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVLPS, operand1: Some(IndirectDisplaced(RDI, 1678008317, Some(OperandSize::Qword), None)), operand2: Some(Direct(XMM11)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 120, 19, 159, 253, 95, 4, 100], OperandSize::Qword)
}

