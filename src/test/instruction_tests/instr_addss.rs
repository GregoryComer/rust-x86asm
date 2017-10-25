use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn addss_1() {
    run_test(&Instruction { mnemonic: Mnemonic::ADDSS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 88, 213], OperandSize::Dword)
}

#[test]
fn addss_2() {
    run_test(&Instruction { mnemonic: Mnemonic::ADDSS, operand1: Some(Direct(XMM7)), operand2: Some(IndirectScaledDisplaced(EBX, Two, 759904892, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 88, 60, 93, 124, 58, 75, 45], OperandSize::Dword)
}

#[test]
fn addss_3() {
    run_test(&Instruction { mnemonic: Mnemonic::ADDSS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 88, 204], OperandSize::Qword)
}

#[test]
fn addss_4() {
    run_test(&Instruction { mnemonic: Mnemonic::ADDSS, operand1: Some(Direct(XMM3)), operand2: Some(IndirectScaledIndexedDisplaced(RAX, RDX, Two, 2098630542, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 88, 156, 80, 142, 143, 22, 125], OperandSize::Qword)
}

