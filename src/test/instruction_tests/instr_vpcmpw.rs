use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpcmpw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPW, operand1: Some(Direct(K3)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM7)), operand4: Some(Literal8(56)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 243, 229, 14, 63, 223, 56], OperandSize::Dword)
}

#[test]
fn vpcmpw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPW, operand1: Some(Direct(K1)), operand2: Some(Direct(XMM1)), operand3: Some(Indirect(EDI, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(72)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 243, 245, 11, 63, 15, 72], OperandSize::Dword)
}

#[test]
fn vpcmpw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPW, operand1: Some(Direct(K1)), operand2: Some(Direct(XMM8)), operand3: Some(Direct(XMM14)), operand4: Some(Literal8(75)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 211, 189, 13, 63, 206, 75], OperandSize::Qword)
}

#[test]
fn vpcmpw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPW, operand1: Some(Direct(K6)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledIndexed(RAX, RAX, Two, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(54)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 243, 229, 9, 63, 52, 64, 54], OperandSize::Qword)
}

#[test]
fn vpcmpw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPW, operand1: Some(Direct(K6)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM1)), operand4: Some(Literal8(95)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 243, 245, 43, 63, 241, 95], OperandSize::Dword)
}

#[test]
fn vpcmpw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPW, operand1: Some(Direct(K1)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectScaledDisplaced(EDI, Eight, 1128081587, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(49)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 243, 245, 45, 63, 12, 253, 179, 40, 61, 67, 49], OperandSize::Dword)
}

#[test]
fn vpcmpw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPW, operand1: Some(Direct(K7)), operand2: Some(Direct(YMM21)), operand3: Some(Direct(YMM7)), operand4: Some(Literal8(50)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 243, 213, 37, 63, 255, 50], OperandSize::Qword)
}

#[test]
fn vpcmpw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPW, operand1: Some(Direct(K2)), operand2: Some(Direct(YMM31)), operand3: Some(IndirectScaledDisplaced(RAX, Eight, 1555861334, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(2)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 243, 133, 37, 63, 20, 197, 86, 143, 188, 92, 2], OperandSize::Qword)
}

#[test]
fn vpcmpw_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPW, operand1: Some(Direct(K2)), operand2: Some(Direct(ZMM6)), operand3: Some(Direct(ZMM0)), operand4: Some(Literal8(113)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 243, 205, 77, 63, 208, 113], OperandSize::Dword)
}

#[test]
fn vpcmpw_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPW, operand1: Some(Direct(K5)), operand2: Some(Direct(ZMM1)), operand3: Some(IndirectDisplaced(EAX, 1386339452, Some(OperandSize::Zmmword), None)), operand4: Some(Literal8(82)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 243, 245, 78, 63, 168, 124, 220, 161, 82, 82], OperandSize::Dword)
}

#[test]
fn vpcmpw_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPW, operand1: Some(Direct(K2)), operand2: Some(Direct(ZMM27)), operand3: Some(Direct(ZMM25)), operand4: Some(Literal8(60)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 147, 165, 67, 63, 209, 60], OperandSize::Qword)
}

#[test]
fn vpcmpw_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPW, operand1: Some(Direct(K5)), operand2: Some(Direct(ZMM29)), operand3: Some(IndirectScaledIndexedDisplaced(RSI, RBX, Two, 302001894, Some(OperandSize::Zmmword), None)), operand4: Some(Literal8(62)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 243, 149, 65, 63, 172, 94, 230, 46, 0, 18, 62], OperandSize::Qword)
}

