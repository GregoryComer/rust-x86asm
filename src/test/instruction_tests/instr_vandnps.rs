use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vandnps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDNPS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 200, 85, 224], OperandSize::Dword)
}

#[test]
fn vandnps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDNPS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectDisplaced(ECX, 9408053, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 240, 85, 137, 53, 142, 143, 0], OperandSize::Dword)
}

#[test]
fn vandnps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDNPS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 208, 85, 251], OperandSize::Qword)
}

#[test]
fn vandnps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDNPS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledDisplaced(RDI, Four, 1308326069, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 85, 52, 189, 181, 120, 251, 77], OperandSize::Qword)
}

#[test]
fn vandnps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDNPS, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(YMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 204, 85, 233], OperandSize::Dword)
}

#[test]
fn vandnps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDNPS, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM1)), operand3: Some(Indirect(EDX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 244, 85, 26], OperandSize::Dword)
}

#[test]
fn vandnps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDNPS, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(YMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 252, 85, 215], OperandSize::Qword)
}

#[test]
fn vandnps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDNPS, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectScaledDisplaced(RCX, Two, 1665558861, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 244, 85, 12, 77, 77, 105, 70, 99], OperandSize::Qword)
}

#[test]
fn vandnps_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDNPS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 92, 141, 85, 230], OperandSize::Dword)
}

#[test]
fn vandnps_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDNPS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledIndexedDisplaced(EDX, EBX, Two, 1734518411, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 84, 143, 85, 164, 90, 139, 166, 98, 103], OperandSize::Dword)
}

#[test]
fn vandnps_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDNPS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM5)), operand3: Some(Indirect(ECX, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 84, 159, 85, 57], OperandSize::Dword)
}

#[test]
fn vandnps_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDNPS, operand1: Some(Direct(XMM8)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM15)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 81, 116, 143, 85, 199], OperandSize::Qword)
}

#[test]
fn vandnps_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDNPS, operand1: Some(Direct(XMM22)), operand2: Some(Direct(XMM25)), operand3: Some(IndirectScaledDisplaced(RCX, Two, 341122162, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 225, 52, 135, 85, 52, 77, 114, 28, 85, 20], OperandSize::Qword)
}

#[test]
fn vandnps_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDNPS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledIndexed(RDI, RDI, Two, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 108, 157, 85, 28, 127], OperandSize::Qword)
}

#[test]
fn vandnps_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDNPS, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(YMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 100, 174, 85, 213], OperandSize::Dword)
}

#[test]
fn vandnps_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDNPS, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectScaledIndexed(EDX, EBX, Four, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 108, 175, 85, 52, 154], OperandSize::Dword)
}

#[test]
fn vandnps_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDNPS, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectDisplaced(EAX, 1148220463, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 92, 191, 85, 144, 47, 116, 112, 68], OperandSize::Dword)
}

#[test]
fn vandnps_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDNPS, operand1: Some(Direct(YMM31)), operand2: Some(Direct(YMM31)), operand3: Some(Direct(YMM18)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 33, 4, 163, 85, 250], OperandSize::Qword)
}

#[test]
fn vandnps_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDNPS, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM26)), operand3: Some(Indirect(RSI, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 44, 165, 85, 22], OperandSize::Qword)
}

#[test]
fn vandnps_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDNPS, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM14)), operand3: Some(IndirectScaledIndexedDisplaced(RDX, RDI, Two, 1093838112, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 12, 189, 85, 180, 122, 32, 165, 50, 65], OperandSize::Qword)
}

#[test]
fn vandnps_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDNPS, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM3)), operand3: Some(Direct(ZMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 100, 204, 85, 193], OperandSize::Dword)
}

#[test]
fn vandnps_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDNPS, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM1)), operand3: Some(IndirectScaledIndexedDisplaced(EDI, ECX, Eight, 728070834, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 116, 203, 85, 172, 207, 178, 122, 101, 43], OperandSize::Dword)
}

#[test]
fn vandnps_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDNPS, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM2)), operand3: Some(Indirect(EAX, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 241, 108, 219, 85, 32], OperandSize::Dword)
}

#[test]
fn vandnps_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDNPS, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM16)), operand3: Some(Direct(ZMM17)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 177, 124, 193, 85, 249], OperandSize::Qword)
}

#[test]
fn vandnps_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDNPS, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM20)), operand3: Some(Indirect(RDX, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 92, 193, 85, 2], OperandSize::Qword)
}

#[test]
fn vandnps_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDNPS, operand1: Some(Direct(ZMM24)), operand2: Some(Direct(ZMM5)), operand3: Some(IndirectScaledDisplaced(RAX, Eight, 1438603633, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 97, 84, 222, 85, 4, 197, 113, 89, 191, 85], OperandSize::Qword)
}

