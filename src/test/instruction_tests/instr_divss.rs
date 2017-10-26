use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn divss_1() {
    run_test(&Instruction { mnemonic: Mnemonic::DIVSS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 94, 242], OperandSize::Dword)
}

#[test]
fn divss_2() {
    run_test(&Instruction { mnemonic: Mnemonic::DIVSS, operand1: Some(Direct(XMM0)), operand2: Some(IndirectScaledIndexedDisplaced(ECX, EBX, Four, 1979377965, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 94, 132, 153, 45, 233, 250, 117], OperandSize::Dword)
}

#[test]
fn divss_3() {
    run_test(&Instruction { mnemonic: Mnemonic::DIVSS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 94, 218], OperandSize::Qword)
}

#[test]
fn divss_4() {
    run_test(&Instruction { mnemonic: Mnemonic::DIVSS, operand1: Some(Direct(XMM4)), operand2: Some(Indirect(RDX, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 94, 34], OperandSize::Qword)
}

