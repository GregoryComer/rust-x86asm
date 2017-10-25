use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpcmpq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPQ, operand1: Some(Direct(K1)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM7)), operand4: Some(Literal8(37)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 243, 205, 13, 31, 207, 37], OperandSize::Dword)
}

#[test]
fn vpcmpq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPQ, operand1: Some(Direct(K7)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledDisplaced(EAX, Four, 2130912874, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(113)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 243, 229, 13, 31, 60, 133, 106, 38, 3, 127, 113], OperandSize::Dword)
}

#[test]
fn vpcmpq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPQ, operand1: Some(Direct(K1)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledDisplaced(EAX, Four, 1130818777, Some(OperandSize::Qword), None)), operand4: Some(Literal8(12)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 243, 253, 30, 31, 12, 133, 217, 236, 102, 67, 12], OperandSize::Dword)
}

#[test]
fn vpcmpq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPQ, operand1: Some(Direct(K4)), operand2: Some(Direct(XMM15)), operand3: Some(Direct(XMM29)), operand4: Some(Literal8(26)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 147, 133, 9, 31, 229, 26], OperandSize::Qword)
}

#[test]
fn vpcmpq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPQ, operand1: Some(Direct(K5)), operand2: Some(Direct(XMM8)), operand3: Some(Indirect(RBX, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(92)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 243, 189, 13, 31, 43, 92], OperandSize::Qword)
}

#[test]
fn vpcmpq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPQ, operand1: Some(Direct(K6)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledDisplaced(RAX, Eight, 764941368, Some(OperandSize::Qword), None)), operand4: Some(Literal8(89)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 243, 221, 29, 31, 52, 197, 56, 20, 152, 45, 89], OperandSize::Qword)
}

#[test]
fn vpcmpq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPQ, operand1: Some(Direct(K5)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM0)), operand4: Some(Literal8(104)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 243, 245, 42, 31, 232, 104], OperandSize::Dword)
}

#[test]
fn vpcmpq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPQ, operand1: Some(Direct(K1)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectScaledIndexedDisplaced(EDX, ESI, Four, 415554261, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(47)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 243, 253, 46, 31, 140, 178, 213, 218, 196, 24, 47], OperandSize::Dword)
}

#[test]
fn vpcmpq_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPQ, operand1: Some(Direct(K6)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectScaledIndexed(EAX, ECX, Eight, Some(OperandSize::Qword), None)), operand4: Some(Literal8(54)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 243, 229, 57, 31, 52, 200, 54], OperandSize::Dword)
}

#[test]
fn vpcmpq_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPQ, operand1: Some(Direct(K7)), operand2: Some(Direct(YMM26)), operand3: Some(Direct(YMM3)), operand4: Some(Literal8(0)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 243, 173, 38, 31, 251, 0], OperandSize::Qword)
}

#[test]
fn vpcmpq_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPQ, operand1: Some(Direct(K4)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectDisplaced(RBX, 1631630912, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(63)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 243, 213, 44, 31, 163, 64, 182, 64, 97, 63], OperandSize::Qword)
}

#[test]
fn vpcmpq_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPQ, operand1: Some(Direct(K7)), operand2: Some(Direct(YMM0)), operand3: Some(Indirect(RSI, Some(OperandSize::Qword), None)), operand4: Some(Literal8(86)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 243, 253, 63, 31, 62, 86], OperandSize::Qword)
}

#[test]
fn vpcmpq_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPQ, operand1: Some(Direct(K4)), operand2: Some(Direct(ZMM5)), operand3: Some(Direct(ZMM2)), operand4: Some(Literal8(24)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 243, 213, 77, 31, 226, 24], OperandSize::Dword)
}

#[test]
fn vpcmpq_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPQ, operand1: Some(Direct(K6)), operand2: Some(Direct(ZMM3)), operand3: Some(Indirect(EAX, Some(OperandSize::Zmmword), None)), operand4: Some(Literal8(98)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 243, 229, 74, 31, 48, 98], OperandSize::Dword)
}

#[test]
fn vpcmpq_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPQ, operand1: Some(Direct(K4)), operand2: Some(Direct(ZMM7)), operand3: Some(Indirect(ECX, Some(OperandSize::Qword), None)), operand4: Some(Literal8(12)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 243, 197, 89, 31, 33, 12], OperandSize::Dword)
}

#[test]
fn vpcmpq_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPQ, operand1: Some(Direct(K3)), operand2: Some(Direct(ZMM9)), operand3: Some(Direct(ZMM24)), operand4: Some(Literal8(63)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 147, 181, 78, 31, 216, 63], OperandSize::Qword)
}

#[test]
fn vpcmpq_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPQ, operand1: Some(Direct(K7)), operand2: Some(Direct(ZMM8)), operand3: Some(IndirectScaledIndexed(RAX, RDI, Eight, Some(OperandSize::Zmmword), None)), operand4: Some(Literal8(79)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 243, 189, 76, 31, 60, 248, 79], OperandSize::Qword)
}

#[test]
fn vpcmpq_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPQ, operand1: Some(Direct(K1)), operand2: Some(Direct(ZMM16)), operand3: Some(IndirectDisplaced(RSI, 921684785, Some(OperandSize::Qword), None)), operand4: Some(Literal8(92)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 243, 253, 85, 31, 142, 49, 203, 239, 54, 92], OperandSize::Qword)
}

