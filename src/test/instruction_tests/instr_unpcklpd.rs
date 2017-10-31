use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn unpcklpd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::UNPCKLPD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 20, 194], OperandSize::Dword)
}

#[test]
fn unpcklpd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::UNPCKLPD, operand1: Some(Direct(XMM3)), operand2: Some(IndirectScaledIndexedDisplaced(EDI, ESI, Eight, 2128255733, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 20, 156, 247, 245, 154, 218, 126], OperandSize::Dword)
}

#[test]
fn unpcklpd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::UNPCKLPD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 20, 254], OperandSize::Qword)
}

#[test]
fn unpcklpd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::UNPCKLPD, operand1: Some(Direct(XMM2)), operand2: Some(IndirectScaledDisplaced(RSI, Two, 1187866902, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 20, 20, 117, 22, 105, 205, 70], OperandSize::Qword)
}

