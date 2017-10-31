use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vgetmantps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETMANTPS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM7)), operand3: Some(Literal8(104)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 243, 125, 139, 38, 255, 104], OperandSize::Dword)
}

#[test]
fn vgetmantps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETMANTPS, operand1: Some(Direct(XMM7)), operand2: Some(IndirectScaledIndexedDisplaced(EDX, EDI, Two, 2111434056, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(101)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 243, 125, 137, 38, 188, 122, 72, 237, 217, 125, 101], OperandSize::Dword)
}

#[test]
fn vgetmantps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETMANTPS, operand1: Some(Direct(XMM7)), operand2: Some(IndirectDisplaced(ECX, 206631987, Some(OperandSize::Dword), None)), operand3: Some(Literal8(56)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 243, 125, 156, 38, 185, 51, 244, 80, 12, 56], OperandSize::Dword)
}

#[test]
fn vgetmantps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETMANTPS, operand1: Some(Direct(XMM8)), operand2: Some(Direct(XMM9)), operand3: Some(Literal8(76)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 83, 125, 137, 38, 193, 76], OperandSize::Qword)
}

#[test]
fn vgetmantps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETMANTPS, operand1: Some(Direct(XMM12)), operand2: Some(IndirectScaledDisplaced(RCX, Eight, 1827765750, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(118)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 115, 125, 137, 38, 36, 205, 246, 125, 241, 108, 118], OperandSize::Qword)
}

#[test]
fn vgetmantps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETMANTPS, operand1: Some(Direct(XMM28)), operand2: Some(IndirectScaledDisplaced(RAX, Eight, 341258818, Some(OperandSize::Dword), None)), operand3: Some(Literal8(35)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 99, 125, 158, 38, 36, 197, 66, 50, 87, 20, 35], OperandSize::Qword)
}

#[test]
fn vgetmantps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETMANTPS, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM4)), operand3: Some(Literal8(92)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 243, 125, 174, 38, 228, 92], OperandSize::Dword)
}

#[test]
fn vgetmantps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETMANTPS, operand1: Some(Direct(YMM5)), operand2: Some(IndirectScaledIndexedDisplaced(EBX, EDI, Two, 2143008093, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(85)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 243, 125, 169, 38, 172, 123, 93, 181, 187, 127, 85], OperandSize::Dword)
}

#[test]
fn vgetmantps_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETMANTPS, operand1: Some(Direct(YMM1)), operand2: Some(Indirect(EDI, Some(OperandSize::Dword), None)), operand3: Some(Literal8(76)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 243, 125, 187, 38, 15, 76], OperandSize::Dword)
}

#[test]
fn vgetmantps_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETMANTPS, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM9)), operand3: Some(Literal8(48)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 211, 125, 170, 38, 217, 48], OperandSize::Qword)
}

#[test]
fn vgetmantps_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETMANTPS, operand1: Some(Direct(YMM4)), operand2: Some(IndirectScaledDisplaced(RCX, Two, 1470838992, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(69)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 243, 125, 171, 38, 36, 77, 208, 56, 171, 87, 69], OperandSize::Qword)
}

#[test]
fn vgetmantps_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETMANTPS, operand1: Some(Direct(YMM28)), operand2: Some(IndirectScaledIndexedDisplaced(RDI, RDX, Two, 1396229808, Some(OperandSize::Dword), None)), operand3: Some(Literal8(23)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 99, 125, 189, 38, 164, 87, 176, 198, 56, 83, 23], OperandSize::Qword)
}

#[test]
fn vgetmantps_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETMANTPS, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM4)), operand3: Some(Literal8(103)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K1), broadcast: None }, &[98, 243, 125, 153, 38, 220, 103], OperandSize::Dword)
}

#[test]
fn vgetmantps_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETMANTPS, operand1: Some(Direct(ZMM5)), operand2: Some(IndirectDisplaced(ESI, 1738676563, Some(OperandSize::Zmmword), None)), operand3: Some(Literal8(103)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 243, 125, 207, 38, 174, 83, 25, 162, 103, 103], OperandSize::Dword)
}

#[test]
fn vgetmantps_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETMANTPS, operand1: Some(Direct(ZMM5)), operand2: Some(IndirectDisplaced(ECX, 489557596, Some(OperandSize::Dword), None)), operand3: Some(Literal8(66)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 243, 125, 219, 38, 169, 92, 14, 46, 29, 66], OperandSize::Dword)
}

#[test]
fn vgetmantps_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETMANTPS, operand1: Some(Direct(ZMM12)), operand2: Some(Direct(ZMM14)), operand3: Some(Literal8(20)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K1), broadcast: None }, &[98, 83, 125, 153, 38, 230, 20], OperandSize::Qword)
}

#[test]
fn vgetmantps_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETMANTPS, operand1: Some(Direct(ZMM29)), operand2: Some(Indirect(RCX, Some(OperandSize::Zmmword), None)), operand3: Some(Literal8(95)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 99, 125, 206, 38, 41, 95], OperandSize::Qword)
}

#[test]
fn vgetmantps_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETMANTPS, operand1: Some(Direct(ZMM14)), operand2: Some(IndirectDisplaced(RBX, 1392937755, Some(OperandSize::Dword), None)), operand3: Some(Literal8(71)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 115, 125, 220, 38, 179, 27, 139, 6, 83, 71], OperandSize::Qword)
}

