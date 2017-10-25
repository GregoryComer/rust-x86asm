use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn pcmpestri_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PCMPESTRI, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM7)), operand3: Some(Literal8(88)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 97, 215, 88], OperandSize::Dword)
}

#[test]
fn pcmpestri_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PCMPESTRI, operand1: Some(Direct(XMM5)), operand2: Some(IndirectScaledIndexedDisplaced(EDX, EAX, Eight, 1759094786, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(30)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 97, 172, 194, 2, 168, 217, 104, 30], OperandSize::Dword)
}

#[test]
fn pcmpestri_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PCMPESTRI, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM7)), operand3: Some(Literal8(99)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 97, 247, 99], OperandSize::Qword)
}

#[test]
fn pcmpestri_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PCMPESTRI, operand1: Some(Direct(XMM5)), operand2: Some(IndirectScaledDisplaced(RAX, Eight, 111636743, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 97, 44, 197, 7, 113, 167, 6, 2], OperandSize::Qword)
}

