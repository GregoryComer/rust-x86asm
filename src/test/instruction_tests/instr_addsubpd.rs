use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn addsubpd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::ADDSUBPD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 208, 194], OperandSize::Dword)
}

#[test]
fn addsubpd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::ADDSUBPD, operand1: Some(Direct(XMM0)), operand2: Some(IndirectDisplaced(EAX, 1588386907, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 208, 128, 91, 220, 172, 94], OperandSize::Dword)
}

#[test]
fn addsubpd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::ADDSUBPD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 208, 201], OperandSize::Qword)
}

#[test]
fn addsubpd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::ADDSUBPD, operand1: Some(Direct(XMM2)), operand2: Some(IndirectScaledIndexed(RDI, RSI, Eight, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 208, 20, 247], OperandSize::Qword)
}

