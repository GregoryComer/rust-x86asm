use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn movlps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVLPS, operand1: Some(Direct(XMM3)), operand2: Some(IndirectDisplaced(ESI, 1629356529, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 18, 158, 241, 1, 30, 97], OperandSize::Dword)
}

#[test]
fn movlps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVLPS, operand1: Some(Direct(XMM0)), operand2: Some(IndirectScaledIndexedDisplaced(RBX, RSI, Two, 1651022664, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 18, 132, 115, 72, 155, 104, 98], OperandSize::Qword)
}

#[test]
fn movlps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVLPS, operand1: Some(IndirectScaledIndexedDisplaced(EDI, EDX, Two, 1354480687, Some(OperandSize::Qword), None)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 19, 172, 87, 47, 188, 187, 80], OperandSize::Dword)
}

#[test]
fn movlps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVLPS, operand1: Some(Indirect(RSI, Some(OperandSize::Qword), None)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 19, 30], OperandSize::Qword)
}

