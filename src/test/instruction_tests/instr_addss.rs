use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn addss_1() {
    run_test(&Instruction { mnemonic: Mnemonic::ADDSS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 88, 229], OperandSize::Dword)
}

#[test]
fn addss_2() {
    run_test(&Instruction { mnemonic: Mnemonic::ADDSS, operand1: Some(Direct(XMM0)), operand2: Some(IndirectScaledIndexedDisplaced(EDX, ESI, Four, 2110132666, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 88, 132, 178, 186, 17, 198, 125], OperandSize::Dword)
}

#[test]
fn addss_3() {
    run_test(&Instruction { mnemonic: Mnemonic::ADDSS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 88, 231], OperandSize::Qword)
}

#[test]
fn addss_4() {
    run_test(&Instruction { mnemonic: Mnemonic::ADDSS, operand1: Some(Direct(XMM2)), operand2: Some(IndirectScaledIndexedDisplaced(RDX, RSI, Two, 1596491686, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 88, 148, 114, 166, 135, 40, 95], OperandSize::Qword)
}

