use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn subss_1() {
    run_test(&Instruction { mnemonic: Mnemonic::SUBSS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 92, 225], OperandSize::Dword)
}

#[test]
fn subss_2() {
    run_test(&Instruction { mnemonic: Mnemonic::SUBSS, operand1: Some(Direct(XMM6)), operand2: Some(IndirectScaledIndexed(EDI, EDI, Two, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 92, 52, 127], OperandSize::Dword)
}

#[test]
fn subss_3() {
    run_test(&Instruction { mnemonic: Mnemonic::SUBSS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 92, 237], OperandSize::Qword)
}

#[test]
fn subss_4() {
    run_test(&Instruction { mnemonic: Mnemonic::SUBSS, operand1: Some(Direct(XMM5)), operand2: Some(IndirectScaledIndexedDisplaced(RDI, RCX, Four, 409791948, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 92, 172, 143, 204, 237, 108, 24], OperandSize::Qword)
}

