use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn ptest_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PTEST, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 23, 200], OperandSize::Dword)
}

#[test]
fn ptest_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PTEST, operand1: Some(Direct(XMM0)), operand2: Some(IndirectDisplaced(EAX, 2049001037, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 23, 128, 77, 70, 33, 122], OperandSize::Dword)
}

#[test]
fn ptest_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PTEST, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 23, 238], OperandSize::Qword)
}

#[test]
fn ptest_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PTEST, operand1: Some(Direct(XMM3)), operand2: Some(IndirectScaledIndexed(RSI, RDX, Four, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 23, 28, 150], OperandSize::Qword)
}

