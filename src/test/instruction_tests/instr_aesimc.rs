use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn aesimc_1() {
    run_test(&Instruction { mnemonic: Mnemonic::AESIMC, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 219, 241], OperandSize::Dword)
}

#[test]
fn aesimc_2() {
    run_test(&Instruction { mnemonic: Mnemonic::AESIMC, operand1: Some(Direct(XMM4)), operand2: Some(IndirectDisplaced(EDX, 1066261130, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 219, 162, 138, 218, 141, 63], OperandSize::Dword)
}

#[test]
fn aesimc_3() {
    run_test(&Instruction { mnemonic: Mnemonic::AESIMC, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 219, 230], OperandSize::Qword)
}

#[test]
fn aesimc_4() {
    run_test(&Instruction { mnemonic: Mnemonic::AESIMC, operand1: Some(Direct(XMM0)), operand2: Some(IndirectScaledDisplaced(RCX, Two, 1325330752, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 219, 4, 77, 64, 241, 254, 78], OperandSize::Qword)
}

