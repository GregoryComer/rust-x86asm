use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vblendpd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VBLENDPD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM0)), operand4: Some(Literal8(76)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 81, 13, 192, 76], OperandSize::Dword)
}

#[test]
fn vblendpd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VBLENDPD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM7)), operand3: Some(Indirect(EBX, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(15)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 65, 13, 35, 15], OperandSize::Dword)
}

#[test]
fn vblendpd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VBLENDPD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM7)), operand4: Some(Literal8(26)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 97, 13, 255, 26], OperandSize::Qword)
}

#[test]
fn vblendpd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VBLENDPD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledIndexed(RDX, RCX, Eight, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(95)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 65, 13, 44, 202, 95], OperandSize::Qword)
}

#[test]
fn vblendpd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VBLENDPD, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM1)), operand4: Some(Literal8(18)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 117, 13, 193, 18], OperandSize::Dword)
}

#[test]
fn vblendpd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VBLENDPD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectScaledIndexedDisplaced(ECX, ECX, Eight, 1323979963, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(71)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 69, 13, 172, 201, 187, 84, 234, 78, 71], OperandSize::Dword)
}

#[test]
fn vblendpd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VBLENDPD, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM3)), operand4: Some(Literal8(75)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 93, 13, 227, 75], OperandSize::Qword)
}

#[test]
fn vblendpd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VBLENDPD, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM5)), operand3: Some(Indirect(RDX, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(17)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 85, 13, 10, 17], OperandSize::Qword)
}

