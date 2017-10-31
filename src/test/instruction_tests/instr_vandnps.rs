use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vandnps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDNPS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 208, 85, 236], OperandSize::Dword)
}

#[test]
fn vandnps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDNPS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledIndexedDisplaced(ESI, EBX, Two, 1025501094, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 208, 85, 188, 94, 166, 231, 31, 61], OperandSize::Dword)
}

#[test]
fn vandnps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDNPS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 240, 85, 229], OperandSize::Qword)
}

#[test]
fn vandnps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDNPS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledDisplaced(RCX, Eight, 407207344, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 224, 85, 28, 205, 176, 125, 69, 24], OperandSize::Qword)
}

#[test]
fn vandnps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDNPS, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 236, 85, 230], OperandSize::Dword)
}

#[test]
fn vandnps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDNPS, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectScaledIndexed(EBX, EDX, Two, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 204, 85, 36, 83], OperandSize::Dword)
}

#[test]
fn vandnps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDNPS, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 220, 85, 255], OperandSize::Qword)
}

#[test]
fn vandnps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDNPS, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM3)), operand3: Some(Indirect(RSI, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 228, 85, 30], OperandSize::Qword)
}

#[test]
fn vandnps_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDNPS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 108, 137, 85, 233], OperandSize::Dword)
}

#[test]
fn vandnps_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDNPS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectDisplaced(EAX, 1026195678, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 124, 137, 85, 160, 222, 128, 42, 61], OperandSize::Dword)
}

#[test]
fn vandnps_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDNPS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledIndexed(EAX, EDI, Two, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 100, 158, 85, 52, 120], OperandSize::Dword)
}

#[test]
fn vandnps_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDNPS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM9)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 209, 76, 143, 85, 241], OperandSize::Qword)
}

#[test]
fn vandnps_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDNPS, operand1: Some(Direct(XMM11)), operand2: Some(Direct(XMM18)), operand3: Some(IndirectScaledDisplaced(RDI, Four, 1556048637, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 113, 108, 133, 85, 28, 189, 253, 106, 191, 92], OperandSize::Qword)
}

#[test]
fn vandnps_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDNPS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM16)), operand3: Some(IndirectScaledIndexed(RDI, RAX, Two, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 124, 146, 85, 28, 71], OperandSize::Qword)
}

#[test]
fn vandnps_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDNPS, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(YMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 100, 175, 85, 234], OperandSize::Dword)
}

#[test]
fn vandnps_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDNPS, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectScaledIndexedDisplaced(EBX, ESI, Four, 1392817386, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 84, 175, 85, 164, 179, 234, 180, 4, 83], OperandSize::Dword)
}

#[test]
fn vandnps_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDNPS, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectScaledIndexed(EAX, EAX, Eight, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 100, 189, 85, 4, 192], OperandSize::Dword)
}

#[test]
fn vandnps_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDNPS, operand1: Some(Direct(YMM16)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM21)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 161, 108, 172, 85, 197], OperandSize::Qword)
}

#[test]
fn vandnps_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDNPS, operand1: Some(Direct(YMM19)), operand2: Some(Direct(YMM19)), operand3: Some(Indirect(RSI, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 225, 100, 161, 85, 30], OperandSize::Qword)
}

#[test]
fn vandnps_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDNPS, operand1: Some(Direct(YMM14)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectScaledIndexedDisplaced(RAX, RBX, Eight, 466678629, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 113, 116, 186, 85, 180, 216, 101, 243, 208, 27], OperandSize::Qword)
}

#[test]
fn vandnps_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDNPS, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM3)), operand3: Some(Direct(ZMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 100, 202, 85, 196], OperandSize::Dword)
}

#[test]
fn vandnps_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDNPS, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM6)), operand3: Some(IndirectScaledDisplaced(EBX, Two, 993598387, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 76, 205, 85, 28, 93, 179, 27, 57, 59], OperandSize::Dword)
}

#[test]
fn vandnps_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDNPS, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM5)), operand3: Some(IndirectScaledIndexed(EDI, EAX, Four, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 241, 84, 223, 85, 12, 135], OperandSize::Dword)
}

#[test]
fn vandnps_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDNPS, operand1: Some(Direct(ZMM17)), operand2: Some(Direct(ZMM25)), operand3: Some(Direct(ZMM15)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 193, 52, 196, 85, 207], OperandSize::Qword)
}

#[test]
fn vandnps_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDNPS, operand1: Some(Direct(ZMM27)), operand2: Some(Direct(ZMM25)), operand3: Some(Indirect(RCX, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 97, 52, 196, 85, 25], OperandSize::Qword)
}

#[test]
fn vandnps_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDNPS, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM19)), operand3: Some(IndirectDisplaced(RDI, 280192530, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 241, 100, 212, 85, 135, 18, 102, 179, 16], OperandSize::Qword)
}

