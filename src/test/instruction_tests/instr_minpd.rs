use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn minpd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::MINPD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 93, 205], OperandSize::Dword)
}

#[test]
fn minpd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::MINPD, operand1: Some(Direct(XMM0)), operand2: Some(Indirect(EAX, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 93, 0], OperandSize::Dword)
}

#[test]
fn minpd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::MINPD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 93, 243], OperandSize::Qword)
}

#[test]
fn minpd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::MINPD, operand1: Some(Direct(XMM2)), operand2: Some(IndirectScaledIndexedDisplaced(RDX, RSI, Four, 2118579532, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 93, 148, 178, 76, 245, 70, 126], OperandSize::Qword)
}

