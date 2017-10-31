use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpackusdw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKUSDW, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 113, 43, 243], OperandSize::Dword)
}

#[test]
fn vpackusdw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKUSDW, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectScaledDisplaced(EDX, Eight, 897748907, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 113, 43, 36, 213, 171, 143, 130, 53], OperandSize::Dword)
}

#[test]
fn vpackusdw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKUSDW, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 65, 43, 241], OperandSize::Qword)
}

#[test]
fn vpackusdw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKUSDW, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectDisplaced(RSI, 1203547907, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 113, 43, 166, 3, 175, 188, 71], OperandSize::Qword)
}

#[test]
fn vpackusdw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKUSDW, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(YMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 85, 43, 193], OperandSize::Dword)
}

#[test]
fn vpackusdw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKUSDW, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectDisplaced(ESI, 1666757701, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 93, 43, 182, 69, 180, 88, 99], OperandSize::Dword)
}

#[test]
fn vpackusdw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKUSDW, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 117, 43, 197], OperandSize::Qword)
}

#[test]
fn vpackusdw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKUSDW, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectDisplaced(RDI, 634486604, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 101, 43, 191, 76, 127, 209, 37], OperandSize::Qword)
}

#[test]
fn vpackusdw_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKUSDW, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 117, 137, 43, 207], OperandSize::Dword)
}

#[test]
fn vpackusdw_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKUSDW, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledIndexed(EAX, EDI, Four, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 69, 143, 43, 36, 184], OperandSize::Dword)
}

#[test]
fn vpackusdw_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKUSDW, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectDisplaced(EAX, 1948845592, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 101, 158, 43, 152, 24, 6, 41, 116], OperandSize::Dword)
}

#[test]
fn vpackusdw_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKUSDW, operand1: Some(Direct(XMM22)), operand2: Some(Direct(XMM9)), operand3: Some(Direct(XMM29)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 130, 53, 137, 43, 245], OperandSize::Qword)
}

#[test]
fn vpackusdw_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKUSDW, operand1: Some(Direct(XMM17)), operand2: Some(Direct(XMM15)), operand3: Some(IndirectScaledIndexed(RCX, RBX, Eight, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 226, 5, 138, 43, 12, 217], OperandSize::Qword)
}

#[test]
fn vpackusdw_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKUSDW, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM19)), operand3: Some(IndirectScaledIndexed(RSI, RCX, Eight, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 101, 146, 43, 4, 206], OperandSize::Qword)
}

#[test]
fn vpackusdw_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKUSDW, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM1)), operand3: Some(Direct(ZMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 117, 203, 43, 251], OperandSize::Dword)
}

#[test]
fn vpackusdw_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKUSDW, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM4)), operand3: Some(IndirectScaledDisplaced(ECX, Two, 676329496, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 93, 204, 43, 4, 77, 24, 248, 79, 40], OperandSize::Dword)
}

#[test]
fn vpackusdw_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKUSDW, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM6)), operand3: Some(IndirectScaledIndexed(EBX, EDX, Two, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 242, 77, 218, 43, 12, 83], OperandSize::Dword)
}

#[test]
fn vpackusdw_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKUSDW, operand1: Some(Direct(ZMM29)), operand2: Some(Direct(ZMM14)), operand3: Some(Direct(ZMM12)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 66, 13, 207, 43, 236], OperandSize::Qword)
}

#[test]
fn vpackusdw_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKUSDW, operand1: Some(Direct(ZMM16)), operand2: Some(Direct(ZMM14)), operand3: Some(IndirectScaledIndexed(RBX, RBX, Eight, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 226, 13, 201, 43, 4, 219], OperandSize::Qword)
}

#[test]
fn vpackusdw_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKUSDW, operand1: Some(Direct(ZMM9)), operand2: Some(Direct(ZMM23)), operand3: Some(Indirect(RDX, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 114, 69, 215, 43, 10], OperandSize::Qword)
}

