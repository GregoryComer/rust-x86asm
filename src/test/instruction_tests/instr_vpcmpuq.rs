use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpcmpuq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPUQ, operand1: Some(Direct(K2)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM5)), operand4: Some(Literal8(104)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 243, 245, 9, 30, 213, 104], OperandSize::Dword)
}

#[test]
fn vpcmpuq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPUQ, operand1: Some(Direct(K7)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledIndexed(EBX, ESI, Two, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(92)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 243, 213, 9, 30, 60, 115, 92], OperandSize::Dword)
}

#[test]
fn vpcmpuq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPUQ, operand1: Some(Direct(K1)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectDisplaced(ECX, 919255185, Some(OperandSize::Qword), None)), operand4: Some(Literal8(108)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 243, 213, 26, 30, 137, 145, 184, 202, 54, 108], OperandSize::Dword)
}

#[test]
fn vpcmpuq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPUQ, operand1: Some(Direct(K3)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM27)), operand4: Some(Literal8(22)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 147, 245, 10, 30, 219, 22], OperandSize::Qword)
}

#[test]
fn vpcmpuq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPUQ, operand1: Some(Direct(K7)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectDisplaced(RDX, 1982563855, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(16)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 243, 237, 13, 30, 186, 15, 134, 43, 118, 16], OperandSize::Qword)
}

#[test]
fn vpcmpuq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPUQ, operand1: Some(Direct(K1)), operand2: Some(Direct(XMM14)), operand3: Some(IndirectScaledIndexed(RDX, RCX, Four, Some(OperandSize::Qword), None)), operand4: Some(Literal8(39)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 243, 141, 26, 30, 12, 138, 39], OperandSize::Qword)
}

#[test]
fn vpcmpuq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPUQ, operand1: Some(Direct(K2)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM0)), operand4: Some(Literal8(111)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 243, 221, 44, 30, 208, 111], OperandSize::Dword)
}

#[test]
fn vpcmpuq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPUQ, operand1: Some(Direct(K1)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectScaledIndexed(ECX, EAX, Eight, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(122)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 243, 213, 45, 30, 12, 193, 122], OperandSize::Dword)
}

#[test]
fn vpcmpuq_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPUQ, operand1: Some(Direct(K5)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectDisplaced(EBX, 900925226, Some(OperandSize::Qword), None)), operand4: Some(Literal8(47)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 243, 197, 60, 30, 171, 42, 7, 179, 53, 47], OperandSize::Dword)
}

#[test]
fn vpcmpuq_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPUQ, operand1: Some(Direct(K3)), operand2: Some(Direct(YMM31)), operand3: Some(Direct(YMM19)), operand4: Some(Literal8(51)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 179, 133, 37, 30, 219, 51], OperandSize::Qword)
}

#[test]
fn vpcmpuq_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPUQ, operand1: Some(Direct(K6)), operand2: Some(Direct(YMM1)), operand3: Some(Indirect(RSI, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(81)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 243, 245, 44, 30, 54, 81], OperandSize::Qword)
}

#[test]
fn vpcmpuq_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPUQ, operand1: Some(Direct(K4)), operand2: Some(Direct(YMM3)), operand3: Some(Indirect(RAX, Some(OperandSize::Qword), None)), operand4: Some(Literal8(75)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 243, 229, 62, 30, 32, 75], OperandSize::Qword)
}

#[test]
fn vpcmpuq_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPUQ, operand1: Some(Direct(K6)), operand2: Some(Direct(ZMM5)), operand3: Some(Direct(ZMM2)), operand4: Some(Literal8(116)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 243, 213, 76, 30, 242, 116], OperandSize::Dword)
}

#[test]
fn vpcmpuq_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPUQ, operand1: Some(Direct(K1)), operand2: Some(Direct(ZMM3)), operand3: Some(IndirectScaledIndexed(EAX, ECX, Two, Some(OperandSize::Zmmword), None)), operand4: Some(Literal8(111)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 243, 229, 74, 30, 12, 72, 111], OperandSize::Dword)
}

#[test]
fn vpcmpuq_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPUQ, operand1: Some(Direct(K2)), operand2: Some(Direct(ZMM5)), operand3: Some(IndirectDisplaced(ECX, 628860479, Some(OperandSize::Qword), None)), operand4: Some(Literal8(111)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 243, 213, 93, 30, 145, 63, 166, 123, 37, 111], OperandSize::Dword)
}

#[test]
fn vpcmpuq_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPUQ, operand1: Some(Direct(K6)), operand2: Some(Direct(ZMM20)), operand3: Some(Direct(ZMM6)), operand4: Some(Literal8(126)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 243, 221, 65, 30, 246, 126], OperandSize::Qword)
}

#[test]
fn vpcmpuq_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPUQ, operand1: Some(Direct(K2)), operand2: Some(Direct(ZMM4)), operand3: Some(Indirect(RDX, Some(OperandSize::Zmmword), None)), operand4: Some(Literal8(95)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 243, 221, 79, 30, 18, 95], OperandSize::Qword)
}

#[test]
fn vpcmpuq_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPUQ, operand1: Some(Direct(K3)), operand2: Some(Direct(ZMM7)), operand3: Some(IndirectScaledIndexed(RSI, RBX, Eight, Some(OperandSize::Qword), None)), operand4: Some(Literal8(91)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 243, 197, 90, 30, 28, 222, 91], OperandSize::Qword)
}

