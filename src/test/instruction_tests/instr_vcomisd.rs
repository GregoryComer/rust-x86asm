use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vcomisd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VCOMISD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 47, 236], OperandSize::Dword)
}

#[test]
fn vcomisd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VCOMISD, operand1: Some(Direct(XMM6)), operand2: Some(IndirectDisplaced(ECX, 997045741, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 47, 177, 237, 181, 109, 59], OperandSize::Dword)
}

#[test]
fn vcomisd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VCOMISD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 47, 252], OperandSize::Qword)
}

#[test]
fn vcomisd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VCOMISD, operand1: Some(Direct(XMM4)), operand2: Some(Indirect(RCX, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 47, 33], OperandSize::Qword)
}

#[test]
fn vcomisd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VCOMISD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: true, mask: None, broadcast: None }, &[98, 241, 253, 24, 47, 193], OperandSize::Dword)
}

#[test]
fn vcomisd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VCOMISD, operand1: Some(Direct(XMM0)), operand2: Some(IndirectScaledIndexedDisplaced(EBX, ESI, Eight, 1905487649, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 47, 132, 243, 33, 111, 147, 113], OperandSize::Dword)
}

#[test]
fn vcomisd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VCOMISD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM31)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: true, mask: None, broadcast: None }, &[98, 145, 253, 24, 47, 223], OperandSize::Qword)
}

#[test]
fn vcomisd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VCOMISD, operand1: Some(Direct(XMM16)), operand2: Some(Indirect(RDI, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 225, 253, 8, 47, 7], OperandSize::Qword)
}

