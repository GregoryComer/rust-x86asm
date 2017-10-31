use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vcvtusi2sd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTUSI2SD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(EDX)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 87, 8, 123, 242], OperandSize::Dword)
}

#[test]
fn vcvtusi2sd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTUSI2SD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectScaledIndexedDisplaced(EBX, ECX, Two, 288346097, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 119, 8, 123, 180, 75, 241, 207, 47, 17], OperandSize::Dword)
}

#[test]
fn vcvtusi2sd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTUSI2SD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM27)), operand3: Some(Direct(EDX)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 39, 0, 123, 250], OperandSize::Qword)
}

#[test]
fn vcvtusi2sd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTUSI2SD, operand1: Some(Direct(XMM22)), operand2: Some(Direct(XMM22)), operand3: Some(IndirectScaledIndexedDisplaced(RDX, RBX, Four, 2026894608, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 225, 79, 0, 123, 180, 154, 16, 245, 207, 120], OperandSize::Qword)
}

#[test]
fn vcvtusi2sd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTUSI2SD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(RBP)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Zero), merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 199, 120, 123, 205], OperandSize::Qword)
}

#[test]
fn vcvtusi2sd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTUSI2SD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM20)), operand3: Some(Indirect(RBX, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 223, 0, 123, 59], OperandSize::Qword)
}

