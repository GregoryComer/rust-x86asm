use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vaesdec_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VAESDEC, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 65, 222, 215], OperandSize::Dword)
}

#[test]
fn vaesdec_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VAESDEC, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM1)), operand3: Some(Indirect(EAX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 113, 222, 8], OperandSize::Dword)
}

#[test]
fn vaesdec_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VAESDEC, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 73, 222, 238], OperandSize::Qword)
}

#[test]
fn vaesdec_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VAESDEC, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectDisplaced(RCX, 1406049638, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 105, 222, 137, 102, 157, 206, 83], OperandSize::Qword)
}

