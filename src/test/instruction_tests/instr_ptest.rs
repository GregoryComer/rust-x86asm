use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn ptest_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PTEST, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 23, 228], OperandSize::Dword)
}

#[test]
fn ptest_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PTEST, operand1: Some(Direct(XMM7)), operand2: Some(IndirectDisplaced(ECX, 1387916677, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 23, 185, 133, 237, 185, 82], OperandSize::Dword)
}

#[test]
fn ptest_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PTEST, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 23, 203], OperandSize::Qword)
}

#[test]
fn ptest_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PTEST, operand1: Some(Direct(XMM2)), operand2: Some(IndirectScaledDisplaced(RSI, Four, 206134156, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 23, 20, 181, 140, 91, 73, 12], OperandSize::Qword)
}

