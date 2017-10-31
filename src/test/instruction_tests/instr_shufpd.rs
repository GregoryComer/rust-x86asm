use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn shufpd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::SHUFPD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM6)), operand3: Some(Literal8(82)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 198, 214, 82], OperandSize::Dword)
}

#[test]
fn shufpd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::SHUFPD, operand1: Some(Direct(XMM7)), operand2: Some(Indirect(ECX, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(50)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 198, 57, 50], OperandSize::Dword)
}

#[test]
fn shufpd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::SHUFPD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM4)), operand3: Some(Literal8(0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 198, 212, 0], OperandSize::Qword)
}

#[test]
fn shufpd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::SHUFPD, operand1: Some(Direct(XMM2)), operand2: Some(IndirectDisplaced(RDI, 471050838, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(38)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 198, 151, 86, 170, 19, 28, 38], OperandSize::Qword)
}

