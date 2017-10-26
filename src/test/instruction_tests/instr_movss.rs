use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn movss_1() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVSS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 16, 216], OperandSize::Dword)
}

#[test]
fn movss_2() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVSS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 16, 254], OperandSize::Qword)
}

#[test]
fn movss_3() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVSS, operand1: Some(Direct(XMM5)), operand2: Some(IndirectScaledIndexed(EAX, EDI, Two, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 16, 44, 120], OperandSize::Dword)
}

#[test]
fn movss_4() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVSS, operand1: Some(Direct(XMM4)), operand2: Some(IndirectScaledIndexed(RBX, RDI, Eight, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 16, 36, 251], OperandSize::Qword)
}

#[test]
fn movss_5() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVSS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 16, 215], OperandSize::Dword)
}

#[test]
fn movss_6() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVSS, operand1: Some(Indirect(EBX, Some(OperandSize::Dword), None)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 17, 59], OperandSize::Dword)
}

#[test]
fn movss_7() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVSS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 16, 230], OperandSize::Qword)
}

#[test]
fn movss_8() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVSS, operand1: Some(IndirectScaledIndexed(RCX, RCX, Eight, Some(OperandSize::Dword), None)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 17, 36, 201], OperandSize::Qword)
}

