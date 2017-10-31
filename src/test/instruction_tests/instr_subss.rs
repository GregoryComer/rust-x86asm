use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn subss_1() {
    run_test(&Instruction { mnemonic: Mnemonic::SUBSS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 92, 219], OperandSize::Dword)
}

#[test]
fn subss_2() {
    run_test(&Instruction { mnemonic: Mnemonic::SUBSS, operand1: Some(Direct(XMM7)), operand2: Some(Indirect(EDX, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 92, 58], OperandSize::Dword)
}

#[test]
fn subss_3() {
    run_test(&Instruction { mnemonic: Mnemonic::SUBSS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 92, 228], OperandSize::Qword)
}

#[test]
fn subss_4() {
    run_test(&Instruction { mnemonic: Mnemonic::SUBSS, operand1: Some(Direct(XMM0)), operand2: Some(IndirectScaledIndexedDisplaced(RBX, RBX, Two, 1812698508, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 92, 132, 91, 140, 149, 11, 108], OperandSize::Qword)
}

