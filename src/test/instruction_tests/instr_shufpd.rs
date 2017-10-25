use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn shufpd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::SHUFPD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM7)), operand3: Some(Literal8(21)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 198, 247, 21], OperandSize::Dword)
}

#[test]
fn shufpd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::SHUFPD, operand1: Some(Direct(XMM6)), operand2: Some(Indirect(EAX, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(87)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 198, 48, 87], OperandSize::Dword)
}

#[test]
fn shufpd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::SHUFPD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM6)), operand3: Some(Literal8(122)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 198, 222, 122], OperandSize::Qword)
}

#[test]
fn shufpd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::SHUFPD, operand1: Some(Direct(XMM4)), operand2: Some(IndirectDisplaced(RDX, 1040152553, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(18)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 198, 162, 233, 119, 255, 61, 18], OperandSize::Qword)
}

