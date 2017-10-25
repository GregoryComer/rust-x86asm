use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn roundss_1() {
    run_test(&Instruction { mnemonic: Mnemonic::ROUNDSS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM7)), operand3: Some(Literal8(127)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 10, 199, 127], OperandSize::Dword)
}

#[test]
fn roundss_2() {
    run_test(&Instruction { mnemonic: Mnemonic::ROUNDSS, operand1: Some(Direct(XMM4)), operand2: Some(Indirect(ECX, Some(OperandSize::Dword), None)), operand3: Some(Literal8(67)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 10, 33, 67], OperandSize::Dword)
}

#[test]
fn roundss_3() {
    run_test(&Instruction { mnemonic: Mnemonic::ROUNDSS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM6)), operand3: Some(Literal8(8)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 10, 222, 8], OperandSize::Qword)
}

#[test]
fn roundss_4() {
    run_test(&Instruction { mnemonic: Mnemonic::ROUNDSS, operand1: Some(Direct(XMM6)), operand2: Some(IndirectDisplaced(RDX, 271370255, Some(OperandSize::Dword), None)), operand3: Some(Literal8(48)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 10, 178, 15, 200, 44, 16, 48], OperandSize::Qword)
}

