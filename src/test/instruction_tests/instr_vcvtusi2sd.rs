use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vcvtusi2sd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTUSI2SD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(EBX)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 111, 8, 123, 203], OperandSize::Dword)
}

#[test]
fn vcvtusi2sd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTUSI2SD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledIndexed(ESI, EDI, Two, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 127, 8, 123, 20, 126], OperandSize::Dword)
}

#[test]
fn vcvtusi2sd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTUSI2SD, operand1: Some(Direct(XMM13)), operand2: Some(Direct(XMM26)), operand3: Some(Direct(EDI)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 113, 47, 0, 123, 239], OperandSize::Qword)
}

#[test]
fn vcvtusi2sd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTUSI2SD, operand1: Some(Direct(XMM16)), operand2: Some(Direct(XMM14)), operand3: Some(IndirectScaledIndexedDisplaced(RCX, RSI, Four, 1103949059, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 225, 15, 8, 123, 132, 177, 3, 237, 204, 65], OperandSize::Qword)
}

#[test]
fn vcvtusi2sd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTUSI2SD, operand1: Some(Direct(XMM21)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(RCX)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Down), merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 225, 199, 56, 123, 233], OperandSize::Qword)
}

#[test]
fn vcvtusi2sd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTUSI2SD, operand1: Some(Direct(XMM8)), operand2: Some(Direct(XMM23)), operand3: Some(IndirectScaledDisplaced(RSI, Two, 447081381, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 113, 199, 0, 123, 4, 117, 165, 235, 165, 26], OperandSize::Qword)
}

