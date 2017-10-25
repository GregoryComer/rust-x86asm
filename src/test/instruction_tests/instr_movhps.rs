use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn movhps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVHPS, operand1: Some(Direct(XMM5)), operand2: Some(IndirectDisplaced(ESI, 642052924, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 22, 174, 60, 243, 68, 38], OperandSize::Dword)
}

#[test]
fn movhps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVHPS, operand1: Some(Direct(XMM3)), operand2: Some(IndirectScaledIndexedDisplaced(RDI, RDI, Two, 1060147671, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 22, 156, 127, 215, 145, 48, 63], OperandSize::Qword)
}

#[test]
fn movhps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVHPS, operand1: Some(IndirectScaledIndexedDisplaced(EDX, ESI, Two, 1757537560, Some(OperandSize::Qword), None)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 23, 156, 114, 24, 229, 193, 104], OperandSize::Dword)
}

#[test]
fn movhps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVHPS, operand1: Some(IndirectScaledIndexedDisplaced(RBX, RSI, Four, 1678546189, Some(OperandSize::Qword), None)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 23, 172, 179, 13, 149, 12, 100], OperandSize::Qword)
}

