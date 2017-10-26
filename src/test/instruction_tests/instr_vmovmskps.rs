use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vmovmskps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVMSKPS, operand1: Some(Direct(ECX)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 80, 205], OperandSize::Dword)
}

#[test]
fn vmovmskps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVMSKPS, operand1: Some(Direct(RDX)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 80, 213], OperandSize::Qword)
}

#[test]
fn vmovmskps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVMSKPS, operand1: Some(Direct(EDI)), operand2: Some(Direct(YMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 252, 80, 252], OperandSize::Dword)
}

#[test]
fn vmovmskps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVMSKPS, operand1: Some(Direct(RSP)), operand2: Some(Direct(YMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 252, 80, 229], OperandSize::Qword)
}

