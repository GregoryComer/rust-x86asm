use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn sqrtss_1() {
    run_test(&Instruction { mnemonic: Mnemonic::SQRTSS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 81, 239], OperandSize::Dword)
}

#[test]
fn sqrtss_2() {
    run_test(&Instruction { mnemonic: Mnemonic::SQRTSS, operand1: Some(Direct(XMM2)), operand2: Some(IndirectScaledIndexedDisplaced(ECX, EAX, Two, 1424447235, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 81, 148, 65, 3, 87, 231, 84], OperandSize::Dword)
}

#[test]
fn sqrtss_3() {
    run_test(&Instruction { mnemonic: Mnemonic::SQRTSS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 81, 228], OperandSize::Qword)
}

#[test]
fn sqrtss_4() {
    run_test(&Instruction { mnemonic: Mnemonic::SQRTSS, operand1: Some(Direct(XMM5)), operand2: Some(IndirectScaledIndexedDisplaced(RDX, RDX, Four, 531963920, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 81, 172, 146, 16, 32, 181, 31], OperandSize::Qword)
}

