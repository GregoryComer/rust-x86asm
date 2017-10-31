use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn ptest_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PTEST, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 23, 206], OperandSize::Dword)
}

#[test]
fn ptest_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PTEST, operand1: Some(Direct(XMM4)), operand2: Some(IndirectScaledDisplaced(EDX, Four, 755819546, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 23, 36, 149, 26, 228, 12, 45], OperandSize::Dword)
}

#[test]
fn ptest_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PTEST, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 23, 229], OperandSize::Qword)
}

#[test]
fn ptest_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PTEST, operand1: Some(Direct(XMM3)), operand2: Some(IndirectScaledIndexedDisplaced(RAX, RAX, Eight, 1730976804, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 23, 156, 192, 36, 156, 44, 103], OperandSize::Qword)
}

