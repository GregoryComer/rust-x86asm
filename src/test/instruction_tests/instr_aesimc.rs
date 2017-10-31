use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn aesimc_1() {
    run_test(&Instruction { mnemonic: Mnemonic::AESIMC, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 219, 202], OperandSize::Dword)
}

#[test]
fn aesimc_2() {
    run_test(&Instruction { mnemonic: Mnemonic::AESIMC, operand1: Some(Direct(XMM0)), operand2: Some(IndirectScaledIndexed(EAX, ECX, Four, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 219, 4, 136], OperandSize::Dword)
}

#[test]
fn aesimc_3() {
    run_test(&Instruction { mnemonic: Mnemonic::AESIMC, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 219, 212], OperandSize::Qword)
}

#[test]
fn aesimc_4() {
    run_test(&Instruction { mnemonic: Mnemonic::AESIMC, operand1: Some(Direct(XMM7)), operand2: Some(IndirectScaledIndexedDisplaced(RDX, RDX, Eight, 1867641788, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 219, 188, 210, 188, 243, 81, 111], OperandSize::Qword)
}

