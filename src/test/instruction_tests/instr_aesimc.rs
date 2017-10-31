use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn aesimc_1() {
    run_test(&Instruction { mnemonic: Mnemonic::AESIMC, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 219, 221], OperandSize::Dword)
}

#[test]
fn aesimc_2() {
    run_test(&Instruction { mnemonic: Mnemonic::AESIMC, operand1: Some(Direct(XMM3)), operand2: Some(Indirect(EDI, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 219, 31], OperandSize::Dword)
}

#[test]
fn aesimc_3() {
    run_test(&Instruction { mnemonic: Mnemonic::AESIMC, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 219, 213], OperandSize::Qword)
}

#[test]
fn aesimc_4() {
    run_test(&Instruction { mnemonic: Mnemonic::AESIMC, operand1: Some(Direct(XMM1)), operand2: Some(IndirectScaledIndexed(RSI, RDI, Four, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 219, 12, 190], OperandSize::Qword)
}

