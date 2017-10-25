use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpcmpub_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPUB, operand1: Some(Direct(K2)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM0)), operand4: Some(Literal8(88)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 243, 109, 15, 62, 208, 88], OperandSize::Dword)
}

#[test]
fn vpcmpub_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPUB, operand1: Some(Direct(K4)), operand2: Some(Direct(XMM4)), operand3: Some(Indirect(EDX, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(14)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 243, 93, 14, 62, 34, 14], OperandSize::Dword)
}

#[test]
fn vpcmpub_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPUB, operand1: Some(Direct(K3)), operand2: Some(Direct(XMM13)), operand3: Some(Direct(XMM3)), operand4: Some(Literal8(41)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 243, 21, 15, 62, 219, 41], OperandSize::Qword)
}

#[test]
fn vpcmpub_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPUB, operand1: Some(Direct(K3)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledDisplaced(RSI, Eight, 915959030, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(15)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 243, 93, 14, 62, 28, 245, 246, 108, 152, 54, 15], OperandSize::Qword)
}

#[test]
fn vpcmpub_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPUB, operand1: Some(Direct(K5)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(YMM5)), operand4: Some(Literal8(72)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 243, 77, 44, 62, 237, 72], OperandSize::Dword)
}

#[test]
fn vpcmpub_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPUB, operand1: Some(Direct(K3)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectScaledDisplaced(EBX, Two, 1603824903, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(73)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 243, 117, 41, 62, 28, 93, 7, 109, 152, 95, 73], OperandSize::Dword)
}

#[test]
fn vpcmpub_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPUB, operand1: Some(Direct(K6)), operand2: Some(Direct(YMM10)), operand3: Some(Direct(YMM12)), operand4: Some(Literal8(103)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 211, 45, 46, 62, 244, 103], OperandSize::Qword)
}

#[test]
fn vpcmpub_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPUB, operand1: Some(Direct(K7)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectDisplaced(RCX, 145323906, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(118)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 243, 109, 41, 62, 185, 130, 119, 169, 8, 118], OperandSize::Qword)
}

#[test]
fn vpcmpub_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPUB, operand1: Some(Direct(K5)), operand2: Some(Direct(ZMM7)), operand3: Some(Direct(ZMM4)), operand4: Some(Literal8(28)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 243, 69, 78, 62, 236, 28], OperandSize::Dword)
}

#[test]
fn vpcmpub_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPUB, operand1: Some(Direct(K5)), operand2: Some(Direct(ZMM5)), operand3: Some(IndirectScaledIndexed(EDI, EAX, Four, Some(OperandSize::Zmmword), None)), operand4: Some(Literal8(77)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 243, 85, 77, 62, 44, 135, 77], OperandSize::Dword)
}

#[test]
fn vpcmpub_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPUB, operand1: Some(Direct(K6)), operand2: Some(Direct(ZMM30)), operand3: Some(Direct(ZMM6)), operand4: Some(Literal8(113)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 243, 13, 65, 62, 246, 113], OperandSize::Qword)
}

#[test]
fn vpcmpub_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPUB, operand1: Some(Direct(K4)), operand2: Some(Direct(ZMM5)), operand3: Some(IndirectScaledIndexed(RAX, RAX, Eight, Some(OperandSize::Zmmword), None)), operand4: Some(Literal8(33)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 243, 85, 75, 62, 36, 192, 33], OperandSize::Qword)
}

