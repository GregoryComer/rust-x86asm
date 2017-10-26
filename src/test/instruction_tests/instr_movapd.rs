use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn movapd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVAPD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 40, 200], OperandSize::Dword)
}

#[test]
fn movapd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVAPD, operand1: Some(Direct(XMM0)), operand2: Some(IndirectScaledIndexed(ESI, EDX, Two, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 40, 4, 86], OperandSize::Dword)
}

#[test]
fn movapd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVAPD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 40, 193], OperandSize::Qword)
}

#[test]
fn movapd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVAPD, operand1: Some(Direct(XMM0)), operand2: Some(Indirect(RDI, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 40, 7], OperandSize::Qword)
}

#[test]
fn movapd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVAPD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 40, 192], OperandSize::Dword)
}

#[test]
fn movapd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVAPD, operand1: Some(IndirectScaledIndexed(EBX, EDI, Four, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 41, 60, 187], OperandSize::Dword)
}

#[test]
fn movapd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVAPD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 40, 232], OperandSize::Qword)
}

#[test]
fn movapd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVAPD, operand1: Some(IndirectDisplaced(RDX, 1116496110, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 41, 162, 238, 96, 140, 66], OperandSize::Qword)
}

