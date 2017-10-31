use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpcmpuw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPUW, operand1: Some(Direct(K3)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM1)), operand4: Some(Literal8(53)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 243, 197, 15, 62, 217, 53], OperandSize::Dword)
}

#[test]
fn vpcmpuw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPUW, operand1: Some(Direct(K1)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledDisplaced(EDX, Eight, 2144732023, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(108)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 243, 213, 12, 62, 12, 213, 119, 3, 214, 127, 108], OperandSize::Dword)
}

#[test]
fn vpcmpuw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPUW, operand1: Some(Direct(K4)), operand2: Some(Direct(XMM10)), operand3: Some(Direct(XMM23)), operand4: Some(Literal8(92)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 179, 173, 15, 62, 231, 92], OperandSize::Qword)
}

#[test]
fn vpcmpuw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPUW, operand1: Some(Direct(K6)), operand2: Some(Direct(XMM24)), operand3: Some(IndirectScaledIndexed(RSI, RSI, Four, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(25)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 243, 189, 1, 62, 52, 182, 25], OperandSize::Qword)
}

#[test]
fn vpcmpuw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPUW, operand1: Some(Direct(K2)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM6)), operand4: Some(Literal8(79)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 243, 245, 42, 62, 214, 79], OperandSize::Dword)
}

#[test]
fn vpcmpuw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPUW, operand1: Some(Direct(K1)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectDisplaced(EAX, 2036232406, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(113)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 243, 205, 46, 62, 136, 214, 112, 94, 121, 113], OperandSize::Dword)
}

#[test]
fn vpcmpuw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPUW, operand1: Some(Direct(K1)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM29)), operand4: Some(Literal8(93)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 147, 237, 47, 62, 205, 93], OperandSize::Qword)
}

#[test]
fn vpcmpuw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPUW, operand1: Some(Direct(K1)), operand2: Some(Direct(YMM16)), operand3: Some(IndirectScaledDisplaced(RCX, Eight, 1518565530, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(111)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 243, 253, 33, 62, 12, 205, 154, 120, 131, 90, 111], OperandSize::Qword)
}

#[test]
fn vpcmpuw_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPUW, operand1: Some(Direct(K5)), operand2: Some(Direct(ZMM0)), operand3: Some(Direct(ZMM6)), operand4: Some(Literal8(62)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 243, 253, 78, 62, 238, 62], OperandSize::Dword)
}

#[test]
fn vpcmpuw_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPUW, operand1: Some(Direct(K6)), operand2: Some(Direct(ZMM7)), operand3: Some(IndirectScaledDisplaced(ESI, Eight, 1681212595, Some(OperandSize::Zmmword), None)), operand4: Some(Literal8(102)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 243, 197, 79, 62, 52, 245, 179, 68, 53, 100, 102], OperandSize::Dword)
}

#[test]
fn vpcmpuw_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPUW, operand1: Some(Direct(K2)), operand2: Some(Direct(ZMM12)), operand3: Some(Direct(ZMM15)), operand4: Some(Literal8(113)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 211, 157, 74, 62, 215, 113], OperandSize::Qword)
}

#[test]
fn vpcmpuw_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPUW, operand1: Some(Direct(K1)), operand2: Some(Direct(ZMM11)), operand3: Some(IndirectScaledDisplaced(RDX, Four, 1742436219, Some(OperandSize::Zmmword), None)), operand4: Some(Literal8(81)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 243, 165, 74, 62, 12, 149, 123, 119, 219, 103, 81], OperandSize::Qword)
}

