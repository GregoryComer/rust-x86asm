use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpackusdw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKUSDW, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 105, 43, 252], OperandSize::Dword)
}

#[test]
fn vpackusdw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKUSDW, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM0)), operand3: Some(Indirect(EDX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 43, 50], OperandSize::Dword)
}

#[test]
fn vpackusdw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKUSDW, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 65, 43, 210], OperandSize::Qword)
}

#[test]
fn vpackusdw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKUSDW, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM6)), operand3: Some(Indirect(RDI, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 73, 43, 47], OperandSize::Qword)
}

#[test]
fn vpackusdw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKUSDW, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 69, 43, 194], OperandSize::Dword)
}

#[test]
fn vpackusdw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKUSDW, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectScaledIndexedDisplaced(EBX, EDI, Eight, 1373347902, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 117, 43, 180, 251, 62, 160, 219, 81], OperandSize::Dword)
}

#[test]
fn vpackusdw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKUSDW, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 117, 43, 235], OperandSize::Qword)
}

#[test]
fn vpackusdw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKUSDW, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectScaledIndexed(RDI, RDI, Four, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 85, 43, 60, 191], OperandSize::Qword)
}

#[test]
fn vpackusdw_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKUSDW, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 109, 138, 43, 200], OperandSize::Dword)
}

#[test]
fn vpackusdw_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKUSDW, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledIndexed(ESI, EAX, Two, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 93, 143, 43, 20, 70], OperandSize::Dword)
}

#[test]
fn vpackusdw_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKUSDW, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledIndexedDisplaced(ECX, EBX, Eight, 52422005, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 93, 156, 43, 140, 217, 117, 229, 31, 3], OperandSize::Dword)
}

#[test]
fn vpackusdw_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKUSDW, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM23)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 178, 117, 141, 43, 255], OperandSize::Qword)
}

#[test]
fn vpackusdw_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKUSDW, operand1: Some(Direct(XMM30)), operand2: Some(Direct(XMM8)), operand3: Some(IndirectScaledDisplaced(RAX, Eight, 1950825518, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 98, 61, 139, 43, 52, 197, 46, 60, 71, 116], OperandSize::Qword)
}

#[test]
fn vpackusdw_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKUSDW, operand1: Some(Direct(XMM22)), operand2: Some(Direct(XMM29)), operand3: Some(IndirectScaledIndexed(RAX, RDI, Four, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 226, 21, 149, 43, 52, 184], OperandSize::Qword)
}

#[test]
fn vpackusdw_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKUSDW, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM2)), operand3: Some(Direct(ZMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 109, 204, 43, 241], OperandSize::Dword)
}

#[test]
fn vpackusdw_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKUSDW, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM2)), operand3: Some(IndirectScaledDisplaced(EAX, Two, 1771103025, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 109, 204, 43, 28, 69, 49, 227, 144, 105], OperandSize::Dword)
}

#[test]
fn vpackusdw_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKUSDW, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM3)), operand3: Some(Indirect(EDX, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 242, 101, 221, 43, 26], OperandSize::Dword)
}

#[test]
fn vpackusdw_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKUSDW, operand1: Some(Direct(ZMM28)), operand2: Some(Direct(ZMM28)), operand3: Some(Direct(ZMM31)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 2, 29, 198, 43, 231], OperandSize::Qword)
}

#[test]
fn vpackusdw_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKUSDW, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM14)), operand3: Some(IndirectScaledIndexedDisplaced(RBX, RBX, Two, 969869669, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 13, 203, 43, 164, 91, 101, 9, 207, 57], OperandSize::Qword)
}

#[test]
fn vpackusdw_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKUSDW, operand1: Some(Direct(ZMM9)), operand2: Some(Direct(ZMM1)), operand3: Some(Indirect(RCX, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 114, 117, 221, 43, 9], OperandSize::Qword)
}

