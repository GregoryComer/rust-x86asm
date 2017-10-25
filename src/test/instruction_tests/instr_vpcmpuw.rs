use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpcmpuw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPUW, operand1: Some(Direct(K6)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM1)), operand4: Some(Literal8(36)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 243, 213, 15, 62, 241, 36], OperandSize::Dword)
}

#[test]
fn vpcmpuw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPUW, operand1: Some(Direct(K2)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledIndexed(EDI, EDX, Four, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(93)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 243, 237, 10, 62, 20, 151, 93], OperandSize::Dword)
}

#[test]
fn vpcmpuw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPUW, operand1: Some(Direct(K2)), operand2: Some(Direct(XMM30)), operand3: Some(Direct(XMM8)), operand4: Some(Literal8(120)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 211, 141, 3, 62, 208, 120], OperandSize::Qword)
}

#[test]
fn vpcmpuw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPUW, operand1: Some(Direct(K1)), operand2: Some(Direct(XMM15)), operand3: Some(IndirectScaledIndexed(RSI, RBX, Two, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(72)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 243, 133, 9, 62, 12, 94, 72], OperandSize::Qword)
}

#[test]
fn vpcmpuw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPUW, operand1: Some(Direct(K2)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM2)), operand4: Some(Literal8(49)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 243, 197, 42, 62, 210, 49], OperandSize::Dword)
}

#[test]
fn vpcmpuw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPUW, operand1: Some(Direct(K6)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectScaledIndexedDisplaced(EBX, EDI, Eight, 1692792299, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(105)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 243, 197, 45, 62, 180, 251, 235, 245, 229, 100, 105], OperandSize::Dword)
}

#[test]
fn vpcmpuw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPUW, operand1: Some(Direct(K7)), operand2: Some(Direct(YMM18)), operand3: Some(Direct(YMM6)), operand4: Some(Literal8(97)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 243, 237, 39, 62, 254, 97], OperandSize::Qword)
}

#[test]
fn vpcmpuw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPUW, operand1: Some(Direct(K3)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectScaledDisplaced(RCX, Two, 420961133, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(90)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 243, 253, 45, 62, 28, 77, 109, 91, 23, 25, 90], OperandSize::Qword)
}

#[test]
fn vpcmpuw_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPUW, operand1: Some(Direct(K3)), operand2: Some(Direct(ZMM0)), operand3: Some(Direct(ZMM5)), operand4: Some(Literal8(126)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 243, 253, 78, 62, 221, 126], OperandSize::Dword)
}

#[test]
fn vpcmpuw_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPUW, operand1: Some(Direct(K3)), operand2: Some(Direct(ZMM1)), operand3: Some(IndirectScaledIndexed(ECX, ECX, Four, Some(OperandSize::Zmmword), None)), operand4: Some(Literal8(73)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 243, 245, 77, 62, 28, 137, 73], OperandSize::Dword)
}

#[test]
fn vpcmpuw_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPUW, operand1: Some(Direct(K6)), operand2: Some(Direct(ZMM13)), operand3: Some(Direct(ZMM7)), operand4: Some(Literal8(89)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 243, 149, 76, 62, 247, 89], OperandSize::Qword)
}

#[test]
fn vpcmpuw_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPUW, operand1: Some(Direct(K5)), operand2: Some(Direct(ZMM31)), operand3: Some(IndirectScaledIndexedDisplaced(RDX, RCX, Eight, 972702342, Some(OperandSize::Zmmword), None)), operand4: Some(Literal8(85)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 243, 133, 65, 62, 172, 202, 134, 66, 250, 57, 85], OperandSize::Qword)
}

