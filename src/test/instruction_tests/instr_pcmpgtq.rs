use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn pcmpgtq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PCMPGTQ, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 55, 194], OperandSize::Dword)
}

#[test]
fn pcmpgtq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PCMPGTQ, operand1: Some(Direct(XMM4)), operand2: Some(IndirectScaledDisplaced(EBX, Two, 1979933831, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 55, 36, 93, 135, 100, 3, 118], OperandSize::Dword)
}

#[test]
fn pcmpgtq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PCMPGTQ, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 55, 224], OperandSize::Qword)
}

#[test]
fn pcmpgtq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PCMPGTQ, operand1: Some(Direct(XMM6)), operand2: Some(IndirectDisplaced(RDX, 1579920636, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 55, 178, 252, 172, 43, 94], OperandSize::Qword)
}

