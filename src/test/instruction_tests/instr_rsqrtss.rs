use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn rsqrtss_1() {
    run_test(&Instruction { mnemonic: Mnemonic::RSQRTSS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 82, 232], OperandSize::Dword)
}

#[test]
fn rsqrtss_2() {
    run_test(&Instruction { mnemonic: Mnemonic::RSQRTSS, operand1: Some(Direct(XMM4)), operand2: Some(IndirectScaledIndexedDisplaced(ESI, EAX, Eight, 195355551, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 82, 164, 198, 159, 227, 164, 11], OperandSize::Dword)
}

#[test]
fn rsqrtss_3() {
    run_test(&Instruction { mnemonic: Mnemonic::RSQRTSS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 82, 196], OperandSize::Qword)
}

#[test]
fn rsqrtss_4() {
    run_test(&Instruction { mnemonic: Mnemonic::RSQRTSS, operand1: Some(Direct(XMM3)), operand2: Some(IndirectScaledIndexed(RDI, RBX, Four, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 82, 28, 159], OperandSize::Qword)
}

