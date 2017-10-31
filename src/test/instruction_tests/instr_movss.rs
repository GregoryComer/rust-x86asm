use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn movss_1() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVSS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 16, 250], OperandSize::Dword)
}

#[test]
fn movss_2() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVSS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 16, 230], OperandSize::Qword)
}

#[test]
fn movss_3() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVSS, operand1: Some(Direct(XMM1)), operand2: Some(IndirectScaledIndexedDisplaced(ECX, ESI, Four, 1439595338, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 16, 140, 177, 74, 123, 206, 85], OperandSize::Dword)
}

#[test]
fn movss_4() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVSS, operand1: Some(Direct(XMM2)), operand2: Some(IndirectScaledIndexed(RDX, RSI, Eight, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 16, 20, 242], OperandSize::Qword)
}

#[test]
fn movss_5() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVSS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 16, 204], OperandSize::Dword)
}

#[test]
fn movss_6() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVSS, operand1: Some(IndirectScaledDisplaced(EBX, Two, 1816417241, Some(OperandSize::Dword), None)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 17, 52, 93, 217, 83, 68, 108], OperandSize::Dword)
}

#[test]
fn movss_7() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVSS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 16, 215], OperandSize::Qword)
}

#[test]
fn movss_8() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVSS, operand1: Some(IndirectScaledIndexedDisplaced(RAX, RDI, Four, 1243741918, Some(OperandSize::Dword), None)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 17, 172, 184, 222, 254, 33, 74], OperandSize::Qword)
}

