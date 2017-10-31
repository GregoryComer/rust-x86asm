use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpcmpw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPW, operand1: Some(Direct(K6)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM4)), operand4: Some(Literal8(16)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 243, 245, 12, 63, 244, 16], OperandSize::Dword)
}

#[test]
fn vpcmpw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPW, operand1: Some(Direct(K5)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledIndexed(EDI, EAX, Two, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(122)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 243, 197, 14, 63, 44, 71, 122], OperandSize::Dword)
}

#[test]
fn vpcmpw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPW, operand1: Some(Direct(K1)), operand2: Some(Direct(XMM21)), operand3: Some(Direct(XMM4)), operand4: Some(Literal8(8)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 243, 213, 5, 63, 204, 8], OperandSize::Qword)
}

#[test]
fn vpcmpw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPW, operand1: Some(Direct(K4)), operand2: Some(Direct(XMM15)), operand3: Some(IndirectScaledIndexedDisplaced(RCX, RAX, Eight, 991418578, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(44)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 243, 133, 12, 63, 164, 193, 210, 216, 23, 59, 44], OperandSize::Qword)
}

#[test]
fn vpcmpw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPW, operand1: Some(Direct(K2)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM4)), operand4: Some(Literal8(34)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 243, 245, 47, 63, 212, 34], OperandSize::Dword)
}

#[test]
fn vpcmpw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPW, operand1: Some(Direct(K1)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectScaledIndexedDisplaced(EDI, EBX, Eight, 255770447, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(28)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 243, 213, 46, 63, 140, 223, 79, 191, 62, 15, 28], OperandSize::Dword)
}

#[test]
fn vpcmpw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPW, operand1: Some(Direct(K6)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(YMM28)), operand4: Some(Literal8(24)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 147, 205, 45, 63, 244, 24], OperandSize::Qword)
}

#[test]
fn vpcmpw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPW, operand1: Some(Direct(K5)), operand2: Some(Direct(YMM18)), operand3: Some(IndirectScaledIndexed(RAX, RBX, Four, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(93)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 243, 237, 38, 63, 44, 152, 93], OperandSize::Qword)
}

#[test]
fn vpcmpw_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPW, operand1: Some(Direct(K7)), operand2: Some(Direct(ZMM7)), operand3: Some(Direct(ZMM7)), operand4: Some(Literal8(115)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 243, 197, 73, 63, 255, 115], OperandSize::Dword)
}

#[test]
fn vpcmpw_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPW, operand1: Some(Direct(K5)), operand2: Some(Direct(ZMM1)), operand3: Some(IndirectDisplaced(ECX, 264019599, Some(OperandSize::Zmmword), None)), operand4: Some(Literal8(38)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 243, 245, 74, 63, 169, 143, 158, 188, 15, 38], OperandSize::Dword)
}

#[test]
fn vpcmpw_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPW, operand1: Some(Direct(K7)), operand2: Some(Direct(ZMM8)), operand3: Some(Direct(ZMM4)), operand4: Some(Literal8(40)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 243, 189, 75, 63, 252, 40], OperandSize::Qword)
}

#[test]
fn vpcmpw_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPW, operand1: Some(Direct(K6)), operand2: Some(Direct(ZMM0)), operand3: Some(IndirectScaledIndexedDisplaced(RDX, RBX, Four, 775133740, Some(OperandSize::Zmmword), None)), operand4: Some(Literal8(72)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 243, 253, 74, 63, 180, 154, 44, 154, 51, 46, 72], OperandSize::Qword)
}

