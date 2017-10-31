use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpcmpeqd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 241, 118, 234], OperandSize::Dword)
}

#[test]
fn vpcmpeqd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectDisplaced(EAX, 824587292, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 201, 118, 144, 28, 52, 38, 49], OperandSize::Dword)
}

#[test]
fn vpcmpeqd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 225, 118, 241], OperandSize::Qword)
}

#[test]
fn vpcmpeqd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledIndexedDisplaced(RDI, RBX, Two, 921772666, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 118, 140, 95, 122, 34, 241, 54], OperandSize::Qword)
}

#[test]
fn vpcmpeqd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQD, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(YMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 213, 118, 210], OperandSize::Dword)
}

#[test]
fn vpcmpeqd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQD, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectDisplaced(EDX, 763852241, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 229, 118, 178, 209, 117, 135, 45], OperandSize::Dword)
}

#[test]
fn vpcmpeqd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQD, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 197, 118, 244], OperandSize::Qword)
}

#[test]
fn vpcmpeqd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQD, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectScaledIndexedDisplaced(RDX, RAX, Eight, 942063091, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 245, 118, 180, 194, 243, 189, 38, 56], OperandSize::Qword)
}

#[test]
fn vpcmpeqd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQD, operand1: Some(Direct(K3)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 117, 13, 118, 223], OperandSize::Dword)
}

#[test]
fn vpcmpeqd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQD, operand1: Some(Direct(K4)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectScaledIndexed(EDX, EBX, Two, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 117, 11, 118, 36, 90], OperandSize::Dword)
}

#[test]
fn vpcmpeqd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQD, operand1: Some(Direct(K2)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledDisplaced(EAX, Four, 765467062, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 109, 27, 118, 20, 133, 182, 25, 160, 45], OperandSize::Dword)
}

#[test]
fn vpcmpeqd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQD, operand1: Some(Direct(K6)), operand2: Some(Direct(XMM15)), operand3: Some(Direct(XMM30)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 145, 5, 14, 118, 246], OperandSize::Qword)
}

#[test]
fn vpcmpeqd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQD, operand1: Some(Direct(K7)), operand2: Some(Direct(XMM4)), operand3: Some(Indirect(RCX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 93, 15, 118, 57], OperandSize::Qword)
}

#[test]
fn vpcmpeqd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQD, operand1: Some(Direct(K5)), operand2: Some(Direct(XMM8)), operand3: Some(IndirectScaledDisplaced(RAX, Two, 505779958, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 61, 28, 118, 44, 69, 246, 150, 37, 30], OperandSize::Qword)
}

#[test]
fn vpcmpeqd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQD, operand1: Some(Direct(K7)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 109, 45, 118, 248], OperandSize::Dword)
}

#[test]
fn vpcmpeqd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQD, operand1: Some(Direct(K3)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectScaledDisplaced(EDX, Eight, 722834984, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 117, 44, 118, 28, 213, 40, 150, 21, 43], OperandSize::Dword)
}

#[test]
fn vpcmpeqd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQD, operand1: Some(Direct(K6)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectScaledIndexedDisplaced(EBX, EDI, Two, 254975183, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 93, 59, 118, 180, 123, 207, 156, 50, 15], OperandSize::Dword)
}

#[test]
fn vpcmpeqd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQD, operand1: Some(Direct(K3)), operand2: Some(Direct(YMM19)), operand3: Some(Direct(YMM24)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 145, 101, 36, 118, 216], OperandSize::Qword)
}

#[test]
fn vpcmpeqd_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQD, operand1: Some(Direct(K2)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectScaledIndexed(RDI, RBX, Two, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 69, 43, 118, 20, 95], OperandSize::Qword)
}

#[test]
fn vpcmpeqd_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQD, operand1: Some(Direct(K4)), operand2: Some(Direct(YMM19)), operand3: Some(IndirectScaledIndexed(RCX, RSI, Two, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 101, 49, 118, 36, 113], OperandSize::Qword)
}

#[test]
fn vpcmpeqd_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQD, operand1: Some(Direct(K6)), operand2: Some(Direct(ZMM1)), operand3: Some(Direct(ZMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 117, 77, 118, 243], OperandSize::Dword)
}

#[test]
fn vpcmpeqd_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQD, operand1: Some(Direct(K4)), operand2: Some(Direct(ZMM2)), operand3: Some(Indirect(EDI, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 109, 75, 118, 39], OperandSize::Dword)
}

#[test]
fn vpcmpeqd_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQD, operand1: Some(Direct(K5)), operand2: Some(Direct(ZMM3)), operand3: Some(Indirect(EDX, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 241, 101, 95, 118, 42], OperandSize::Dword)
}

#[test]
fn vpcmpeqd_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQD, operand1: Some(Direct(K5)), operand2: Some(Direct(ZMM14)), operand3: Some(Direct(ZMM11)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 209, 13, 75, 118, 235], OperandSize::Qword)
}

#[test]
fn vpcmpeqd_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQD, operand1: Some(Direct(K6)), operand2: Some(Direct(ZMM18)), operand3: Some(IndirectScaledIndexedDisplaced(RBX, RDI, Two, 275074823, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 109, 66, 118, 180, 123, 7, 79, 101, 16], OperandSize::Qword)
}

#[test]
fn vpcmpeqd_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQD, operand1: Some(Direct(K5)), operand2: Some(Direct(ZMM22)), operand3: Some(IndirectDisplaced(RDI, 1563669316, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 241, 77, 84, 118, 175, 68, 179, 51, 93], OperandSize::Qword)
}

