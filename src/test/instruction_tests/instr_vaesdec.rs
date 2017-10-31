use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vaesdec_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VAESDEC, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 89, 222, 218], OperandSize::Dword)
}

#[test]
fn vaesdec_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VAESDEC, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledIndexed(EDI, EBX, Two, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 81, 222, 20, 95], OperandSize::Dword)
}

#[test]
fn vaesdec_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VAESDEC, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 222, 197], OperandSize::Qword)
}

#[test]
fn vaesdec_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VAESDEC, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledIndexed(RDI, RBX, Four, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 81, 222, 4, 159], OperandSize::Qword)
}

