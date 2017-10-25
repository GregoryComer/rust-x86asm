use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn cvtss2sd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTSS2SD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 90, 249], OperandSize::Dword)
}

#[test]
fn cvtss2sd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTSS2SD, operand1: Some(Direct(XMM4)), operand2: Some(IndirectScaledDisplaced(EDI, Four, 863716284, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 90, 36, 189, 188, 67, 123, 51], OperandSize::Dword)
}

#[test]
fn cvtss2sd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTSS2SD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 90, 229], OperandSize::Qword)
}

#[test]
fn cvtss2sd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTSS2SD, operand1: Some(Direct(XMM0)), operand2: Some(IndirectScaledDisplaced(RCX, Eight, 1107658084, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 90, 4, 205, 100, 133, 5, 66], OperandSize::Qword)
}

