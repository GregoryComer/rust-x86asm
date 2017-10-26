use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpcmpq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPQ, operand1: Some(Direct(K5)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM7)), operand4: Some(Literal8(13)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 243, 213, 11, 31, 239, 13], OperandSize::Dword)
}

#[test]
fn vpcmpq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPQ, operand1: Some(Direct(K6)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectScaledIndexedDisplaced(EDI, EBX, Two, 631917192, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(79)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 243, 245, 9, 31, 180, 95, 136, 74, 170, 37, 79], OperandSize::Dword)
}

#[test]
fn vpcmpq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPQ, operand1: Some(Direct(K3)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectScaledDisplaced(EDI, Two, 179654866, Some(OperandSize::Qword), None)), operand4: Some(Literal8(47)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 243, 245, 30, 31, 28, 125, 210, 80, 181, 10, 47], OperandSize::Dword)
}

#[test]
fn vpcmpq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPQ, operand1: Some(Direct(K2)), operand2: Some(Direct(XMM25)), operand3: Some(Direct(XMM16)), operand4: Some(Literal8(56)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 179, 181, 3, 31, 208, 56], OperandSize::Qword)
}

#[test]
fn vpcmpq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPQ, operand1: Some(Direct(K1)), operand2: Some(Direct(XMM11)), operand3: Some(IndirectScaledIndexedDisplaced(RAX, RBX, Eight, 42513257, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(121)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 243, 165, 15, 31, 140, 216, 105, 179, 136, 2, 121], OperandSize::Qword)
}

#[test]
fn vpcmpq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPQ, operand1: Some(Direct(K3)), operand2: Some(Direct(XMM28)), operand3: Some(IndirectScaledIndexed(RSI, RCX, Eight, Some(OperandSize::Qword), None)), operand4: Some(Literal8(32)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 243, 157, 20, 31, 28, 206, 32], OperandSize::Qword)
}

#[test]
fn vpcmpq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPQ, operand1: Some(Direct(K7)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(YMM3)), operand4: Some(Literal8(81)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 243, 205, 46, 31, 251, 81], OperandSize::Dword)
}

#[test]
fn vpcmpq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPQ, operand1: Some(Direct(K3)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectDisplaced(EDI, 1300456548, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(28)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 243, 237, 41, 31, 159, 100, 100, 131, 77, 28], OperandSize::Dword)
}

#[test]
fn vpcmpq_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPQ, operand1: Some(Direct(K5)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectDisplaced(EAX, 2021124051, Some(OperandSize::Qword), None)), operand4: Some(Literal8(76)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 243, 229, 57, 31, 168, 211, 231, 119, 120, 76], OperandSize::Dword)
}

#[test]
fn vpcmpq_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPQ, operand1: Some(Direct(K2)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM19)), operand4: Some(Literal8(13)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 179, 221, 43, 31, 211, 13], OperandSize::Qword)
}

#[test]
fn vpcmpq_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPQ, operand1: Some(Direct(K7)), operand2: Some(Direct(YMM20)), operand3: Some(Indirect(RAX, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(25)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 243, 221, 37, 31, 56, 25], OperandSize::Qword)
}

#[test]
fn vpcmpq_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPQ, operand1: Some(Direct(K6)), operand2: Some(Direct(YMM18)), operand3: Some(IndirectScaledDisplaced(RSI, Two, 20377058, Some(OperandSize::Qword), None)), operand4: Some(Literal8(0)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 243, 237, 49, 31, 52, 117, 226, 237, 54, 1, 0], OperandSize::Qword)
}

#[test]
fn vpcmpq_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPQ, operand1: Some(Direct(K3)), operand2: Some(Direct(ZMM2)), operand3: Some(Direct(ZMM2)), operand4: Some(Literal8(85)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 243, 237, 75, 31, 218, 85], OperandSize::Dword)
}

#[test]
fn vpcmpq_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPQ, operand1: Some(Direct(K4)), operand2: Some(Direct(ZMM3)), operand3: Some(IndirectScaledIndexedDisplaced(EBX, EDI, Four, 843466509, Some(OperandSize::Zmmword), None)), operand4: Some(Literal8(92)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 243, 229, 77, 31, 164, 187, 13, 71, 70, 50, 92], OperandSize::Dword)
}

#[test]
fn vpcmpq_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPQ, operand1: Some(Direct(K2)), operand2: Some(Direct(ZMM4)), operand3: Some(IndirectScaledIndexed(ECX, EAX, Four, Some(OperandSize::Qword), None)), operand4: Some(Literal8(83)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 243, 221, 94, 31, 20, 129, 83], OperandSize::Dword)
}

#[test]
fn vpcmpq_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPQ, operand1: Some(Direct(K2)), operand2: Some(Direct(ZMM9)), operand3: Some(Direct(ZMM22)), operand4: Some(Literal8(51)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 179, 181, 79, 31, 214, 51], OperandSize::Qword)
}

#[test]
fn vpcmpq_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPQ, operand1: Some(Direct(K1)), operand2: Some(Direct(ZMM26)), operand3: Some(IndirectScaledDisplaced(RDX, Eight, 2054785308, Some(OperandSize::Zmmword), None)), operand4: Some(Literal8(93)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 243, 173, 68, 31, 12, 213, 28, 137, 121, 122, 93], OperandSize::Qword)
}

#[test]
fn vpcmpq_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPQ, operand1: Some(Direct(K7)), operand2: Some(Direct(ZMM8)), operand3: Some(IndirectScaledDisplaced(RBX, Eight, 76876890, Some(OperandSize::Qword), None)), operand4: Some(Literal8(126)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 243, 189, 94, 31, 60, 221, 90, 12, 149, 4, 126], OperandSize::Qword)
}

