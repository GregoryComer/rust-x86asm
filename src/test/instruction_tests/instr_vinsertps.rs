use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vinsertps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VINSERTPS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM5)), operand4: Some(Literal8(36)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 81, 33, 245, 36], OperandSize::Dword)
}

#[test]
fn vinsertps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VINSERTPS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectDisplaced(EAX, 1257285100, Some(OperandSize::Dword), None)), operand4: Some(Literal8(66)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 113, 33, 144, 236, 165, 240, 74, 66], OperandSize::Dword)
}

#[test]
fn vinsertps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VINSERTPS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM7)), operand4: Some(Literal8(12)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 65, 33, 231, 12], OperandSize::Qword)
}

#[test]
fn vinsertps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VINSERTPS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectScaledIndexedDisplaced(RBX, RDX, Eight, 1466911157, Some(OperandSize::Dword), None)), operand4: Some(Literal8(4)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 113, 33, 188, 211, 181, 73, 111, 87, 4], OperandSize::Qword)
}

#[test]
fn vinsertps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VINSERTPS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM5)), operand4: Some(Literal8(80)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 89, 33, 253, 80], OperandSize::Dword)
}

#[test]
fn vinsertps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VINSERTPS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledIndexed(EDX, EAX, Two, Some(OperandSize::Dword), None)), operand4: Some(Literal8(36)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 121, 33, 28, 66, 36], OperandSize::Dword)
}

#[test]
fn vinsertps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VINSERTPS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM28)), operand3: Some(Direct(XMM12)), operand4: Some(Literal8(3)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 211, 29, 0, 33, 236, 3], OperandSize::Qword)
}

#[test]
fn vinsertps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VINSERTPS, operand1: Some(Direct(XMM20)), operand2: Some(Direct(XMM8)), operand3: Some(IndirectDisplaced(RSI, 122498000, Some(OperandSize::Dword), None)), operand4: Some(Literal8(99)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 227, 61, 8, 33, 166, 208, 43, 77, 7, 99], OperandSize::Qword)
}

