use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn cvtss2sd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTSS2SD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 90, 242], OperandSize::Dword)
}

#[test]
fn cvtss2sd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTSS2SD, operand1: Some(Direct(XMM1)), operand2: Some(IndirectScaledDisplaced(EAX, Eight, 159243935, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 90, 12, 197, 159, 222, 125, 9], OperandSize::Dword)
}

#[test]
fn cvtss2sd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTSS2SD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 90, 200], OperandSize::Qword)
}

#[test]
fn cvtss2sd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTSS2SD, operand1: Some(Direct(XMM4)), operand2: Some(IndirectScaledDisplaced(RDX, Four, 1348666670, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 90, 36, 149, 46, 5, 99, 80], OperandSize::Qword)
}

