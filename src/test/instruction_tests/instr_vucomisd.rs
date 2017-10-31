use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vucomisd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VUCOMISD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 46, 199], OperandSize::Dword)
}

#[test]
fn vucomisd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VUCOMISD, operand1: Some(Direct(XMM3)), operand2: Some(IndirectScaledDisplaced(EBX, Two, 1944297128, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 46, 28, 93, 168, 158, 227, 115], OperandSize::Dword)
}

#[test]
fn vucomisd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VUCOMISD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 46, 251], OperandSize::Qword)
}

#[test]
fn vucomisd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VUCOMISD, operand1: Some(Direct(XMM1)), operand2: Some(IndirectDisplaced(RDI, 1742824905, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 46, 143, 201, 101, 225, 103], OperandSize::Qword)
}

#[test]
fn vucomisd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VUCOMISD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: true, mask: None, broadcast: None }, &[98, 241, 253, 24, 46, 217], OperandSize::Dword)
}

#[test]
fn vucomisd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VUCOMISD, operand1: Some(Direct(XMM2)), operand2: Some(Indirect(EBX, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 46, 19], OperandSize::Dword)
}

#[test]
fn vucomisd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VUCOMISD, operand1: Some(Direct(XMM27)), operand2: Some(Direct(XMM15)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: true, mask: None, broadcast: None }, &[98, 65, 253, 24, 46, 223], OperandSize::Qword)
}

#[test]
fn vucomisd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VUCOMISD, operand1: Some(Direct(XMM14)), operand2: Some(IndirectScaledIndexed(RCX, RSI, Two, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 121, 46, 52, 113], OperandSize::Qword)
}

