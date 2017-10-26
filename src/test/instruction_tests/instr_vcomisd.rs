use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vcomisd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VCOMISD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 47, 239], OperandSize::Dword)
}

#[test]
fn vcomisd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VCOMISD, operand1: Some(Direct(XMM1)), operand2: Some(IndirectScaledIndexedDisplaced(ECX, ESI, Four, 717102568, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 47, 140, 177, 232, 29, 190, 42], OperandSize::Dword)
}

#[test]
fn vcomisd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VCOMISD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 47, 252], OperandSize::Qword)
}

#[test]
fn vcomisd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VCOMISD, operand1: Some(Direct(XMM6)), operand2: Some(IndirectScaledDisplaced(RAX, Four, 1414465799, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 47, 52, 133, 7, 9, 79, 84], OperandSize::Qword)
}

#[test]
fn vcomisd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VCOMISD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: true, mask: None, broadcast: None }, &[98, 241, 253, 24, 47, 217], OperandSize::Dword)
}

#[test]
fn vcomisd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VCOMISD, operand1: Some(Direct(XMM4)), operand2: Some(IndirectScaledDisplaced(EAX, Two, 1013442243, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 47, 36, 69, 195, 230, 103, 60], OperandSize::Dword)
}

#[test]
fn vcomisd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VCOMISD, operand1: Some(Direct(XMM24)), operand2: Some(Direct(XMM17)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: true, mask: None, broadcast: None }, &[98, 33, 253, 24, 47, 193], OperandSize::Qword)
}

#[test]
fn vcomisd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VCOMISD, operand1: Some(Direct(XMM9)), operand2: Some(IndirectDisplaced(RCX, 1396540012, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 121, 47, 137, 108, 130, 61, 83], OperandSize::Qword)
}

