use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vscalefpd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VSCALEFPD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 205, 139, 44, 230], OperandSize::Dword)
}

#[test]
fn vscalefpd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VSCALEFPD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectScaledDisplaced(EDX, Two, 585238374, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 245, 138, 44, 36, 85, 102, 7, 226, 34], OperandSize::Dword)
}

#[test]
fn vscalefpd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VSCALEFPD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledIndexed(EDX, ESI, Four, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 242, 221, 154, 44, 12, 178], OperandSize::Dword)
}

#[test]
fn vscalefpd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VSCALEFPD, operand1: Some(Direct(XMM10)), operand2: Some(Direct(XMM30)), operand3: Some(Direct(XMM18)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 50, 141, 130, 44, 210], OperandSize::Qword)
}

#[test]
fn vscalefpd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VSCALEFPD, operand1: Some(Direct(XMM20)), operand2: Some(Direct(XMM19)), operand3: Some(IndirectScaledIndexed(RDI, RDX, Two, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 226, 229, 131, 44, 36, 87], OperandSize::Qword)
}

#[test]
fn vscalefpd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VSCALEFPD, operand1: Some(Direct(XMM20)), operand2: Some(Direct(XMM27)), operand3: Some(IndirectScaledIndexed(RBX, RCX, Eight, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 226, 165, 148, 44, 36, 203], OperandSize::Qword)
}

#[test]
fn vscalefpd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VSCALEFPD, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(YMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 205, 174, 44, 198], OperandSize::Dword)
}

#[test]
fn vscalefpd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VSCALEFPD, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectScaledDisplaced(EDI, Two, 1935326908, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 229, 172, 44, 28, 125, 188, 190, 90, 115], OperandSize::Dword)
}

#[test]
fn vscalefpd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VSCALEFPD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectDisplaced(ECX, 1776925666, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 253, 185, 44, 185, 226, 187, 233, 105], OperandSize::Dword)
}

#[test]
fn vscalefpd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VSCALEFPD, operand1: Some(Direct(YMM16)), operand2: Some(Direct(YMM9)), operand3: Some(Direct(YMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 226, 181, 175, 44, 195], OperandSize::Qword)
}

#[test]
fn vscalefpd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VSCALEFPD, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM18)), operand3: Some(Indirect(RAX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 237, 167, 44, 16], OperandSize::Qword)
}

#[test]
fn vscalefpd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VSCALEFPD, operand1: Some(Direct(YMM19)), operand2: Some(Direct(YMM22)), operand3: Some(IndirectScaledDisplaced(RDI, Two, 1694744112, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 226, 205, 180, 44, 28, 125, 48, 190, 3, 101], OperandSize::Qword)
}

#[test]
fn vscalefpd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VSCALEFPD, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM5)), operand3: Some(Direct(ZMM4)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Down), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 213, 186, 44, 220], OperandSize::Dword)
}

#[test]
fn vscalefpd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VSCALEFPD, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM5)), operand3: Some(IndirectScaledIndexedDisplaced(EDX, ECX, Four, 729613498, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 213, 201, 44, 148, 138, 186, 4, 125, 43], OperandSize::Dword)
}

#[test]
fn vscalefpd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VSCALEFPD, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM4)), operand3: Some(IndirectScaledDisplaced(EDX, Four, 722501243, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 221, 220, 44, 28, 149, 123, 126, 16, 43], OperandSize::Dword)
}

#[test]
fn vscalefpd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VSCALEFPD, operand1: Some(Direct(ZMM12)), operand2: Some(Direct(ZMM22)), operand3: Some(Direct(ZMM15)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Nearest), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 82, 205, 151, 44, 231], OperandSize::Qword)
}

#[test]
fn vscalefpd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VSCALEFPD, operand1: Some(Direct(ZMM20)), operand2: Some(Direct(ZMM8)), operand3: Some(IndirectScaledIndexedDisplaced(RBX, RAX, Two, 1262805538, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 226, 189, 201, 44, 164, 67, 34, 226, 68, 75], OperandSize::Qword)
}

#[test]
fn vscalefpd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VSCALEFPD, operand1: Some(Direct(ZMM30)), operand2: Some(Direct(ZMM5)), operand3: Some(IndirectScaledIndexedDisplaced(RBX, RDI, Four, 323175160, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 98, 213, 218, 44, 180, 187, 248, 66, 67, 19], OperandSize::Qword)
}

