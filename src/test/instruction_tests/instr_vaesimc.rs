use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vaesimc_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VAESIMC, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 219, 194], OperandSize::Dword)
}

#[test]
fn vaesimc_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VAESIMC, operand1: Some(Direct(XMM3)), operand2: Some(Indirect(EDX, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 219, 26], OperandSize::Dword)
}

#[test]
fn vaesimc_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VAESIMC, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 219, 199], OperandSize::Qword)
}

#[test]
fn vaesimc_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VAESIMC, operand1: Some(Direct(XMM2)), operand2: Some(Indirect(RSI, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 219, 22], OperandSize::Qword)
}

