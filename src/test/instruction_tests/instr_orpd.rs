use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn orpd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::ORPD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 86, 210], OperandSize::Dword)
}

#[test]
fn orpd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::ORPD, operand1: Some(Direct(XMM6)), operand2: Some(IndirectScaledDisplaced(ECX, Eight, 1548210540, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 86, 52, 205, 108, 209, 71, 92], OperandSize::Dword)
}

#[test]
fn orpd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::ORPD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 86, 244], OperandSize::Qword)
}

#[test]
fn orpd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::ORPD, operand1: Some(Direct(XMM7)), operand2: Some(Indirect(RSI, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 86, 62], OperandSize::Qword)
}

