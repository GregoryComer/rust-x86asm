use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn packusdw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PACKUSDW, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 43, 225], OperandSize::Dword)
}

#[test]
fn packusdw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PACKUSDW, operand1: Some(Direct(XMM6)), operand2: Some(IndirectScaledDisplaced(EDI, Eight, 1841349563, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 43, 52, 253, 187, 195, 192, 109], OperandSize::Dword)
}

#[test]
fn packusdw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PACKUSDW, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 43, 207], OperandSize::Qword)
}

#[test]
fn packusdw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PACKUSDW, operand1: Some(Direct(XMM2)), operand2: Some(IndirectScaledDisplaced(RAX, Four, 376855608, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 43, 20, 133, 56, 92, 118, 22], OperandSize::Qword)
}

