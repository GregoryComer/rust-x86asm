use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn aesimc_1() {
    run_test(&Instruction { mnemonic: Mnemonic::AESIMC, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 219, 193], OperandSize::Dword)
}

#[test]
fn aesimc_2() {
    run_test(&Instruction { mnemonic: Mnemonic::AESIMC, operand1: Some(Direct(XMM5)), operand2: Some(IndirectScaledDisplaced(EDX, Two, 986480781, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 219, 44, 85, 141, 128, 204, 58], OperandSize::Dword)
}

#[test]
fn aesimc_3() {
    run_test(&Instruction { mnemonic: Mnemonic::AESIMC, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 219, 251], OperandSize::Qword)
}

#[test]
fn aesimc_4() {
    run_test(&Instruction { mnemonic: Mnemonic::AESIMC, operand1: Some(Direct(XMM2)), operand2: Some(IndirectScaledIndexed(RCX, RDI, Eight, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 219, 20, 249], OperandSize::Qword)
}

