use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn pmaxsb_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PMAXSB, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 60, 203], OperandSize::Dword)
}

#[test]
fn pmaxsb_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PMAXSB, operand1: Some(Direct(XMM0)), operand2: Some(IndirectScaledIndexed(EAX, EAX, Eight, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 60, 4, 192], OperandSize::Dword)
}

#[test]
fn pmaxsb_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PMAXSB, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 60, 239], OperandSize::Qword)
}

#[test]
fn pmaxsb_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PMAXSB, operand1: Some(Direct(XMM1)), operand2: Some(IndirectScaledIndexedDisplaced(RDX, RDI, Two, 533024291, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 60, 140, 122, 35, 78, 197, 31], OperandSize::Qword)
}

