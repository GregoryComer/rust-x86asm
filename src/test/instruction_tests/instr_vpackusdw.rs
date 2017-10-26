use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpackusdw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKUSDW, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 97, 43, 205], OperandSize::Dword)
}

#[test]
fn vpackusdw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKUSDW, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM3)), operand3: Some(Indirect(EBX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 97, 43, 11], OperandSize::Dword)
}

#[test]
fn vpackusdw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKUSDW, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 81, 43, 241], OperandSize::Qword)
}

#[test]
fn vpackusdw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKUSDW, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectDisplaced(RBX, 414096441, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 97, 43, 131, 57, 156, 174, 24], OperandSize::Qword)
}

#[test]
fn vpackusdw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKUSDW, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(YMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 85, 43, 253], OperandSize::Dword)
}

#[test]
fn vpackusdw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKUSDW, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectScaledDisplaced(EDX, Two, 239407409, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 109, 43, 28, 85, 49, 17, 69, 14], OperandSize::Dword)
}

#[test]
fn vpackusdw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKUSDW, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(YMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 77, 43, 196], OperandSize::Qword)
}

#[test]
fn vpackusdw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKUSDW, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectScaledDisplaced(RAX, Two, 1957172728, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 85, 43, 52, 69, 248, 21, 168, 116], OperandSize::Qword)
}

#[test]
fn vpackusdw_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKUSDW, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 101, 139, 43, 225], OperandSize::Dword)
}

#[test]
fn vpackusdw_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKUSDW, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM7)), operand3: Some(Indirect(EDX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 69, 137, 43, 50], OperandSize::Dword)
}

#[test]
fn vpackusdw_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKUSDW, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledDisplaced(EBX, Four, 2003113201, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 101, 153, 43, 12, 157, 241, 20, 101, 119], OperandSize::Dword)
}

#[test]
fn vpackusdw_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKUSDW, operand1: Some(Direct(XMM30)), operand2: Some(Direct(XMM9)), operand3: Some(Direct(XMM22)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 34, 53, 137, 43, 246], OperandSize::Qword)
}

#[test]
fn vpackusdw_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKUSDW, operand1: Some(Direct(XMM19)), operand2: Some(Direct(XMM20)), operand3: Some(Indirect(RDI, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 226, 93, 134, 43, 31], OperandSize::Qword)
}

#[test]
fn vpackusdw_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKUSDW, operand1: Some(Direct(XMM10)), operand2: Some(Direct(XMM14)), operand3: Some(IndirectScaledDisplaced(RAX, Two, 1687861606, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 114, 13, 158, 43, 20, 69, 102, 185, 154, 100], OperandSize::Qword)
}

#[test]
fn vpackusdw_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKUSDW, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM3)), operand3: Some(Direct(ZMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 101, 206, 43, 233], OperandSize::Dword)
}

#[test]
fn vpackusdw_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKUSDW, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM4)), operand3: Some(Indirect(ESI, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 93, 206, 43, 62], OperandSize::Dword)
}

#[test]
fn vpackusdw_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKUSDW, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM2)), operand3: Some(IndirectScaledIndexedDisplaced(EBX, EDI, Four, 519924757, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 242, 109, 220, 43, 140, 187, 21, 108, 253, 30], OperandSize::Dword)
}

#[test]
fn vpackusdw_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKUSDW, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM13)), operand3: Some(Direct(ZMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 21, 201, 43, 217], OperandSize::Qword)
}

#[test]
fn vpackusdw_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKUSDW, operand1: Some(Direct(ZMM15)), operand2: Some(Direct(ZMM1)), operand3: Some(IndirectScaledIndexedDisplaced(RAX, RAX, Four, 721894465, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 114, 117, 203, 43, 188, 128, 65, 60, 7, 43], OperandSize::Qword)
}

#[test]
fn vpackusdw_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKUSDW, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM6)), operand3: Some(IndirectScaledIndexed(RCX, RDI, Two, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 242, 77, 218, 43, 52, 121], OperandSize::Qword)
}

