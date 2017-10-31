use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vcomisd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VCOMISD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 47, 216], OperandSize::Dword)
}

#[test]
fn vcomisd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VCOMISD, operand1: Some(Direct(XMM2)), operand2: Some(IndirectScaledDisplaced(ECX, Eight, 1535988117, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 47, 20, 205, 149, 81, 141, 91], OperandSize::Dword)
}

#[test]
fn vcomisd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VCOMISD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 47, 232], OperandSize::Qword)
}

#[test]
fn vcomisd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VCOMISD, operand1: Some(Direct(XMM6)), operand2: Some(IndirectScaledDisplaced(RDI, Four, 692610895, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 47, 52, 189, 79, 103, 72, 41], OperandSize::Qword)
}

#[test]
fn vcomisd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VCOMISD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: true, mask: None, broadcast: None }, &[98, 241, 253, 24, 47, 216], OperandSize::Dword)
}

#[test]
fn vcomisd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VCOMISD, operand1: Some(Direct(XMM4)), operand2: Some(IndirectScaledIndexed(ECX, EDX, Two, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 47, 36, 81], OperandSize::Dword)
}

#[test]
fn vcomisd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VCOMISD, operand1: Some(Direct(XMM31)), operand2: Some(Direct(XMM12)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: true, mask: None, broadcast: None }, &[98, 65, 253, 24, 47, 252], OperandSize::Qword)
}

#[test]
fn vcomisd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VCOMISD, operand1: Some(Direct(XMM24)), operand2: Some(IndirectScaledIndexed(RBX, RCX, Four, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 97, 253, 8, 47, 4, 139], OperandSize::Qword)
}

