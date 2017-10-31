use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vscalefpd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VSCALEFPD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 253, 138, 44, 254], OperandSize::Dword)
}

#[test]
fn vscalefpd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VSCALEFPD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledIndexed(EDX, EDI, Two, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 221, 138, 44, 36, 122], OperandSize::Dword)
}

#[test]
fn vscalefpd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VSCALEFPD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectScaledIndexed(EDX, ESI, Four, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 242, 205, 156, 44, 12, 178], OperandSize::Dword)
}

#[test]
fn vscalefpd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VSCALEFPD, operand1: Some(Direct(XMM19)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 226, 245, 139, 44, 219], OperandSize::Qword)
}

#[test]
fn vscalefpd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VSCALEFPD, operand1: Some(Direct(XMM24)), operand2: Some(Direct(XMM21)), operand3: Some(IndirectScaledIndexedDisplaced(RAX, RCX, Four, 559462701, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 98, 213, 129, 44, 132, 136, 45, 185, 88, 33], OperandSize::Qword)
}

#[test]
fn vscalefpd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VSCALEFPD, operand1: Some(Direct(XMM14)), operand2: Some(Direct(XMM1)), operand3: Some(Indirect(RDI, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 114, 245, 158, 44, 55], OperandSize::Qword)
}

#[test]
fn vscalefpd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VSCALEFPD, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 245, 169, 44, 193], OperandSize::Dword)
}

#[test]
fn vscalefpd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VSCALEFPD, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectScaledIndexed(EDX, ECX, Eight, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 205, 171, 44, 12, 202], OperandSize::Dword)
}

#[test]
fn vscalefpd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VSCALEFPD, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectScaledIndexedDisplaced(EBX, EDX, Four, 1096984013, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 205, 188, 44, 180, 147, 205, 165, 98, 65], OperandSize::Dword)
}

#[test]
fn vscalefpd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VSCALEFPD, operand1: Some(Direct(YMM11)), operand2: Some(Direct(YMM24)), operand3: Some(Direct(YMM22)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 50, 189, 165, 44, 222], OperandSize::Qword)
}

#[test]
fn vscalefpd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VSCALEFPD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM28)), operand3: Some(IndirectDisplaced(RSI, 1105160386, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 157, 163, 44, 174, 194, 104, 223, 65], OperandSize::Qword)
}

#[test]
fn vscalefpd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VSCALEFPD, operand1: Some(Direct(YMM10)), operand2: Some(Direct(YMM8)), operand3: Some(IndirectDisplaced(RAX, 276228178, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 114, 189, 189, 44, 144, 82, 232, 118, 16], OperandSize::Qword)
}

#[test]
fn vscalefpd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VSCALEFPD, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM6)), operand3: Some(Direct(ZMM3)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Zero), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 205, 255, 44, 219], OperandSize::Dword)
}

#[test]
fn vscalefpd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VSCALEFPD, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM3)), operand3: Some(IndirectScaledIndexed(EDI, EBX, Eight, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 229, 201, 44, 36, 223], OperandSize::Dword)
}

#[test]
fn vscalefpd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VSCALEFPD, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM6)), operand3: Some(IndirectScaledIndexed(ECX, ECX, Eight, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 205, 219, 44, 52, 201], OperandSize::Dword)
}

#[test]
fn vscalefpd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VSCALEFPD, operand1: Some(Direct(ZMM9)), operand2: Some(Direct(ZMM0)), operand3: Some(Direct(ZMM14)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Down), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 82, 253, 185, 44, 206], OperandSize::Qword)
}

#[test]
fn vscalefpd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VSCALEFPD, operand1: Some(Direct(ZMM20)), operand2: Some(Direct(ZMM5)), operand3: Some(IndirectScaledIndexed(RDX, RDX, Two, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 226, 213, 206, 44, 36, 82], OperandSize::Qword)
}

#[test]
fn vscalefpd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VSCALEFPD, operand1: Some(Direct(ZMM11)), operand2: Some(Direct(ZMM8)), operand3: Some(IndirectScaledDisplaced(RAX, Two, 2093487895, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 114, 189, 222, 44, 28, 69, 23, 23, 200, 124], OperandSize::Qword)
}

