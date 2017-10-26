use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vmovlhps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVLHPS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 208, 22, 192], OperandSize::Dword)
}

#[test]
fn vmovlhps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVLHPS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 232, 22, 199], OperandSize::Qword)
}

#[test]
fn vmovlhps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVLHPS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 208, 22, 244], OperandSize::Dword)
}

#[test]
fn vmovlhps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVLHPS, operand1: Some(Direct(XMM18)), operand2: Some(Direct(XMM29)), operand3: Some(Direct(XMM17)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 161, 20, 0, 22, 209], OperandSize::Qword)
}

