use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn ptest_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PTEST, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 23, 240], OperandSize::Dword)
}

#[test]
fn ptest_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PTEST, operand1: Some(Direct(XMM1)), operand2: Some(IndirectScaledIndexed(EBX, ECX, Four, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 23, 12, 139], OperandSize::Dword)
}

#[test]
fn ptest_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PTEST, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 23, 193], OperandSize::Qword)
}

#[test]
fn ptest_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PTEST, operand1: Some(Direct(XMM6)), operand2: Some(Indirect(RAX, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 23, 48], OperandSize::Qword)
}

