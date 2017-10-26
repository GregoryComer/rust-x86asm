use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn cvtdq2ps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTDQ2PS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 91, 221], OperandSize::Dword)
}

#[test]
fn cvtdq2ps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTDQ2PS, operand1: Some(Direct(XMM1)), operand2: Some(IndirectScaledIndexed(EDI, EDI, Eight, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 91, 12, 255], OperandSize::Dword)
}

#[test]
fn cvtdq2ps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTDQ2PS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 91, 206], OperandSize::Qword)
}

#[test]
fn cvtdq2ps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTDQ2PS, operand1: Some(Direct(XMM2)), operand2: Some(IndirectDisplaced(RDI, 1473580903, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 91, 151, 103, 15, 213, 87], OperandSize::Qword)
}

