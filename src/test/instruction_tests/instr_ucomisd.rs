use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn ucomisd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::UCOMISD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 46, 216], OperandSize::Dword)
}

#[test]
fn ucomisd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::UCOMISD, operand1: Some(Direct(XMM2)), operand2: Some(IndirectScaledIndexedDisplaced(ECX, ESI, Two, 1889637125, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 46, 148, 113, 5, 147, 161, 112], OperandSize::Dword)
}

#[test]
fn ucomisd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::UCOMISD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 46, 222], OperandSize::Qword)
}

#[test]
fn ucomisd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::UCOMISD, operand1: Some(Direct(XMM0)), operand2: Some(IndirectScaledIndexedDisplaced(RAX, RSI, Four, 1323464136, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 46, 132, 176, 200, 117, 226, 78], OperandSize::Qword)
}

