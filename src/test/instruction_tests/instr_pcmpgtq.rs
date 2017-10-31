use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn pcmpgtq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PCMPGTQ, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 55, 220], OperandSize::Dword)
}

#[test]
fn pcmpgtq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PCMPGTQ, operand1: Some(Direct(XMM6)), operand2: Some(IndirectDisplaced(ESI, 471907897, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 55, 182, 57, 190, 32, 28], OperandSize::Dword)
}

#[test]
fn pcmpgtq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PCMPGTQ, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 55, 234], OperandSize::Qword)
}

#[test]
fn pcmpgtq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PCMPGTQ, operand1: Some(Direct(XMM3)), operand2: Some(IndirectScaledIndexedDisplaced(RCX, RDI, Eight, 310796349, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 55, 156, 249, 61, 96, 134, 18], OperandSize::Qword)
}

