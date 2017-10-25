use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn cvtdq2ps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTDQ2PS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 91, 247], OperandSize::Dword)
}

#[test]
fn cvtdq2ps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTDQ2PS, operand1: Some(Direct(XMM7)), operand2: Some(IndirectScaledIndexed(EAX, ECX, Two, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 91, 60, 72], OperandSize::Dword)
}

#[test]
fn cvtdq2ps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTDQ2PS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 91, 248], OperandSize::Qword)
}

#[test]
fn cvtdq2ps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTDQ2PS, operand1: Some(Direct(XMM4)), operand2: Some(IndirectScaledDisplaced(RDI, Eight, 836136961, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 91, 36, 253, 1, 112, 214, 49], OperandSize::Qword)
}

