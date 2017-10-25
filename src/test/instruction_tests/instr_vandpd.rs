use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vandpd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDPD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 233, 84, 237], OperandSize::Dword)
}

#[test]
fn vandpd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDPD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledIndexed(EDI, EDX, Eight, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 217, 84, 60, 215], OperandSize::Dword)
}

#[test]
fn vandpd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDPD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 233, 84, 219], OperandSize::Qword)
}

#[test]
fn vandpd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDPD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM4)), operand3: Some(Indirect(RBX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 217, 84, 43], OperandSize::Qword)
}

#[test]
fn vandpd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDPD, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(YMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 84, 215], OperandSize::Dword)
}

#[test]
fn vandpd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDPD, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectScaledIndexedDisplaced(EDI, ECX, Two, 3198866, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 197, 84, 140, 79, 146, 207, 48, 0], OperandSize::Dword)
}

#[test]
fn vandpd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDPD, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 245, 84, 227], OperandSize::Qword)
}

#[test]
fn vandpd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDPD, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectScaledIndexed(RBX, RDX, Eight, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 205, 84, 4, 211], OperandSize::Qword)
}

#[test]
fn vandpd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDPD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 221, 141, 84, 226], OperandSize::Dword)
}

#[test]
fn vandpd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDPD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM7)), operand3: Some(Indirect(ESI, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 197, 138, 84, 62], OperandSize::Dword)
}

#[test]
fn vandpd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDPD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledDisplaced(EDI, Eight, 1514069820, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 241, 221, 158, 84, 36, 253, 60, 223, 62, 90], OperandSize::Dword)
}

#[test]
fn vandpd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDPD, operand1: Some(Direct(XMM31)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 97, 213, 142, 84, 248], OperandSize::Qword)
}

#[test]
fn vandpd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDPD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectDisplaced(RDI, 2122204265, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 253, 143, 84, 135, 105, 68, 126, 126], OperandSize::Qword)
}

#[test]
fn vandpd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDPD, operand1: Some(Direct(XMM19)), operand2: Some(Direct(XMM29)), operand3: Some(IndirectScaledDisplaced(RDI, Two, 1239713417, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 225, 149, 147, 84, 28, 125, 137, 134, 228, 73], OperandSize::Qword)
}

#[test]
fn vandpd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDPD, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(YMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 253, 172, 84, 245], OperandSize::Dword)
}

#[test]
fn vandpd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDPD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectScaledIndexed(EAX, EAX, Two, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 213, 170, 84, 44, 64], OperandSize::Dword)
}

#[test]
fn vandpd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDPD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM0)), operand3: Some(Indirect(ECX, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 253, 187, 84, 41], OperandSize::Dword)
}

#[test]
fn vandpd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDPD, operand1: Some(Direct(YMM23)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 225, 245, 171, 84, 253], OperandSize::Qword)
}

#[test]
fn vandpd_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDPD, operand1: Some(Direct(YMM24)), operand2: Some(Direct(YMM24)), operand3: Some(Indirect(RSI, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 97, 189, 162, 84, 6], OperandSize::Qword)
}

#[test]
fn vandpd_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDPD, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM12)), operand3: Some(IndirectScaledIndexed(RSI, RSI, Two, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 157, 188, 84, 36, 118], OperandSize::Qword)
}

#[test]
fn vandpd_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDPD, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM0)), operand3: Some(Direct(ZMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 253, 206, 84, 236], OperandSize::Dword)
}

#[test]
fn vandpd_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDPD, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM1)), operand3: Some(IndirectScaledIndexed(EDX, EDX, Eight, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 245, 205, 84, 36, 210], OperandSize::Dword)
}

#[test]
fn vandpd_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDPD, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM1)), operand3: Some(IndirectScaledIndexed(EBX, EDI, Two, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 245, 219, 84, 36, 123], OperandSize::Dword)
}

#[test]
fn vandpd_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDPD, operand1: Some(Direct(ZMM13)), operand2: Some(Direct(ZMM8)), operand3: Some(Direct(ZMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 113, 189, 205, 84, 233], OperandSize::Qword)
}

#[test]
fn vandpd_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDPD, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM29)), operand3: Some(IndirectScaledDisplaced(RSI, Eight, 1450377283, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 149, 198, 84, 12, 245, 67, 0, 115, 86], OperandSize::Qword)
}

#[test]
fn vandpd_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDPD, operand1: Some(Direct(ZMM8)), operand2: Some(Direct(ZMM28)), operand3: Some(IndirectScaledIndexedDisplaced(RSI, RAX, Four, 1193644895, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 113, 157, 210, 84, 132, 134, 95, 147, 37, 71], OperandSize::Qword)
}

