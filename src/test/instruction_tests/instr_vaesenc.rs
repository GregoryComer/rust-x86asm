use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vaesenc_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VAESENC, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 220, 239], OperandSize::Dword)
}

#[test]
fn vaesenc_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VAESENC, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectScaledIndexed(EAX, ESI, Eight, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 113, 220, 12, 240], OperandSize::Dword)
}

#[test]
fn vaesenc_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VAESENC, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 81, 220, 250], OperandSize::Qword)
}

#[test]
fn vaesenc_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VAESENC, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectScaledIndexedDisplaced(RBX, RAX, Eight, 545646735, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 73, 220, 172, 195, 143, 232, 133, 32], OperandSize::Qword)
}

