use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn roundss_1() {
    run_test(&Instruction { mnemonic: Mnemonic::ROUNDSS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM6)), operand3: Some(Literal8(111)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 10, 238, 111], OperandSize::Dword)
}

#[test]
fn roundss_2() {
    run_test(&Instruction { mnemonic: Mnemonic::ROUNDSS, operand1: Some(Direct(XMM2)), operand2: Some(Indirect(EDI, Some(OperandSize::Dword), None)), operand3: Some(Literal8(64)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 10, 23, 64], OperandSize::Dword)
}

#[test]
fn roundss_3() {
    run_test(&Instruction { mnemonic: Mnemonic::ROUNDSS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM7)), operand3: Some(Literal8(102)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 10, 231, 102], OperandSize::Qword)
}

#[test]
fn roundss_4() {
    run_test(&Instruction { mnemonic: Mnemonic::ROUNDSS, operand1: Some(Direct(XMM1)), operand2: Some(IndirectScaledIndexedDisplaced(RDI, RDI, Two, 1666999943, Some(OperandSize::Dword), None)), operand3: Some(Literal8(125)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 10, 140, 127, 135, 102, 92, 99, 125], OperandSize::Qword)
}

