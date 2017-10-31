use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn aesdeclast_1() {
    run_test(&Instruction { mnemonic: Mnemonic::AESDECLAST, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 223, 194], OperandSize::Dword)
}

#[test]
fn aesdeclast_2() {
    run_test(&Instruction { mnemonic: Mnemonic::AESDECLAST, operand1: Some(Direct(XMM7)), operand2: Some(IndirectScaledIndexed(EBX, ECX, Four, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 223, 60, 139], OperandSize::Dword)
}

#[test]
fn aesdeclast_3() {
    run_test(&Instruction { mnemonic: Mnemonic::AESDECLAST, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 223, 234], OperandSize::Qword)
}

#[test]
fn aesdeclast_4() {
    run_test(&Instruction { mnemonic: Mnemonic::AESDECLAST, operand1: Some(Direct(XMM4)), operand2: Some(IndirectScaledIndexed(RSI, RAX, Four, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 223, 36, 134], OperandSize::Qword)
}

