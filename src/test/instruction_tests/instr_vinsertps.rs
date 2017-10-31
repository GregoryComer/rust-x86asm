use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vinsertps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VINSERTPS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM6)), operand4: Some(Literal8(97)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 81, 33, 230, 97], OperandSize::Dword)
}

#[test]
fn vinsertps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VINSERTPS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledIndexed(ECX, ESI, Two, Some(OperandSize::Dword), None)), operand4: Some(Literal8(82)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 89, 33, 52, 113, 82], OperandSize::Dword)
}

#[test]
fn vinsertps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VINSERTPS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM2)), operand4: Some(Literal8(39)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 97, 33, 218, 39], OperandSize::Qword)
}

#[test]
fn vinsertps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VINSERTPS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM5)), operand3: Some(Indirect(RDI, Some(OperandSize::Dword), None)), operand4: Some(Literal8(42)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 81, 33, 63, 42], OperandSize::Qword)
}

#[test]
fn vinsertps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VINSERTPS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM0)), operand4: Some(Literal8(1)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 105, 33, 224, 1], OperandSize::Dword)
}

#[test]
fn vinsertps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VINSERTPS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM6)), operand3: Some(Indirect(EDI, Some(OperandSize::Dword), None)), operand4: Some(Literal8(35)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 73, 33, 63, 35], OperandSize::Dword)
}

#[test]
fn vinsertps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VINSERTPS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM25)), operand4: Some(Literal8(91)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 147, 101, 8, 33, 233, 91], OperandSize::Qword)
}

#[test]
fn vinsertps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VINSERTPS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM30)), operand3: Some(IndirectScaledDisplaced(RBX, Four, 1940497705, Some(OperandSize::Dword), None)), operand4: Some(Literal8(111)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 243, 13, 0, 33, 36, 157, 41, 165, 169, 115, 111], OperandSize::Qword)
}

