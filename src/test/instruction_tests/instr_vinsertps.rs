use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vinsertps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VINSERTPS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM7)), operand4: Some(Literal8(42)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 65, 33, 231, 42], OperandSize::Dword)
}

#[test]
fn vinsertps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VINSERTPS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectScaledIndexed(EBX, EAX, Two, Some(OperandSize::Dword), None)), operand4: Some(Literal8(37)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 113, 33, 28, 67, 37], OperandSize::Dword)
}

#[test]
fn vinsertps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VINSERTPS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM7)), operand4: Some(Literal8(50)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 113, 33, 199, 50], OperandSize::Qword)
}

#[test]
fn vinsertps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VINSERTPS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledIndexed(RCX, RDI, Eight, Some(OperandSize::Dword), None)), operand4: Some(Literal8(77)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 65, 33, 36, 249, 77], OperandSize::Qword)
}

#[test]
fn vinsertps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VINSERTPS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM7)), operand4: Some(Literal8(105)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 81, 33, 215, 105], OperandSize::Dword)
}

#[test]
fn vinsertps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VINSERTPS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM2)), operand3: Some(Indirect(EBX, Some(OperandSize::Dword), None)), operand4: Some(Literal8(103)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 105, 33, 35, 103], OperandSize::Dword)
}

#[test]
fn vinsertps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VINSERTPS, operand1: Some(Direct(XMM29)), operand2: Some(Direct(XMM20)), operand3: Some(Direct(XMM16)), operand4: Some(Literal8(70)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 35, 93, 0, 33, 232, 70], OperandSize::Qword)
}

#[test]
fn vinsertps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VINSERTPS, operand1: Some(Direct(XMM12)), operand2: Some(Direct(XMM24)), operand3: Some(IndirectScaledDisplaced(RDI, Eight, 652781706, Some(OperandSize::Dword), None)), operand4: Some(Literal8(48)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 115, 61, 0, 33, 36, 253, 138, 168, 232, 38, 48], OperandSize::Qword)
}

