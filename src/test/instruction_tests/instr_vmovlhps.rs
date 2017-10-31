use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vmovlhps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVLHPS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 192, 22, 226], OperandSize::Dword)
}

#[test]
fn vmovlhps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVLHPS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 232, 22, 216], OperandSize::Qword)
}

#[test]
fn vmovlhps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVLHPS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 216, 22, 231], OperandSize::Dword)
}

#[test]
fn vmovlhps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVLHPS, operand1: Some(Direct(XMM31)), operand2: Some(Direct(XMM21)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 97, 84, 0, 22, 248], OperandSize::Qword)
}

