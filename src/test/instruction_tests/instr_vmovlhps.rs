use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vmovlhps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVLHPS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 208, 22, 195], OperandSize::Dword)
}

#[test]
fn vmovlhps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVLHPS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 216, 22, 196], OperandSize::Qword)
}

#[test]
fn vmovlhps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVLHPS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 240, 22, 192], OperandSize::Dword)
}

#[test]
fn vmovlhps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVLHPS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM10)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 193, 80, 22, 226], OperandSize::Qword)
}

