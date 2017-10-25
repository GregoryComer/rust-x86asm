use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn addpd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::ADDPD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 88, 239], OperandSize::Dword)
}

#[test]
fn addpd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::ADDPD, operand1: Some(Direct(XMM7)), operand2: Some(IndirectDisplaced(ESI, 911104056, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 88, 190, 56, 88, 78, 54], OperandSize::Dword)
}

#[test]
fn addpd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::ADDPD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 88, 193], OperandSize::Qword)
}

#[test]
fn addpd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::ADDPD, operand1: Some(Direct(XMM7)), operand2: Some(IndirectScaledIndexedDisplaced(RSI, RDX, Two, 1950522182, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 88, 188, 86, 70, 155, 66, 116], OperandSize::Qword)
}

