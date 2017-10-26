use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vgetexpps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETEXPPS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 125, 139, 66, 210], OperandSize::Dword)
}

#[test]
fn vgetexpps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETEXPPS, operand1: Some(Direct(XMM2)), operand2: Some(IndirectScaledDisplaced(EDI, Four, 1582381646, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 125, 140, 66, 20, 189, 78, 58, 81, 94], OperandSize::Dword)
}

#[test]
fn vgetexpps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETEXPPS, operand1: Some(Direct(XMM2)), operand2: Some(IndirectScaledIndexedDisplaced(EAX, EBX, Four, 619418481, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 125, 158, 66, 148, 152, 113, 147, 235, 36], OperandSize::Dword)
}

#[test]
fn vgetexpps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETEXPPS, operand1: Some(Direct(XMM24)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 98, 125, 143, 66, 193], OperandSize::Qword)
}

#[test]
fn vgetexpps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETEXPPS, operand1: Some(Direct(XMM20)), operand2: Some(IndirectDisplaced(RCX, 186537134, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 226, 125, 139, 66, 161, 174, 84, 30, 11], OperandSize::Qword)
}

#[test]
fn vgetexpps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETEXPPS, operand1: Some(Direct(XMM17)), operand2: Some(IndirectScaledDisplaced(RSI, Eight, 1583883000, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 226, 125, 159, 66, 12, 245, 248, 34, 104, 94], OperandSize::Qword)
}

#[test]
fn vgetexpps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETEXPPS, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 125, 175, 66, 213], OperandSize::Dword)
}

#[test]
fn vgetexpps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETEXPPS, operand1: Some(Direct(YMM4)), operand2: Some(Indirect(ECX, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 125, 173, 66, 33], OperandSize::Dword)
}

#[test]
fn vgetexpps_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETEXPPS, operand1: Some(Direct(YMM7)), operand2: Some(IndirectScaledIndexedDisplaced(EDI, EDX, Eight, 1466368974, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 125, 185, 66, 188, 215, 206, 3, 103, 87], OperandSize::Dword)
}

#[test]
fn vgetexpps_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETEXPPS, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 125, 174, 66, 252], OperandSize::Qword)
}

#[test]
fn vgetexpps_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETEXPPS, operand1: Some(Direct(YMM31)), operand2: Some(Indirect(RDI, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 98, 125, 173, 66, 63], OperandSize::Qword)
}

#[test]
fn vgetexpps_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETEXPPS, operand1: Some(Direct(YMM17)), operand2: Some(IndirectScaledIndexed(RAX, RDX, Four, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 226, 125, 190, 66, 12, 144], OperandSize::Qword)
}

#[test]
fn vgetexpps_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETEXPPS, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 125, 157, 66, 224], OperandSize::Dword)
}

#[test]
fn vgetexpps_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETEXPPS, operand1: Some(Direct(ZMM3)), operand2: Some(IndirectScaledIndexed(ESI, EAX, Two, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 125, 205, 66, 28, 70], OperandSize::Dword)
}

#[test]
fn vgetexpps_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETEXPPS, operand1: Some(Direct(ZMM6)), operand2: Some(Indirect(ESI, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 242, 125, 218, 66, 54], OperandSize::Dword)
}

#[test]
fn vgetexpps_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETEXPPS, operand1: Some(Direct(ZMM23)), operand2: Some(Direct(ZMM12)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K4), broadcast: None }, &[98, 194, 125, 156, 66, 252], OperandSize::Qword)
}

#[test]
fn vgetexpps_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETEXPPS, operand1: Some(Direct(ZMM22)), operand2: Some(Indirect(RDI, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 226, 125, 203, 66, 55], OperandSize::Qword)
}

#[test]
fn vgetexpps_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETEXPPS, operand1: Some(Direct(ZMM11)), operand2: Some(IndirectDisplaced(RCX, 1694563124, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 114, 125, 223, 66, 153, 52, 251, 0, 101], OperandSize::Qword)
}

