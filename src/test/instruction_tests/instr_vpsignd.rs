use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpsignd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSIGND, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 97, 10, 232], OperandSize::Dword)
}

#[test]
fn vpsignd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSIGND, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledIndexedDisplaced(EDX, ECX, Four, 1799277846, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 89, 10, 164, 138, 22, 205, 62, 107], OperandSize::Dword)
}

#[test]
fn vpsignd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSIGND, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 97, 10, 250], OperandSize::Qword)
}

#[test]
fn vpsignd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSIGND, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledIndexed(RCX, RAX, Eight, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 10, 20, 193], OperandSize::Qword)
}

#[test]
fn vpsignd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSIGND, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 117, 10, 212], OperandSize::Dword)
}

#[test]
fn vpsignd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSIGND, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectDisplaced(ECX, 1201462093, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 10, 161, 77, 219, 156, 71], OperandSize::Dword)
}

#[test]
fn vpsignd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSIGND, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(YMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 85, 10, 205], OperandSize::Qword)
}

#[test]
fn vpsignd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSIGND, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectScaledIndexed(RDX, RDX, Four, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 85, 10, 28, 146], OperandSize::Qword)
}

