use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpcmpw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPW, operand1: Some(Direct(K5)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM3)), operand4: Some(Literal8(58)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 243, 213, 13, 63, 235, 58], OperandSize::Dword)
}

#[test]
fn vpcmpw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPW, operand1: Some(Direct(K1)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectDisplaced(EDX, 1275662060, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(28)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 243, 197, 10, 63, 138, 236, 14, 9, 76, 28], OperandSize::Dword)
}

#[test]
fn vpcmpw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPW, operand1: Some(Direct(K7)), operand2: Some(Direct(XMM21)), operand3: Some(Direct(XMM6)), operand4: Some(Literal8(92)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 243, 213, 4, 63, 254, 92], OperandSize::Qword)
}

#[test]
fn vpcmpw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPW, operand1: Some(Direct(K3)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectDisplaced(RDX, 1635111226, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(22)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 243, 245, 14, 63, 154, 58, 209, 117, 97, 22], OperandSize::Qword)
}

#[test]
fn vpcmpw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPW, operand1: Some(Direct(K5)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(YMM5)), operand4: Some(Literal8(124)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 243, 205, 45, 63, 237, 124], OperandSize::Dword)
}

#[test]
fn vpcmpw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPW, operand1: Some(Direct(K3)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectDisplaced(EBX, 2082653743, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(18)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 243, 253, 47, 63, 155, 47, 198, 34, 124, 18], OperandSize::Dword)
}

#[test]
fn vpcmpw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPW, operand1: Some(Direct(K5)), operand2: Some(Direct(YMM22)), operand3: Some(Direct(YMM18)), operand4: Some(Literal8(125)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 179, 205, 35, 63, 234, 125], OperandSize::Qword)
}

#[test]
fn vpcmpw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPW, operand1: Some(Direct(K4)), operand2: Some(Direct(YMM19)), operand3: Some(IndirectScaledDisplaced(RCX, Two, 1541924590, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(25)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 243, 229, 33, 63, 36, 77, 238, 230, 231, 91, 25], OperandSize::Qword)
}

#[test]
fn vpcmpw_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPW, operand1: Some(Direct(K3)), operand2: Some(Direct(ZMM6)), operand3: Some(Direct(ZMM0)), operand4: Some(Literal8(112)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 243, 205, 77, 63, 216, 112], OperandSize::Dword)
}

#[test]
fn vpcmpw_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPW, operand1: Some(Direct(K7)), operand2: Some(Direct(ZMM1)), operand3: Some(IndirectScaledIndexedDisplaced(ESI, EDX, Eight, 1524404007, Some(OperandSize::Zmmword), None)), operand4: Some(Literal8(80)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 243, 245, 73, 63, 188, 214, 39, 143, 220, 90, 80], OperandSize::Dword)
}

#[test]
fn vpcmpw_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPW, operand1: Some(Direct(K7)), operand2: Some(Direct(ZMM4)), operand3: Some(Direct(ZMM28)), operand4: Some(Literal8(9)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 147, 221, 73, 63, 252, 9], OperandSize::Qword)
}

#[test]
fn vpcmpw_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPW, operand1: Some(Direct(K5)), operand2: Some(Direct(ZMM15)), operand3: Some(IndirectDisplaced(RBX, 772549373, Some(OperandSize::Zmmword), None)), operand4: Some(Literal8(112)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 243, 133, 73, 63, 171, 253, 42, 12, 46, 112], OperandSize::Qword)
}

