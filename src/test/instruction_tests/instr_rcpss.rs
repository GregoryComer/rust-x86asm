use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn rcpss_1() {
    run_test(&Instruction { mnemonic: Mnemonic::RCPSS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 83, 255], OperandSize::Dword)
}

#[test]
fn rcpss_2() {
    run_test(&Instruction { mnemonic: Mnemonic::RCPSS, operand1: Some(Direct(XMM2)), operand2: Some(Indirect(ESI, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 83, 22], OperandSize::Dword)
}

#[test]
fn rcpss_3() {
    run_test(&Instruction { mnemonic: Mnemonic::RCPSS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 83, 244], OperandSize::Qword)
}

#[test]
fn rcpss_4() {
    run_test(&Instruction { mnemonic: Mnemonic::RCPSS, operand1: Some(Direct(XMM4)), operand2: Some(IndirectScaledIndexedDisplaced(RSI, RDI, Eight, 896211856, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 83, 164, 254, 144, 27, 107, 53], OperandSize::Qword)
}

