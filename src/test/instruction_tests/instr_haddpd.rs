use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn haddpd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::HADDPD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 124, 230], OperandSize::Dword)
}

#[test]
fn haddpd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::HADDPD, operand1: Some(Direct(XMM2)), operand2: Some(IndirectScaledIndexedDisplaced(EBX, ECX, Eight, 356745661, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 124, 148, 203, 189, 129, 67, 21], OperandSize::Dword)
}

#[test]
fn haddpd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::HADDPD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 124, 238], OperandSize::Qword)
}

#[test]
fn haddpd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::HADDPD, operand1: Some(Direct(XMM4)), operand2: Some(IndirectScaledDisplaced(RDX, Two, 1543129649, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 124, 36, 85, 49, 74, 250, 91], OperandSize::Qword)
}

