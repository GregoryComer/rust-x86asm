use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vblendpd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VBLENDPD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM4)), operand4: Some(Literal8(84)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 81, 13, 236, 84], OperandSize::Dword)
}

#[test]
fn vblendpd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VBLENDPD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectScaledIndexed(EDX, ESI, Two, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(59)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 113, 13, 20, 114, 59], OperandSize::Dword)
}

#[test]
fn vblendpd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VBLENDPD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM2)), operand4: Some(Literal8(19)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 121, 13, 202, 19], OperandSize::Qword)
}

#[test]
fn vblendpd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VBLENDPD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectScaledIndexedDisplaced(RCX, RDX, Eight, 637437334, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(113)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 113, 13, 172, 209, 150, 133, 254, 37, 113], OperandSize::Qword)
}

#[test]
fn vblendpd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VBLENDPD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(YMM0)), operand4: Some(Literal8(60)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 85, 13, 248, 60], OperandSize::Dword)
}

#[test]
fn vblendpd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VBLENDPD, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectDisplaced(EAX, 1914341569, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(83)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 125, 13, 160, 193, 136, 26, 114, 83], OperandSize::Dword)
}

#[test]
fn vblendpd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VBLENDPD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(YMM7)), operand4: Some(Literal8(29)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 101, 13, 239, 29], OperandSize::Qword)
}

#[test]
fn vblendpd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VBLENDPD, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectScaledIndexedDisplaced(RCX, RCX, Two, 1207338629, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(74)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 77, 13, 148, 73, 133, 134, 246, 71, 74], OperandSize::Qword)
}

