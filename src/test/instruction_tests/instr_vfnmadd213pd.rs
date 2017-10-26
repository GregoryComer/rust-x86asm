use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vfnmadd213pd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD213PD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 217, 172, 253], OperandSize::Dword)
}

#[test]
fn vfnmadd213pd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD213PD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectDisplaced(EDI, 1562885920, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 201, 172, 143, 32, 191, 39, 93], OperandSize::Dword)
}

#[test]
fn vfnmadd213pd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD213PD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 241, 172, 239], OperandSize::Qword)
}

#[test]
fn vfnmadd213pd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD213PD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM2)), operand3: Some(Indirect(RBX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 233, 172, 43], OperandSize::Qword)
}

#[test]
fn vfnmadd213pd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD213PD, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 237, 172, 197], OperandSize::Dword)
}

#[test]
fn vfnmadd213pd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD213PD, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectScaledIndexed(EDX, EDX, Two, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 229, 172, 28, 82], OperandSize::Dword)
}

#[test]
fn vfnmadd213pd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD213PD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(YMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 205, 172, 253], OperandSize::Qword)
}

#[test]
fn vfnmadd213pd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD213PD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM1)), operand3: Some(Indirect(RAX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 245, 172, 40], OperandSize::Qword)
}

#[test]
fn vfnmadd213pd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD213PD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 213, 140, 172, 211], OperandSize::Dword)
}

#[test]
fn vfnmadd213pd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD213PD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledIndexed(EBX, ECX, Eight, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 253, 139, 172, 12, 203], OperandSize::Dword)
}

#[test]
fn vfnmadd213pd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD213PD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectScaledDisplaced(ECX, Four, 1901459736, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 242, 245, 153, 172, 52, 141, 24, 249, 85, 113], OperandSize::Dword)
}

#[test]
fn vfnmadd213pd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD213PD, operand1: Some(Direct(XMM11)), operand2: Some(Direct(XMM14)), operand3: Some(Direct(XMM15)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 82, 141, 142, 172, 223], OperandSize::Qword)
}

#[test]
fn vfnmadd213pd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD213PD, operand1: Some(Direct(XMM8)), operand2: Some(Direct(XMM21)), operand3: Some(Indirect(RSI, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 114, 213, 134, 172, 6], OperandSize::Qword)
}

#[test]
fn vfnmadd213pd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD213PD, operand1: Some(Direct(XMM27)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectDisplaced(RDX, 397703438, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 98, 229, 155, 172, 154, 14, 121, 180, 23], OperandSize::Qword)
}

#[test]
fn vfnmadd213pd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD213PD, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 197, 173, 172, 211], OperandSize::Dword)
}

#[test]
fn vfnmadd213pd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD213PD, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectScaledIndexed(EAX, EDI, Two, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 197, 169, 172, 28, 120], OperandSize::Dword)
}

#[test]
fn vfnmadd213pd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD213PD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectDisplaced(EBX, 1615443643, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 237, 185, 172, 187, 187, 182, 73, 96], OperandSize::Dword)
}

#[test]
fn vfnmadd213pd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD213PD, operand1: Some(Direct(YMM13)), operand2: Some(Direct(YMM16)), operand3: Some(Direct(YMM18)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 50, 253, 165, 172, 234], OperandSize::Qword)
}

#[test]
fn vfnmadd213pd_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD213PD, operand1: Some(Direct(YMM9)), operand2: Some(Direct(YMM27)), operand3: Some(IndirectScaledIndexedDisplaced(RAX, RAX, Two, 1933172903, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 114, 165, 162, 172, 140, 64, 167, 224, 57, 115], OperandSize::Qword)
}

#[test]
fn vfnmadd213pd_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD213PD, operand1: Some(Direct(YMM25)), operand2: Some(Direct(YMM20)), operand3: Some(IndirectDisplaced(RDI, 1785451382, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 98, 221, 182, 172, 143, 118, 211, 107, 106], OperandSize::Qword)
}

#[test]
fn vfnmadd213pd_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD213PD, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM1)), operand3: Some(Direct(ZMM3)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Nearest), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 245, 154, 172, 227], OperandSize::Dword)
}

#[test]
fn vfnmadd213pd_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD213PD, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM5)), operand3: Some(IndirectScaledIndexedDisplaced(EDX, EBX, Two, 680010789, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 213, 202, 172, 140, 90, 37, 36, 136, 40], OperandSize::Dword)
}

#[test]
fn vfnmadd213pd_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD213PD, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM6)), operand3: Some(IndirectScaledIndexed(EBX, EBX, Eight, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 205, 218, 172, 60, 219], OperandSize::Dword)
}

#[test]
fn vfnmadd213pd_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD213PD, operand1: Some(Direct(ZMM9)), operand2: Some(Direct(ZMM20)), operand3: Some(Direct(ZMM21)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Nearest), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 50, 221, 145, 172, 205], OperandSize::Qword)
}

#[test]
fn vfnmadd213pd_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD213PD, operand1: Some(Direct(ZMM12)), operand2: Some(Direct(ZMM4)), operand3: Some(IndirectScaledIndexedDisplaced(RDX, RDI, Four, 1311163284, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 114, 221, 206, 172, 164, 186, 148, 195, 38, 78], OperandSize::Qword)
}

#[test]
fn vfnmadd213pd_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD213PD, operand1: Some(Direct(ZMM18)), operand2: Some(Direct(ZMM31)), operand3: Some(IndirectScaledIndexed(RCX, RDX, Eight, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 226, 133, 214, 172, 20, 209], OperandSize::Qword)
}

