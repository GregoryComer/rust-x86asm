use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpcmpuw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPUW, operand1: Some(Direct(K2)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM0)), operand4: Some(Literal8(11)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 243, 213, 12, 62, 208, 11], OperandSize::Dword)
}

#[test]
fn vpcmpuw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPUW, operand1: Some(Direct(K3)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledIndexedDisplaced(EDI, EBX, Two, 1070035944, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(94)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 243, 237, 13, 62, 156, 95, 232, 115, 199, 63, 94], OperandSize::Dword)
}

#[test]
fn vpcmpuw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPUW, operand1: Some(Direct(K2)), operand2: Some(Direct(XMM14)), operand3: Some(Direct(XMM3)), operand4: Some(Literal8(42)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 243, 141, 12, 62, 211, 42], OperandSize::Qword)
}

#[test]
fn vpcmpuw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPUW, operand1: Some(Direct(K4)), operand2: Some(Direct(XMM23)), operand3: Some(IndirectScaledIndexedDisplaced(RAX, RDX, Eight, 671094508, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(66)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 243, 197, 1, 62, 164, 208, 236, 22, 0, 40, 66], OperandSize::Qword)
}

#[test]
fn vpcmpuw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPUW, operand1: Some(Direct(K1)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(YMM0)), operand4: Some(Literal8(2)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 243, 229, 45, 62, 200, 2], OperandSize::Dword)
}

#[test]
fn vpcmpuw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPUW, operand1: Some(Direct(K2)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectScaledIndexedDisplaced(EAX, EAX, Two, 192133053, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(29)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 243, 229, 47, 62, 148, 64, 189, 183, 115, 11, 29], OperandSize::Dword)
}

#[test]
fn vpcmpuw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPUW, operand1: Some(Direct(K3)), operand2: Some(Direct(YMM12)), operand3: Some(Direct(YMM0)), operand4: Some(Literal8(31)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 243, 157, 45, 62, 216, 31], OperandSize::Qword)
}

#[test]
fn vpcmpuw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPUW, operand1: Some(Direct(K6)), operand2: Some(Direct(YMM14)), operand3: Some(IndirectScaledIndexed(RDX, RDI, Eight, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(109)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 243, 141, 45, 62, 52, 250, 109], OperandSize::Qword)
}

#[test]
fn vpcmpuw_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPUW, operand1: Some(Direct(K4)), operand2: Some(Direct(ZMM5)), operand3: Some(Direct(ZMM6)), operand4: Some(Literal8(106)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 243, 213, 78, 62, 230, 106], OperandSize::Dword)
}

#[test]
fn vpcmpuw_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPUW, operand1: Some(Direct(K7)), operand2: Some(Direct(ZMM7)), operand3: Some(IndirectDisplaced(EDI, 1989686385, Some(OperandSize::Zmmword), None)), operand4: Some(Literal8(97)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 243, 197, 74, 62, 191, 113, 52, 152, 118, 97], OperandSize::Dword)
}

#[test]
fn vpcmpuw_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPUW, operand1: Some(Direct(K7)), operand2: Some(Direct(ZMM24)), operand3: Some(Direct(ZMM24)), operand4: Some(Literal8(13)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 147, 189, 65, 62, 248, 13], OperandSize::Qword)
}

#[test]
fn vpcmpuw_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPUW, operand1: Some(Direct(K2)), operand2: Some(Direct(ZMM24)), operand3: Some(IndirectDisplaced(RCX, 568550207, Some(OperandSize::Zmmword), None)), operand4: Some(Literal8(92)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 243, 189, 67, 62, 145, 63, 99, 227, 33, 92], OperandSize::Qword)
}

