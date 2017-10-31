use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vmovhlps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVHLPS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 200, 18, 239], OperandSize::Dword)
}

#[test]
fn vmovhlps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVHLPS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 240, 18, 219], OperandSize::Qword)
}

#[test]
fn vmovhlps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVHLPS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 208, 18, 242], OperandSize::Dword)
}

#[test]
fn vmovhlps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVHLPS, operand1: Some(Direct(XMM9)), operand2: Some(Direct(XMM22)), operand3: Some(Direct(XMM30)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 17, 76, 0, 18, 206], OperandSize::Qword)
}

