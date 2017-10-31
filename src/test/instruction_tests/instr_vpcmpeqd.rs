use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpcmpeqd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 233, 118, 214], OperandSize::Dword)
}

#[test]
fn vpcmpeqd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledDisplaced(EAX, Two, 1344121284, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 118, 60, 69, 196, 169, 29, 80], OperandSize::Dword)
}

#[test]
fn vpcmpeqd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 217, 118, 199], OperandSize::Qword)
}

#[test]
fn vpcmpeqd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledDisplaced(RSI, Two, 1942253188, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 193, 118, 52, 117, 132, 110, 196, 115], OperandSize::Qword)
}

#[test]
fn vpcmpeqd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQD, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(YMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 213, 118, 202], OperandSize::Dword)
}

#[test]
fn vpcmpeqd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQD, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM2)), operand3: Some(Indirect(EDX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 237, 118, 50], OperandSize::Dword)
}

#[test]
fn vpcmpeqd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQD, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(YMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 213, 118, 219], OperandSize::Qword)
}

#[test]
fn vpcmpeqd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQD, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectScaledIndexed(RAX, RAX, Eight, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 229, 118, 28, 192], OperandSize::Qword)
}

#[test]
fn vpcmpeqd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQD, operand1: Some(Direct(K2)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 85, 13, 118, 213], OperandSize::Dword)
}

#[test]
fn vpcmpeqd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQD, operand1: Some(Direct(K5)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectScaledIndexed(EDI, ECX, Four, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 77, 9, 118, 44, 143], OperandSize::Dword)
}

#[test]
fn vpcmpeqd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQD, operand1: Some(Direct(K1)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledIndexedDisplaced(EDI, ESI, Two, 301439017, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 101, 26, 118, 140, 119, 41, 152, 247, 17], OperandSize::Dword)
}

#[test]
fn vpcmpeqd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQD, operand1: Some(Direct(K7)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM11)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 209, 117, 12, 118, 251], OperandSize::Qword)
}

#[test]
fn vpcmpeqd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQD, operand1: Some(Direct(K1)), operand2: Some(Direct(XMM11)), operand3: Some(IndirectScaledIndexed(RDI, RBX, Four, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 37, 15, 118, 12, 159], OperandSize::Qword)
}

#[test]
fn vpcmpeqd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQD, operand1: Some(Direct(K1)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledIndexedDisplaced(RCX, RDI, Two, 875017524, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 93, 25, 118, 140, 121, 52, 181, 39, 52], OperandSize::Qword)
}

#[test]
fn vpcmpeqd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQD, operand1: Some(Direct(K6)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(YMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 101, 43, 118, 241], OperandSize::Dword)
}

#[test]
fn vpcmpeqd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQD, operand1: Some(Direct(K3)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectDisplaced(ESI, 36828969, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 117, 46, 118, 158, 41, 247, 49, 2], OperandSize::Dword)
}

#[test]
fn vpcmpeqd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQD, operand1: Some(Direct(K5)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectScaledIndexed(EBX, ESI, Two, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 69, 62, 118, 44, 115], OperandSize::Dword)
}

#[test]
fn vpcmpeqd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQD, operand1: Some(Direct(K1)), operand2: Some(Direct(YMM17)), operand3: Some(Direct(YMM22)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 177, 117, 35, 118, 206], OperandSize::Qword)
}

#[test]
fn vpcmpeqd_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQD, operand1: Some(Direct(K7)), operand2: Some(Direct(YMM28)), operand3: Some(IndirectScaledDisplaced(RDI, Two, 498264313, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 29, 38, 118, 60, 125, 249, 232, 178, 29], OperandSize::Qword)
}

#[test]
fn vpcmpeqd_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQD, operand1: Some(Direct(K4)), operand2: Some(Direct(YMM4)), operand3: Some(Indirect(RDX, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 93, 62, 118, 34], OperandSize::Qword)
}

#[test]
fn vpcmpeqd_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQD, operand1: Some(Direct(K4)), operand2: Some(Direct(ZMM1)), operand3: Some(Direct(ZMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 117, 73, 118, 231], OperandSize::Dword)
}

#[test]
fn vpcmpeqd_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQD, operand1: Some(Direct(K3)), operand2: Some(Direct(ZMM6)), operand3: Some(IndirectDisplaced(EAX, 1022158213, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 77, 77, 118, 152, 133, 229, 236, 60], OperandSize::Dword)
}

#[test]
fn vpcmpeqd_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQD, operand1: Some(Direct(K4)), operand2: Some(Direct(ZMM4)), operand3: Some(IndirectScaledDisplaced(ESI, Eight, 682584504, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 241, 93, 92, 118, 36, 245, 184, 105, 175, 40], OperandSize::Dword)
}

#[test]
fn vpcmpeqd_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQD, operand1: Some(Direct(K4)), operand2: Some(Direct(ZMM3)), operand3: Some(Direct(ZMM28)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 145, 101, 77, 118, 228], OperandSize::Qword)
}

#[test]
fn vpcmpeqd_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQD, operand1: Some(Direct(K1)), operand2: Some(Direct(ZMM24)), operand3: Some(IndirectScaledIndexed(RBX, RDI, Four, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 61, 70, 118, 12, 187], OperandSize::Qword)
}

#[test]
fn vpcmpeqd_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQD, operand1: Some(Direct(K2)), operand2: Some(Direct(ZMM7)), operand3: Some(IndirectScaledIndexedDisplaced(RAX, RCX, Four, 161416400, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 241, 69, 90, 118, 148, 136, 208, 4, 159, 9], OperandSize::Qword)
}

