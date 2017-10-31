use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vcvtusi2sd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTUSI2SD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(ESI)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 119, 8, 123, 246], OperandSize::Dword)
}

#[test]
fn vcvtusi2sd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTUSI2SD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectScaledIndexedDisplaced(ESI, EDX, Four, 1664113374, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 79, 8, 123, 180, 150, 222, 90, 48, 99], OperandSize::Dword)
}

#[test]
fn vcvtusi2sd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTUSI2SD, operand1: Some(Direct(XMM25)), operand2: Some(Direct(XMM10)), operand3: Some(Direct(EDX)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 97, 47, 8, 123, 202], OperandSize::Qword)
}

#[test]
fn vcvtusi2sd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTUSI2SD, operand1: Some(Direct(XMM22)), operand2: Some(Direct(XMM31)), operand3: Some(IndirectScaledDisplaced(RCX, Eight, 1745832660, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 225, 7, 0, 123, 52, 205, 212, 74, 15, 104], OperandSize::Qword)
}

#[test]
fn vcvtusi2sd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTUSI2SD, operand1: Some(Direct(XMM25)), operand2: Some(Direct(XMM24)), operand3: Some(Direct(RSI)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Down), merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 97, 191, 48, 123, 206], OperandSize::Qword)
}

#[test]
fn vcvtusi2sd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTUSI2SD, operand1: Some(Direct(XMM19)), operand2: Some(Direct(XMM21)), operand3: Some(Indirect(RDX, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 225, 215, 0, 123, 26], OperandSize::Qword)
}

