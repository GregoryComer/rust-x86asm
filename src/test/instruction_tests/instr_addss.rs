use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn addss_1() {
    run_test(&Instruction { mnemonic: Mnemonic::ADDSS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 88, 255], OperandSize::Dword)
}

#[test]
fn addss_2() {
    run_test(&Instruction { mnemonic: Mnemonic::ADDSS, operand1: Some(Direct(XMM4)), operand2: Some(IndirectScaledIndexedDisplaced(ESI, EBX, Eight, 1875011077, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 88, 164, 222, 5, 102, 194, 111], OperandSize::Dword)
}

#[test]
fn addss_3() {
    run_test(&Instruction { mnemonic: Mnemonic::ADDSS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 88, 244], OperandSize::Qword)
}

#[test]
fn addss_4() {
    run_test(&Instruction { mnemonic: Mnemonic::ADDSS, operand1: Some(Direct(XMM1)), operand2: Some(IndirectScaledIndexed(RDI, RBX, Four, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 88, 12, 159], OperandSize::Qword)
}

