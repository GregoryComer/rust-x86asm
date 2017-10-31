use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn rcpss_1() {
    run_test(&Instruction { mnemonic: Mnemonic::RCPSS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 83, 207], OperandSize::Dword)
}

#[test]
fn rcpss_2() {
    run_test(&Instruction { mnemonic: Mnemonic::RCPSS, operand1: Some(Direct(XMM3)), operand2: Some(IndirectScaledIndexed(EBX, EDX, Four, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 83, 28, 147], OperandSize::Dword)
}

#[test]
fn rcpss_3() {
    run_test(&Instruction { mnemonic: Mnemonic::RCPSS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 83, 245], OperandSize::Qword)
}

#[test]
fn rcpss_4() {
    run_test(&Instruction { mnemonic: Mnemonic::RCPSS, operand1: Some(Direct(XMM1)), operand2: Some(Indirect(RDX, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 83, 10], OperandSize::Qword)
}

