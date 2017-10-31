use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpsubq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBQ, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 251, 228], OperandSize::Dword)
}

#[test]
fn vpsubq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBQ, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectScaledIndexed(ECX, EBX, Eight, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 201, 251, 28, 217], OperandSize::Dword)
}

#[test]
fn vpsubq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBQ, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 225, 251, 208], OperandSize::Qword)
}

#[test]
fn vpsubq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBQ, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectDisplaced(RDX, 1607961042, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 201, 251, 146, 210, 137, 215, 95], OperandSize::Qword)
}

#[test]
fn vpsubq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBQ, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 221, 251, 214], OperandSize::Dword)
}

#[test]
fn vpsubq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBQ, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectScaledIndexedDisplaced(ECX, ESI, Four, 365174359, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 205, 251, 148, 177, 87, 30, 196, 21], OperandSize::Dword)
}

#[test]
fn vpsubq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBQ, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 245, 251, 217], OperandSize::Qword)
}

#[test]
fn vpsubq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBQ, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectScaledDisplaced(RBX, Eight, 1528568624, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 229, 251, 60, 221, 48, 27, 28, 91], OperandSize::Qword)
}

#[test]
fn vpsubq_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBQ, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 229, 137, 251, 248], OperandSize::Dword)
}

#[test]
fn vpsubq_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBQ, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectDisplaced(EDI, 1026974533, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 197, 137, 251, 167, 69, 99, 54, 61], OperandSize::Dword)
}

#[test]
fn vpsubq_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBQ, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectDisplaced(ECX, 1334703556, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 241, 221, 157, 251, 153, 196, 245, 141, 79], OperandSize::Dword)
}

#[test]
fn vpsubq_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBQ, operand1: Some(Direct(XMM26)), operand2: Some(Direct(XMM9)), operand3: Some(Direct(XMM25)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 1, 181, 142, 251, 209], OperandSize::Qword)
}

#[test]
fn vpsubq_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBQ, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM22)), operand3: Some(IndirectScaledDisplaced(RSI, Four, 78945266, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 205, 135, 251, 4, 181, 242, 155, 180, 4], OperandSize::Qword)
}

#[test]
fn vpsubq_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBQ, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM28)), operand3: Some(Indirect(RAX, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 241, 157, 151, 251, 40], OperandSize::Qword)
}

#[test]
fn vpsubq_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBQ, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 245, 170, 251, 221], OperandSize::Dword)
}

#[test]
fn vpsubq_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBQ, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM4)), operand3: Some(Indirect(EBX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 221, 172, 251, 43], OperandSize::Dword)
}

#[test]
fn vpsubq_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBQ, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectDisplaced(EDX, 924061216, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 205, 188, 251, 146, 32, 14, 20, 55], OperandSize::Dword)
}

#[test]
fn vpsubq_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBQ, operand1: Some(Direct(YMM27)), operand2: Some(Direct(YMM19)), operand3: Some(Direct(YMM21)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 33, 229, 166, 251, 221], OperandSize::Qword)
}

#[test]
fn vpsubq_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBQ, operand1: Some(Direct(YMM28)), operand2: Some(Direct(YMM11)), operand3: Some(IndirectScaledDisplaced(RAX, Two, 480556233, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 97, 165, 172, 251, 36, 69, 201, 180, 164, 28], OperandSize::Qword)
}

#[test]
fn vpsubq_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBQ, operand1: Some(Direct(YMM20)), operand2: Some(Direct(YMM8)), operand3: Some(Indirect(RAX, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 225, 189, 189, 251, 32], OperandSize::Qword)
}

#[test]
fn vpsubq_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBQ, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM1)), operand3: Some(Direct(ZMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 245, 202, 251, 216], OperandSize::Dword)
}

#[test]
fn vpsubq_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBQ, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM6)), operand3: Some(IndirectScaledDisplaced(EDX, Eight, 1214317056, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 205, 204, 251, 52, 213, 0, 2, 97, 72], OperandSize::Dword)
}

#[test]
fn vpsubq_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBQ, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM7)), operand3: Some(IndirectScaledIndexedDisplaced(EAX, EDX, Eight, 54858628, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 197, 218, 251, 140, 208, 132, 19, 69, 3], OperandSize::Dword)
}

#[test]
fn vpsubq_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBQ, operand1: Some(Direct(ZMM29)), operand2: Some(Direct(ZMM0)), operand3: Some(Direct(ZMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 97, 253, 202, 251, 233], OperandSize::Qword)
}

#[test]
fn vpsubq_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBQ, operand1: Some(Direct(ZMM29)), operand2: Some(Direct(ZMM16)), operand3: Some(IndirectScaledIndexedDisplaced(RAX, RDX, Four, 1015473961, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 97, 253, 195, 251, 172, 144, 41, 231, 134, 60], OperandSize::Qword)
}

#[test]
fn vpsubq_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBQ, operand1: Some(Direct(ZMM30)), operand2: Some(Direct(ZMM6)), operand3: Some(IndirectScaledIndexed(RCX, RSI, Eight, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 97, 205, 220, 251, 52, 241], OperandSize::Qword)
}

