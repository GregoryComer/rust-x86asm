use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn cmppd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::CMPPD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM7)), operand3: Some(Literal8(32)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 194, 231, 32], OperandSize::Dword)
}

#[test]
fn cmppd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::CMPPD, operand1: Some(Direct(XMM5)), operand2: Some(IndirectScaledIndexed(EDX, ECX, Eight, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(88)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 194, 44, 202, 88], OperandSize::Dword)
}

#[test]
fn cmppd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::CMPPD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM0)), operand3: Some(Literal8(97)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 194, 200, 97], OperandSize::Qword)
}

#[test]
fn cmppd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::CMPPD, operand1: Some(Direct(XMM2)), operand2: Some(IndirectScaledIndexed(RBX, RDI, Four, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(111)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 194, 20, 187, 111], OperandSize::Qword)
}

