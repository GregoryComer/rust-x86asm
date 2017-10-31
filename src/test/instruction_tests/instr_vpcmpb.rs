use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpcmpb_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPB, operand1: Some(Direct(K7)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM5)), operand4: Some(Literal8(74)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 243, 109, 9, 63, 253, 74], OperandSize::Dword)
}

#[test]
fn vpcmpb_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPB, operand1: Some(Direct(K6)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectDisplaced(EAX, 170086557, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(3)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 243, 125, 15, 63, 176, 157, 80, 35, 10, 3], OperandSize::Dword)
}

#[test]
fn vpcmpb_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPB, operand1: Some(Direct(K3)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM15)), operand4: Some(Literal8(46)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 211, 117, 9, 63, 223, 46], OperandSize::Qword)
}

#[test]
fn vpcmpb_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPB, operand1: Some(Direct(K4)), operand2: Some(Direct(XMM26)), operand3: Some(IndirectDisplaced(RDI, 1131720231, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(33)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 243, 45, 2, 63, 167, 39, 174, 116, 67, 33], OperandSize::Qword)
}

#[test]
fn vpcmpb_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPB, operand1: Some(Direct(K5)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM2)), operand4: Some(Literal8(8)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 243, 69, 47, 63, 234, 8], OperandSize::Dword)
}

#[test]
fn vpcmpb_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPB, operand1: Some(Direct(K3)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectDisplaced(EAX, 694848683, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(34)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 243, 69, 44, 63, 152, 171, 140, 106, 41, 34], OperandSize::Dword)
}

#[test]
fn vpcmpb_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPB, operand1: Some(Direct(K5)), operand2: Some(Direct(YMM17)), operand3: Some(Direct(YMM20)), operand4: Some(Literal8(97)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 179, 117, 38, 63, 236, 97], OperandSize::Qword)
}

#[test]
fn vpcmpb_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPB, operand1: Some(Direct(K2)), operand2: Some(Direct(YMM28)), operand3: Some(Indirect(RDX, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(111)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 243, 29, 38, 63, 18, 111], OperandSize::Qword)
}

#[test]
fn vpcmpb_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPB, operand1: Some(Direct(K1)), operand2: Some(Direct(ZMM5)), operand3: Some(Direct(ZMM7)), operand4: Some(Literal8(25)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 243, 85, 76, 63, 207, 25], OperandSize::Dword)
}

#[test]
fn vpcmpb_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPB, operand1: Some(Direct(K5)), operand2: Some(Direct(ZMM6)), operand3: Some(IndirectScaledDisplaced(ESI, Eight, 1379821709, Some(OperandSize::Zmmword), None)), operand4: Some(Literal8(107)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 243, 77, 78, 63, 44, 245, 141, 104, 62, 82, 107], OperandSize::Dword)
}

#[test]
fn vpcmpb_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPB, operand1: Some(Direct(K4)), operand2: Some(Direct(ZMM0)), operand3: Some(Direct(ZMM2)), operand4: Some(Literal8(114)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 243, 125, 78, 63, 226, 114], OperandSize::Qword)
}

#[test]
fn vpcmpb_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPB, operand1: Some(Direct(K2)), operand2: Some(Direct(ZMM11)), operand3: Some(Indirect(RDI, Some(OperandSize::Zmmword), None)), operand4: Some(Literal8(12)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 243, 37, 78, 63, 23, 12], OperandSize::Qword)
}

