use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn aesenclast_1() {
    run_test(&Instruction { mnemonic: Mnemonic::AESENCLAST, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 221, 242], OperandSize::Dword)
}

#[test]
fn aesenclast_2() {
    run_test(&Instruction { mnemonic: Mnemonic::AESENCLAST, operand1: Some(Direct(XMM3)), operand2: Some(IndirectDisplaced(EAX, 1024451028, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 221, 152, 212, 225, 15, 61], OperandSize::Dword)
}

#[test]
fn aesenclast_3() {
    run_test(&Instruction { mnemonic: Mnemonic::AESENCLAST, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 221, 254], OperandSize::Qword)
}

#[test]
fn aesenclast_4() {
    run_test(&Instruction { mnemonic: Mnemonic::AESENCLAST, operand1: Some(Direct(XMM4)), operand2: Some(IndirectScaledIndexed(RSI, RCX, Two, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 221, 36, 78], OperandSize::Qword)
}

