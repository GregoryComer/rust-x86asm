use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn unpcklpd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::UNPCKLPD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 20, 231], OperandSize::Dword)
}

#[test]
fn unpcklpd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::UNPCKLPD, operand1: Some(Direct(XMM7)), operand2: Some(IndirectScaledDisplaced(ECX, Two, 2096590049, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 20, 60, 77, 225, 108, 247, 124], OperandSize::Dword)
}

#[test]
fn unpcklpd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::UNPCKLPD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 20, 223], OperandSize::Qword)
}

#[test]
fn unpcklpd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::UNPCKLPD, operand1: Some(Direct(XMM7)), operand2: Some(Indirect(RDI, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 20, 63], OperandSize::Qword)
}

