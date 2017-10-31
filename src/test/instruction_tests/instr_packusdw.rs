use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn packusdw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PACKUSDW, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 43, 244], OperandSize::Dword)
}

#[test]
fn packusdw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PACKUSDW, operand1: Some(Direct(XMM2)), operand2: Some(IndirectScaledIndexed(EBX, ECX, Eight, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 43, 20, 203], OperandSize::Dword)
}

#[test]
fn packusdw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PACKUSDW, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 43, 247], OperandSize::Qword)
}

#[test]
fn packusdw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PACKUSDW, operand1: Some(Direct(XMM1)), operand2: Some(IndirectScaledIndexedDisplaced(RAX, RBX, Four, 975661804, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 43, 140, 152, 236, 106, 39, 58], OperandSize::Qword)
}

