use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn pcmpestri_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PCMPESTRI, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM6)), operand3: Some(Literal8(10)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 97, 198, 10], OperandSize::Dword)
}

#[test]
fn pcmpestri_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PCMPESTRI, operand1: Some(Direct(XMM4)), operand2: Some(IndirectScaledIndexedDisplaced(EDX, EBX, Four, 280929022, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(95)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 97, 164, 154, 254, 162, 190, 16, 95], OperandSize::Dword)
}

#[test]
fn pcmpestri_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PCMPESTRI, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM2)), operand3: Some(Literal8(50)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 97, 210, 50], OperandSize::Qword)
}

#[test]
fn pcmpestri_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PCMPESTRI, operand1: Some(Direct(XMM6)), operand2: Some(IndirectScaledIndexedDisplaced(RAX, RDI, Eight, 1207252338, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(51)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 97, 180, 248, 114, 53, 245, 71, 51], OperandSize::Qword)
}

