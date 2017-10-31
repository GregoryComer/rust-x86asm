use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpcmpd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPD, operand1: Some(Direct(K4)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM7)), operand4: Some(Literal8(64)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 243, 93, 13, 31, 231, 64], OperandSize::Dword)
}

#[test]
fn vpcmpd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPD, operand1: Some(Direct(K3)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectDisplaced(ESI, 1618511561, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(110)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 243, 101, 9, 31, 158, 201, 134, 120, 96, 110], OperandSize::Dword)
}

#[test]
fn vpcmpd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPD, operand1: Some(Direct(K4)), operand2: Some(Direct(XMM6)), operand3: Some(Indirect(ECX, Some(OperandSize::Dword), None)), operand4: Some(Literal8(77)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 243, 77, 28, 31, 33, 77], OperandSize::Dword)
}

#[test]
fn vpcmpd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPD, operand1: Some(Direct(K1)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM11)), operand4: Some(Literal8(93)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 211, 125, 10, 31, 203, 93], OperandSize::Qword)
}

#[test]
fn vpcmpd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPD, operand1: Some(Direct(K6)), operand2: Some(Direct(XMM30)), operand3: Some(IndirectScaledIndexed(RCX, RSI, Eight, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(93)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 243, 13, 1, 31, 52, 241, 93], OperandSize::Qword)
}

#[test]
fn vpcmpd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPD, operand1: Some(Direct(K6)), operand2: Some(Direct(XMM10)), operand3: Some(IndirectScaledIndexed(RSI, RSI, Four, Some(OperandSize::Dword), None)), operand4: Some(Literal8(27)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 243, 45, 27, 31, 52, 182, 27], OperandSize::Qword)
}

#[test]
fn vpcmpd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPD, operand1: Some(Direct(K7)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM2)), operand4: Some(Literal8(69)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 243, 117, 47, 31, 250, 69], OperandSize::Dword)
}

#[test]
fn vpcmpd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPD, operand1: Some(Direct(K1)), operand2: Some(Direct(YMM1)), operand3: Some(Indirect(EBX, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(9)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 243, 117, 41, 31, 11, 9], OperandSize::Dword)
}

#[test]
fn vpcmpd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPD, operand1: Some(Direct(K4)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectScaledDisplaced(EBX, Two, 832123892, Some(OperandSize::Dword), None)), operand4: Some(Literal8(51)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 243, 93, 58, 31, 36, 93, 244, 51, 153, 49, 51], OperandSize::Dword)
}

#[test]
fn vpcmpd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPD, operand1: Some(Direct(K2)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM15)), operand4: Some(Literal8(74)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 211, 93, 41, 31, 215, 74], OperandSize::Qword)
}

#[test]
fn vpcmpd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPD, operand1: Some(Direct(K7)), operand2: Some(Direct(YMM16)), operand3: Some(IndirectScaledDisplaced(RDX, Four, 688472293, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(96)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 243, 125, 37, 31, 60, 149, 229, 64, 9, 41, 96], OperandSize::Qword)
}

#[test]
fn vpcmpd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPD, operand1: Some(Direct(K6)), operand2: Some(Direct(YMM9)), operand3: Some(Indirect(RDX, Some(OperandSize::Dword), None)), operand4: Some(Literal8(14)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 243, 53, 61, 31, 50, 14], OperandSize::Qword)
}

#[test]
fn vpcmpd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPD, operand1: Some(Direct(K3)), operand2: Some(Direct(ZMM5)), operand3: Some(Direct(ZMM1)), operand4: Some(Literal8(125)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 243, 85, 77, 31, 217, 125], OperandSize::Dword)
}

#[test]
fn vpcmpd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPD, operand1: Some(Direct(K7)), operand2: Some(Direct(ZMM7)), operand3: Some(IndirectScaledIndexed(ESI, EDX, Four, Some(OperandSize::Zmmword), None)), operand4: Some(Literal8(73)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 243, 69, 74, 31, 60, 150, 73], OperandSize::Dword)
}

#[test]
fn vpcmpd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPD, operand1: Some(Direct(K7)), operand2: Some(Direct(ZMM4)), operand3: Some(IndirectDisplaced(EDI, 1765367909, Some(OperandSize::Dword), None)), operand4: Some(Literal8(110)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 243, 93, 94, 31, 191, 101, 96, 57, 105, 110], OperandSize::Dword)
}

#[test]
fn vpcmpd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPD, operand1: Some(Direct(K6)), operand2: Some(Direct(ZMM27)), operand3: Some(Direct(ZMM7)), operand4: Some(Literal8(122)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 243, 37, 65, 31, 247, 122], OperandSize::Qword)
}

#[test]
fn vpcmpd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPD, operand1: Some(Direct(K7)), operand2: Some(Direct(ZMM25)), operand3: Some(Indirect(RBX, Some(OperandSize::Zmmword), None)), operand4: Some(Literal8(88)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 243, 53, 65, 31, 59, 88], OperandSize::Qword)
}

#[test]
fn vpcmpd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPD, operand1: Some(Direct(K1)), operand2: Some(Direct(ZMM14)), operand3: Some(IndirectScaledIndexed(RSI, RCX, Two, Some(OperandSize::Dword), None)), operand4: Some(Literal8(114)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 243, 13, 93, 31, 12, 78, 114], OperandSize::Qword)
}

