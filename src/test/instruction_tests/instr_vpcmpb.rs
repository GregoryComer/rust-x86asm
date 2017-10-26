use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpcmpb_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPB, operand1: Some(Direct(K7)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM0)), operand4: Some(Literal8(8)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 243, 117, 10, 63, 248, 8], OperandSize::Dword)
}

#[test]
fn vpcmpb_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPB, operand1: Some(Direct(K7)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectDisplaced(EDI, 1069998557, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(25)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 243, 117, 15, 63, 191, 221, 225, 198, 63, 25], OperandSize::Dword)
}

#[test]
fn vpcmpb_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPB, operand1: Some(Direct(K6)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM11)), operand4: Some(Literal8(109)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 211, 69, 15, 63, 243, 109], OperandSize::Qword)
}

#[test]
fn vpcmpb_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPB, operand1: Some(Direct(K5)), operand2: Some(Direct(XMM21)), operand3: Some(Indirect(RAX, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(79)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 243, 85, 6, 63, 40, 79], OperandSize::Qword)
}

#[test]
fn vpcmpb_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPB, operand1: Some(Direct(K5)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM7)), operand4: Some(Literal8(18)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 243, 109, 43, 63, 239, 18], OperandSize::Dword)
}

#[test]
fn vpcmpb_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPB, operand1: Some(Direct(K4)), operand2: Some(Direct(YMM5)), operand3: Some(Indirect(ESI, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(38)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 243, 85, 46, 63, 38, 38], OperandSize::Dword)
}

#[test]
fn vpcmpb_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPB, operand1: Some(Direct(K2)), operand2: Some(Direct(YMM13)), operand3: Some(Direct(YMM13)), operand4: Some(Literal8(124)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 211, 21, 46, 63, 213, 124], OperandSize::Qword)
}

#[test]
fn vpcmpb_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPB, operand1: Some(Direct(K7)), operand2: Some(Direct(YMM10)), operand3: Some(IndirectScaledDisplaced(RSI, Four, 1784457093, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(30)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 243, 45, 42, 63, 60, 181, 133, 167, 92, 106, 30], OperandSize::Qword)
}

#[test]
fn vpcmpb_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPB, operand1: Some(Direct(K2)), operand2: Some(Direct(ZMM6)), operand3: Some(Direct(ZMM7)), operand4: Some(Literal8(48)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 243, 77, 74, 63, 215, 48], OperandSize::Dword)
}

#[test]
fn vpcmpb_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPB, operand1: Some(Direct(K2)), operand2: Some(Direct(ZMM3)), operand3: Some(Indirect(ECX, Some(OperandSize::Zmmword), None)), operand4: Some(Literal8(27)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 243, 101, 77, 63, 17, 27], OperandSize::Dword)
}

#[test]
fn vpcmpb_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPB, operand1: Some(Direct(K2)), operand2: Some(Direct(ZMM24)), operand3: Some(Direct(ZMM11)), operand4: Some(Literal8(113)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 211, 61, 66, 63, 211, 113], OperandSize::Qword)
}

#[test]
fn vpcmpb_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPB, operand1: Some(Direct(K5)), operand2: Some(Direct(ZMM10)), operand3: Some(IndirectScaledIndexedDisplaced(RDI, RDI, Eight, 453512439, Some(OperandSize::Zmmword), None)), operand4: Some(Literal8(62)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 243, 45, 73, 63, 172, 255, 247, 12, 8, 27, 62], OperandSize::Qword)
}

