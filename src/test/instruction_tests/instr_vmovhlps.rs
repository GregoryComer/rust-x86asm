use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vmovhlps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVHLPS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 232, 18, 203], OperandSize::Dword)
}

#[test]
fn vmovhlps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVHLPS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 192, 18, 227], OperandSize::Qword)
}

#[test]
fn vmovhlps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVHLPS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 232, 18, 239], OperandSize::Dword)
}

#[test]
fn vmovhlps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVHLPS, operand1: Some(Direct(XMM13)), operand2: Some(Direct(XMM23)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 113, 68, 0, 18, 236], OperandSize::Qword)
}

