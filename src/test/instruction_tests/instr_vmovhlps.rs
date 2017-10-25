use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vmovhlps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVHLPS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 216, 18, 247], OperandSize::Dword)
}

#[test]
fn vmovhlps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVHLPS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 224, 18, 221], OperandSize::Qword)
}

#[test]
fn vmovhlps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVHLPS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 208, 18, 216], OperandSize::Dword)
}

#[test]
fn vmovhlps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVHLPS, operand1: Some(Direct(XMM26)), operand2: Some(Direct(XMM26)), operand3: Some(Direct(XMM25)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 1, 44, 0, 18, 209], OperandSize::Qword)
}

