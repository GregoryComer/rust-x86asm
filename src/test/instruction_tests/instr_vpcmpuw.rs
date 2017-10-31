use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpcmpuw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPUW, operand1: Some(Direct(K1)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM2)), operand4: Some(Literal8(45)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 243, 197, 10, 62, 202, 45], OperandSize::Dword)
}

#[test]
fn vpcmpuw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPUW, operand1: Some(Direct(K3)), operand2: Some(Direct(XMM0)), operand3: Some(Indirect(ECX, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(19)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 243, 253, 14, 62, 25, 19], OperandSize::Dword)
}

#[test]
fn vpcmpuw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPUW, operand1: Some(Direct(K2)), operand2: Some(Direct(XMM14)), operand3: Some(Direct(XMM3)), operand4: Some(Literal8(85)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 243, 141, 13, 62, 211, 85], OperandSize::Qword)
}

#[test]
fn vpcmpuw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPUW, operand1: Some(Direct(K1)), operand2: Some(Direct(XMM15)), operand3: Some(IndirectScaledIndexed(RCX, RDX, Eight, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(58)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 243, 133, 9, 62, 12, 209, 58], OperandSize::Qword)
}

#[test]
fn vpcmpuw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPUW, operand1: Some(Direct(K3)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM6)), operand4: Some(Literal8(51)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 243, 197, 45, 62, 222, 51], OperandSize::Dword)
}

#[test]
fn vpcmpuw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPUW, operand1: Some(Direct(K2)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectScaledIndexed(ECX, ECX, Two, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(80)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 243, 237, 44, 62, 20, 73, 80], OperandSize::Dword)
}

#[test]
fn vpcmpuw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPUW, operand1: Some(Direct(K4)), operand2: Some(Direct(YMM9)), operand3: Some(Direct(YMM25)), operand4: Some(Literal8(92)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 147, 181, 41, 62, 225, 92], OperandSize::Qword)
}

#[test]
fn vpcmpuw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPUW, operand1: Some(Direct(K6)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectScaledDisplaced(RSI, Four, 1157743223, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(124)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 243, 205, 47, 62, 52, 181, 119, 194, 1, 69, 124], OperandSize::Qword)
}

#[test]
fn vpcmpuw_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPUW, operand1: Some(Direct(K2)), operand2: Some(Direct(ZMM1)), operand3: Some(Direct(ZMM1)), operand4: Some(Literal8(37)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 243, 245, 79, 62, 209, 37], OperandSize::Dword)
}

#[test]
fn vpcmpuw_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPUW, operand1: Some(Direct(K3)), operand2: Some(Direct(ZMM3)), operand3: Some(IndirectScaledDisplaced(EDX, Two, 367689104, Some(OperandSize::Zmmword), None)), operand4: Some(Literal8(51)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 243, 229, 76, 62, 28, 85, 144, 125, 234, 21, 51], OperandSize::Dword)
}

#[test]
fn vpcmpuw_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPUW, operand1: Some(Direct(K7)), operand2: Some(Direct(ZMM31)), operand3: Some(Direct(ZMM3)), operand4: Some(Literal8(117)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 243, 133, 68, 62, 251, 117], OperandSize::Qword)
}

#[test]
fn vpcmpuw_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPUW, operand1: Some(Direct(K2)), operand2: Some(Direct(ZMM10)), operand3: Some(IndirectScaledIndexed(RAX, RDI, Four, Some(OperandSize::Zmmword), None)), operand4: Some(Literal8(105)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 243, 173, 76, 62, 20, 184, 105], OperandSize::Qword)
}

