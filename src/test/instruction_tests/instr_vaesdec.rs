use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vaesdec_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VAESDEC, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 89, 222, 226], OperandSize::Dword)
}

#[test]
fn vaesdec_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VAESDEC, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledIndexed(ECX, ECX, Two, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 65, 222, 52, 73], OperandSize::Dword)
}

#[test]
fn vaesdec_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VAESDEC, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 81, 222, 242], OperandSize::Qword)
}

#[test]
fn vaesdec_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VAESDEC, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledIndexedDisplaced(RDX, RDI, Four, 47765047, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 89, 222, 132, 186, 55, 214, 216, 2], OperandSize::Qword)
}

