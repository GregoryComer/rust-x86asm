use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vucomisd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VUCOMISD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 46, 209], OperandSize::Dword)
}

#[test]
fn vucomisd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VUCOMISD, operand1: Some(Direct(XMM4)), operand2: Some(IndirectScaledIndexedDisplaced(EBX, ECX, Four, 1347250615, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 46, 164, 139, 183, 105, 77, 80], OperandSize::Dword)
}

#[test]
fn vucomisd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VUCOMISD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 46, 240], OperandSize::Qword)
}

#[test]
fn vucomisd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VUCOMISD, operand1: Some(Direct(XMM5)), operand2: Some(IndirectScaledIndexed(RAX, RSI, Four, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 46, 44, 176], OperandSize::Qword)
}

#[test]
fn vucomisd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VUCOMISD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: true, mask: None, broadcast: None }, &[98, 241, 253, 24, 46, 209], OperandSize::Dword)
}

#[test]
fn vucomisd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VUCOMISD, operand1: Some(Direct(XMM1)), operand2: Some(IndirectDisplaced(ESI, 2037710206, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 46, 142, 126, 253, 116, 121], OperandSize::Dword)
}

#[test]
fn vucomisd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VUCOMISD, operand1: Some(Direct(XMM8)), operand2: Some(Direct(XMM13)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: true, mask: None, broadcast: None }, &[98, 81, 253, 24, 46, 197], OperandSize::Qword)
}

#[test]
fn vucomisd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VUCOMISD, operand1: Some(Direct(XMM13)), operand2: Some(IndirectScaledDisplaced(RBX, Two, 2113261158, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 121, 46, 44, 93, 102, 206, 245, 125], OperandSize::Qword)
}

