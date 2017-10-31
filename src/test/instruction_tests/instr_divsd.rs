use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn divsd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::DIVSD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 94, 218], OperandSize::Dword)
}

#[test]
fn divsd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::DIVSD, operand1: Some(Direct(XMM7)), operand2: Some(Indirect(ESI, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 94, 62], OperandSize::Dword)
}

#[test]
fn divsd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::DIVSD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 94, 231], OperandSize::Qword)
}

#[test]
fn divsd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::DIVSD, operand1: Some(Direct(XMM1)), operand2: Some(Indirect(RSI, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 94, 14], OperandSize::Qword)
}

