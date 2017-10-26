use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpcmpeqd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 209, 118, 253], OperandSize::Dword)
}

#[test]
fn vpcmpeqd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectScaledDisplaced(EBX, Eight, 1032695014, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 241, 118, 12, 221, 230, 172, 141, 61], OperandSize::Dword)
}

#[test]
fn vpcmpeqd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 209, 118, 202], OperandSize::Qword)
}

#[test]
fn vpcmpeqd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledIndexed(RAX, RDX, Four, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 233, 118, 12, 144], OperandSize::Qword)
}

#[test]
fn vpcmpeqd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQD, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 197, 118, 195], OperandSize::Dword)
}

#[test]
fn vpcmpeqd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQD, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM4)), operand3: Some(Indirect(ESI, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 221, 118, 54], OperandSize::Dword)
}

#[test]
fn vpcmpeqd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQD, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 221, 118, 212], OperandSize::Qword)
}

#[test]
fn vpcmpeqd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQD, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM1)), operand3: Some(Indirect(RAX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 245, 118, 0], OperandSize::Qword)
}

#[test]
fn vpcmpeqd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQD, operand1: Some(Direct(K2)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 85, 9, 118, 208], OperandSize::Dword)
}

#[test]
fn vpcmpeqd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQD, operand1: Some(Direct(K5)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectDisplaced(EDI, 121816838, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 93, 14, 118, 175, 6, 199, 66, 7], OperandSize::Dword)
}

#[test]
fn vpcmpeqd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQD, operand1: Some(Direct(K3)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledIndexedDisplaced(ECX, EDI, Eight, 626853437, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 109, 29, 118, 156, 249, 61, 6, 93, 37], OperandSize::Dword)
}

#[test]
fn vpcmpeqd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQD, operand1: Some(Direct(K3)), operand2: Some(Direct(XMM22)), operand3: Some(Direct(XMM27)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 145, 77, 2, 118, 219], OperandSize::Qword)
}

#[test]
fn vpcmpeqd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQD, operand1: Some(Direct(K1)), operand2: Some(Direct(XMM14)), operand3: Some(IndirectScaledDisplaced(RAX, Two, 930459343, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 13, 11, 118, 12, 69, 207, 174, 117, 55], OperandSize::Qword)
}

#[test]
fn vpcmpeqd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQD, operand1: Some(Direct(K3)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectScaledIndexedDisplaced(RAX, RBX, Two, 846700476, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 77, 30, 118, 156, 88, 188, 159, 119, 50], OperandSize::Qword)
}

#[test]
fn vpcmpeqd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQD, operand1: Some(Direct(K6)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 109, 43, 118, 241], OperandSize::Dword)
}

#[test]
fn vpcmpeqd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQD, operand1: Some(Direct(K1)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectScaledDisplaced(ECX, Four, 1809529358, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 125, 42, 118, 12, 141, 14, 58, 219, 107], OperandSize::Dword)
}

#[test]
fn vpcmpeqd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQD, operand1: Some(Direct(K7)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectScaledIndexed(EDX, EDI, Four, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 125, 62, 118, 60, 186], OperandSize::Dword)
}

#[test]
fn vpcmpeqd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQD, operand1: Some(Direct(K7)), operand2: Some(Direct(YMM27)), operand3: Some(Direct(YMM8)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 209, 37, 34, 118, 248], OperandSize::Qword)
}

#[test]
fn vpcmpeqd_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQD, operand1: Some(Direct(K7)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectDisplaced(RBX, 491839299, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 85, 46, 118, 187, 67, 223, 80, 29], OperandSize::Qword)
}

#[test]
fn vpcmpeqd_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQD, operand1: Some(Direct(K4)), operand2: Some(Direct(YMM27)), operand3: Some(IndirectScaledIndexed(RDX, RCX, Four, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 37, 50, 118, 36, 138], OperandSize::Qword)
}

#[test]
fn vpcmpeqd_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQD, operand1: Some(Direct(K2)), operand2: Some(Direct(ZMM3)), operand3: Some(Direct(ZMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 101, 73, 118, 210], OperandSize::Dword)
}

#[test]
fn vpcmpeqd_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQD, operand1: Some(Direct(K7)), operand2: Some(Direct(ZMM7)), operand3: Some(IndirectScaledIndexed(EBX, EDI, Eight, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 69, 79, 118, 60, 251], OperandSize::Dword)
}

#[test]
fn vpcmpeqd_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQD, operand1: Some(Direct(K4)), operand2: Some(Direct(ZMM5)), operand3: Some(IndirectScaledIndexedDisplaced(EBX, ESI, Two, 1042867209, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 241, 85, 89, 118, 164, 115, 9, 228, 40, 62], OperandSize::Dword)
}

#[test]
fn vpcmpeqd_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQD, operand1: Some(Direct(K4)), operand2: Some(Direct(ZMM22)), operand3: Some(Direct(ZMM27)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 145, 77, 70, 118, 227], OperandSize::Qword)
}

#[test]
fn vpcmpeqd_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQD, operand1: Some(Direct(K5)), operand2: Some(Direct(ZMM24)), operand3: Some(IndirectScaledIndexedDisplaced(RSI, RCX, Two, 2100403848, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 61, 67, 118, 172, 78, 136, 158, 49, 125], OperandSize::Qword)
}

#[test]
fn vpcmpeqd_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQD, operand1: Some(Direct(K2)), operand2: Some(Direct(ZMM27)), operand3: Some(IndirectDisplaced(RAX, 1182337218, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 241, 37, 84, 118, 144, 194, 8, 121, 70], OperandSize::Qword)
}

