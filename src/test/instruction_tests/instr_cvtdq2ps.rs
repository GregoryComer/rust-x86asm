use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn cvtdq2ps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTDQ2PS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 91, 210], OperandSize::Dword)
}

#[test]
fn cvtdq2ps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTDQ2PS, operand1: Some(Direct(XMM6)), operand2: Some(IndirectScaledDisplaced(ECX, Two, 829617393, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 91, 52, 77, 241, 244, 114, 49], OperandSize::Dword)
}

#[test]
fn cvtdq2ps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTDQ2PS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 91, 244], OperandSize::Qword)
}

#[test]
fn cvtdq2ps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTDQ2PS, operand1: Some(Direct(XMM2)), operand2: Some(IndirectScaledDisplaced(RBX, Two, 1952076968, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 91, 20, 93, 168, 84, 90, 116], OperandSize::Qword)
}

