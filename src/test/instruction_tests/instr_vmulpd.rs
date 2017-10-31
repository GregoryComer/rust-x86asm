use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vmulpd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VMULPD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 193, 89, 209], OperandSize::Dword)
}

#[test]
fn vmulpd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VMULPD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectDisplaced(EBX, 972561784, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 241, 89, 147, 120, 29, 248, 57], OperandSize::Dword)
}

#[test]
fn vmulpd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VMULPD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 233, 89, 222], OperandSize::Qword)
}

#[test]
fn vmulpd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VMULPD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM7)), operand3: Some(Indirect(RCX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 193, 89, 9], OperandSize::Qword)
}

#[test]
fn vmulpd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VMULPD, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(YMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 229, 89, 201], OperandSize::Dword)
}

#[test]
fn vmulpd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VMULPD, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectScaledDisplaced(EDI, Four, 1211246169, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 213, 89, 20, 189, 89, 38, 50, 72], OperandSize::Dword)
}

#[test]
fn vmulpd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VMULPD, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(YMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 205, 89, 220], OperandSize::Qword)
}

#[test]
fn vmulpd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VMULPD, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM0)), operand3: Some(Indirect(RAX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 89, 48], OperandSize::Qword)
}

#[test]
fn vmulpd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VMULPD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 245, 142, 89, 192], OperandSize::Dword)
}

#[test]
fn vmulpd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VMULPD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectDisplaced(EBX, 208921882, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 205, 142, 89, 131, 26, 229, 115, 12], OperandSize::Dword)
}

#[test]
fn vmulpd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VMULPD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledDisplaced(EAX, Four, 1795838807, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 241, 253, 154, 89, 28, 133, 87, 83, 10, 107], OperandSize::Dword)
}

#[test]
fn vmulpd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VMULPD, operand1: Some(Direct(XMM25)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM28)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 1, 245, 138, 89, 204], OperandSize::Qword)
}

#[test]
fn vmulpd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VMULPD, operand1: Some(Direct(XMM18)), operand2: Some(Direct(XMM2)), operand3: Some(Indirect(RSI, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 225, 237, 137, 89, 22], OperandSize::Qword)
}

#[test]
fn vmulpd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VMULPD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM15)), operand3: Some(IndirectScaledIndexed(RBX, RBX, Eight, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 241, 133, 157, 89, 12, 219], OperandSize::Qword)
}

#[test]
fn vmulpd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VMULPD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 221, 175, 89, 238], OperandSize::Dword)
}

#[test]
fn vmulpd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VMULPD, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectScaledDisplaced(ESI, Four, 1470125884, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 245, 171, 89, 28, 181, 60, 87, 160, 87], OperandSize::Dword)
}

#[test]
fn vmulpd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VMULPD, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectScaledDisplaced(ECX, Four, 102398944, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 229, 187, 89, 4, 141, 224, 123, 26, 6], OperandSize::Dword)
}

#[test]
fn vmulpd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VMULPD, operand1: Some(Direct(YMM18)), operand2: Some(Direct(YMM11)), operand3: Some(Direct(YMM25)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 129, 165, 169, 89, 209], OperandSize::Qword)
}

#[test]
fn vmulpd_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VMULPD, operand1: Some(Direct(YMM12)), operand2: Some(Direct(YMM11)), operand3: Some(IndirectScaledIndexed(RCX, RSI, Two, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 113, 165, 174, 89, 36, 113], OperandSize::Qword)
}

#[test]
fn vmulpd_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VMULPD, operand1: Some(Direct(YMM11)), operand2: Some(Direct(YMM29)), operand3: Some(IndirectDisplaced(RCX, 1496252907, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 113, 149, 179, 89, 153, 235, 1, 47, 89], OperandSize::Qword)
}

#[test]
fn vmulpd_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VMULPD, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM3)), operand3: Some(Direct(ZMM3)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Nearest), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 229, 154, 89, 235], OperandSize::Dword)
}

#[test]
fn vmulpd_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VMULPD, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM7)), operand3: Some(Indirect(EBX, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 197, 203, 89, 11], OperandSize::Dword)
}

#[test]
fn vmulpd_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VMULPD, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM6)), operand3: Some(IndirectScaledIndexed(EBX, EDI, Four, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 205, 222, 89, 28, 187], OperandSize::Dword)
}

#[test]
fn vmulpd_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VMULPD, operand1: Some(Direct(ZMM30)), operand2: Some(Direct(ZMM22)), operand3: Some(Direct(ZMM12)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Up), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 65, 205, 212, 89, 244], OperandSize::Qword)
}

#[test]
fn vmulpd_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VMULPD, operand1: Some(Direct(ZMM27)), operand2: Some(Direct(ZMM15)), operand3: Some(IndirectScaledIndexed(RBX, RAX, Four, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 97, 133, 201, 89, 28, 131], OperandSize::Qword)
}

#[test]
fn vmulpd_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VMULPD, operand1: Some(Direct(ZMM18)), operand2: Some(Direct(ZMM16)), operand3: Some(IndirectDisplaced(RSI, 307948531, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 225, 253, 214, 89, 150, 243, 235, 90, 18], OperandSize::Qword)
}

