use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vucomisd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VUCOMISD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 46, 195], OperandSize::Dword)
}

#[test]
fn vucomisd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VUCOMISD, operand1: Some(Direct(XMM4)), operand2: Some(IndirectScaledIndexed(EDX, ESI, Eight, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 46, 36, 242], OperandSize::Dword)
}

#[test]
fn vucomisd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VUCOMISD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 46, 226], OperandSize::Qword)
}

#[test]
fn vucomisd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VUCOMISD, operand1: Some(Direct(XMM3)), operand2: Some(IndirectScaledIndexed(RDX, RBX, Eight, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 46, 28, 218], OperandSize::Qword)
}

#[test]
fn vucomisd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VUCOMISD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: true, mask: None, broadcast: None }, &[98, 241, 253, 24, 46, 213], OperandSize::Dword)
}

#[test]
fn vucomisd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VUCOMISD, operand1: Some(Direct(XMM2)), operand2: Some(Indirect(EAX, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 46, 16], OperandSize::Dword)
}

#[test]
fn vucomisd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VUCOMISD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM28)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: true, mask: None, broadcast: None }, &[98, 145, 253, 24, 46, 196], OperandSize::Qword)
}

#[test]
fn vucomisd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VUCOMISD, operand1: Some(Direct(XMM29)), operand2: Some(IndirectDisplaced(RDX, 718382290, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 97, 253, 8, 46, 170, 210, 164, 209, 42], OperandSize::Qword)
}

