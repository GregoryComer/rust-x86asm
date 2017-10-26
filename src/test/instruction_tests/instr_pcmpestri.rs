use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn pcmpestri_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PCMPESTRI, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM3)), operand3: Some(Literal8(12)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 97, 203, 12], OperandSize::Dword)
}

#[test]
fn pcmpestri_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PCMPESTRI, operand1: Some(Direct(XMM0)), operand2: Some(IndirectDisplaced(EDX, 528045122, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(110)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 97, 130, 66, 84, 121, 31, 110], OperandSize::Dword)
}

#[test]
fn pcmpestri_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PCMPESTRI, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM5)), operand3: Some(Literal8(110)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 97, 237, 110], OperandSize::Qword)
}

#[test]
fn pcmpestri_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PCMPESTRI, operand1: Some(Direct(XMM5)), operand2: Some(IndirectScaledIndexedDisplaced(RDX, RAX, Four, 1727035778, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(13)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 97, 172, 130, 130, 121, 240, 102, 13], OperandSize::Qword)
}

