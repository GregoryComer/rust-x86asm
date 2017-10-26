use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn shufpd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::SHUFPD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM7)), operand3: Some(Literal8(101)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 198, 231, 101], OperandSize::Dword)
}

#[test]
fn shufpd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::SHUFPD, operand1: Some(Direct(XMM4)), operand2: Some(IndirectDisplaced(EDI, 1726536564, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(54)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 198, 167, 116, 219, 232, 102, 54], OperandSize::Dword)
}

#[test]
fn shufpd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::SHUFPD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM1)), operand3: Some(Literal8(59)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 198, 241, 59], OperandSize::Qword)
}

#[test]
fn shufpd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::SHUFPD, operand1: Some(Direct(XMM6)), operand2: Some(IndirectDisplaced(RBX, 1017252727, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(98)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 198, 179, 119, 11, 162, 60, 98], OperandSize::Qword)
}

