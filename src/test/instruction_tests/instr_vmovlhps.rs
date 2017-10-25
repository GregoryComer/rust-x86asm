use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vmovlhps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVLHPS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 240, 22, 239], OperandSize::Dword)
}

#[test]
fn vmovlhps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVLHPS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 22, 197], OperandSize::Qword)
}

#[test]
fn vmovlhps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVLHPS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 208, 22, 254], OperandSize::Dword)
}

#[test]
fn vmovlhps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVLHPS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM15)), operand3: Some(Direct(XMM31)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 145, 4, 8, 22, 215], OperandSize::Qword)
}

