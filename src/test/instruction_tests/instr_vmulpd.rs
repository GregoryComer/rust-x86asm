use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vmulpd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VMULPD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 89, 208], OperandSize::Dword)
}

#[test]
fn vmulpd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VMULPD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledIndexed(ESI, EAX, Two, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 89, 28, 70], OperandSize::Dword)
}

#[test]
fn vmulpd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VMULPD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 201, 89, 237], OperandSize::Qword)
}

#[test]
fn vmulpd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VMULPD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM7)), operand3: Some(Indirect(RSI, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 193, 89, 54], OperandSize::Qword)
}

#[test]
fn vmulpd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VMULPD, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 245, 89, 241], OperandSize::Dword)
}

#[test]
fn vmulpd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VMULPD, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectScaledDisplaced(EDI, Eight, 1554570745, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 205, 89, 52, 253, 249, 221, 168, 92], OperandSize::Dword)
}

#[test]
fn vmulpd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VMULPD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(YMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 205, 89, 234], OperandSize::Qword)
}

#[test]
fn vmulpd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VMULPD, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectScaledDisplaced(RAX, Four, 819530785, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 89, 28, 133, 33, 12, 217, 48], OperandSize::Qword)
}

#[test]
fn vmulpd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VMULPD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 221, 138, 89, 252], OperandSize::Dword)
}

#[test]
fn vmulpd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VMULPD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM5)), operand3: Some(Indirect(ESI, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 213, 140, 89, 54], OperandSize::Dword)
}

#[test]
fn vmulpd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VMULPD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledIndexedDisplaced(EDX, ESI, Two, 1276622504, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 241, 221, 155, 89, 180, 114, 168, 182, 23, 76], OperandSize::Dword)
}

#[test]
fn vmulpd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VMULPD, operand1: Some(Direct(XMM31)), operand2: Some(Direct(XMM30)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 97, 141, 129, 89, 249], OperandSize::Qword)
}

#[test]
fn vmulpd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VMULPD, operand1: Some(Direct(XMM26)), operand2: Some(Direct(XMM3)), operand3: Some(Indirect(RSI, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 97, 229, 142, 89, 22], OperandSize::Qword)
}

#[test]
fn vmulpd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VMULPD, operand1: Some(Direct(XMM11)), operand2: Some(Direct(XMM14)), operand3: Some(IndirectScaledIndexedDisplaced(RCX, RBX, Eight, 901134197, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 113, 141, 159, 89, 156, 217, 117, 55, 182, 53], OperandSize::Qword)
}

#[test]
fn vmulpd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VMULPD, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(YMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 205, 170, 89, 218], OperandSize::Dword)
}

#[test]
fn vmulpd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VMULPD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectScaledDisplaced(EDX, Four, 1078439825, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 205, 175, 89, 44, 149, 145, 175, 71, 64], OperandSize::Dword)
}

#[test]
fn vmulpd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VMULPD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM0)), operand3: Some(Indirect(ECX, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 253, 188, 89, 41], OperandSize::Dword)
}

#[test]
fn vmulpd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VMULPD, operand1: Some(Direct(YMM22)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(YMM8)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 193, 213, 171, 89, 240], OperandSize::Qword)
}

#[test]
fn vmulpd_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VMULPD, operand1: Some(Direct(YMM10)), operand2: Some(Direct(YMM1)), operand3: Some(Indirect(RCX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 113, 245, 174, 89, 17], OperandSize::Qword)
}

#[test]
fn vmulpd_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VMULPD, operand1: Some(Direct(YMM8)), operand2: Some(Direct(YMM29)), operand3: Some(Indirect(RBX, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 113, 149, 183, 89, 3], OperandSize::Qword)
}

#[test]
fn vmulpd_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VMULPD, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM1)), operand3: Some(Direct(ZMM2)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Up), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 245, 221, 89, 250], OperandSize::Dword)
}

#[test]
fn vmulpd_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VMULPD, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM7)), operand3: Some(IndirectScaledIndexed(EDX, ESI, Four, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 197, 202, 89, 28, 178], OperandSize::Dword)
}

#[test]
fn vmulpd_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VMULPD, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM5)), operand3: Some(IndirectScaledIndexedDisplaced(EDI, EAX, Eight, 216975911, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 213, 222, 89, 188, 199, 39, 202, 238, 12], OperandSize::Dword)
}

#[test]
fn vmulpd_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VMULPD, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM25)), operand3: Some(Direct(ZMM15)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Down), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 209, 181, 183, 89, 255], OperandSize::Qword)
}

#[test]
fn vmulpd_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VMULPD, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM8)), operand3: Some(Indirect(RDI, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 189, 202, 89, 31], OperandSize::Qword)
}

#[test]
fn vmulpd_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VMULPD, operand1: Some(Direct(ZMM28)), operand2: Some(Direct(ZMM6)), operand3: Some(IndirectScaledIndexedDisplaced(RSI, RDI, Eight, 912869494, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 97, 205, 217, 89, 164, 254, 118, 72, 105, 54], OperandSize::Qword)
}

