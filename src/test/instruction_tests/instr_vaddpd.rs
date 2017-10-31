use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vaddpd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDPD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 217, 88, 224], OperandSize::Dword)
}

#[test]
fn vaddpd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDPD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM3)), operand3: Some(Indirect(EDI, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 225, 88, 47], OperandSize::Dword)
}

#[test]
fn vaddpd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDPD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 241, 88, 241], OperandSize::Qword)
}

#[test]
fn vaddpd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDPD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledIndexed(RSI, RBX, Four, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 209, 88, 28, 158], OperandSize::Qword)
}

#[test]
fn vaddpd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDPD, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(YMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 229, 88, 223], OperandSize::Dword)
}

#[test]
fn vaddpd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDPD, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectScaledIndexedDisplaced(ECX, EAX, Eight, 1755500895, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 213, 88, 164, 193, 95, 209, 162, 104], OperandSize::Dword)
}

#[test]
fn vaddpd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDPD, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 197, 88, 246], OperandSize::Qword)
}

#[test]
fn vaddpd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDPD, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectScaledIndexedDisplaced(RBX, RCX, Four, 1946861304, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 197, 88, 164, 139, 248, 190, 10, 116], OperandSize::Qword)
}

#[test]
fn vaddpd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDPD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 237, 140, 88, 244], OperandSize::Dword)
}

#[test]
fn vaddpd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDPD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectScaledIndexed(EBX, EDX, Eight, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 245, 141, 88, 4, 211], OperandSize::Dword)
}

#[test]
fn vaddpd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDPD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledIndexedDisplaced(EBX, EBX, Eight, 1637492746, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 241, 253, 154, 88, 132, 219, 10, 40, 154, 97], OperandSize::Dword)
}

#[test]
fn vaddpd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDPD, operand1: Some(Direct(XMM17)), operand2: Some(Direct(XMM23)), operand3: Some(Direct(XMM31)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 129, 197, 129, 88, 207], OperandSize::Qword)
}

#[test]
fn vaddpd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDPD, operand1: Some(Direct(XMM28)), operand2: Some(Direct(XMM17)), operand3: Some(IndirectScaledDisplaced(RDX, Two, 533882850, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 97, 245, 129, 88, 36, 85, 226, 103, 210, 31], OperandSize::Qword)
}

#[test]
fn vaddpd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDPD, operand1: Some(Direct(XMM19)), operand2: Some(Direct(XMM11)), operand3: Some(Indirect(RAX, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 225, 165, 159, 88, 24], OperandSize::Qword)
}

#[test]
fn vaddpd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDPD, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(YMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 229, 173, 88, 197], OperandSize::Dword)
}

#[test]
fn vaddpd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDPD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM7)), operand3: Some(Indirect(ESI, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 197, 175, 88, 62], OperandSize::Dword)
}

#[test]
fn vaddpd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDPD, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectScaledIndexed(EBX, ECX, Four, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 237, 190, 88, 36, 139], OperandSize::Dword)
}

#[test]
fn vaddpd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDPD, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM21)), operand3: Some(Direct(YMM18)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 177, 213, 165, 88, 210], OperandSize::Qword)
}

#[test]
fn vaddpd_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDPD, operand1: Some(Direct(YMM19)), operand2: Some(Direct(YMM27)), operand3: Some(IndirectDisplaced(RSI, 1663741089, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 225, 165, 162, 88, 158, 161, 172, 42, 99], OperandSize::Qword)
}

#[test]
fn vaddpd_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDPD, operand1: Some(Direct(YMM18)), operand2: Some(Direct(YMM10)), operand3: Some(IndirectScaledDisplaced(RBX, Four, 430736950, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 225, 173, 189, 88, 20, 157, 54, 134, 172, 25], OperandSize::Qword)
}

#[test]
fn vaddpd_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDPD, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM3)), operand3: Some(Direct(ZMM5)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Up), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 229, 220, 88, 229], OperandSize::Dword)
}

#[test]
fn vaddpd_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDPD, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM5)), operand3: Some(IndirectScaledDisplaced(EDI, Four, 271516486, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 213, 207, 88, 20, 189, 70, 3, 47, 16], OperandSize::Dword)
}

#[test]
fn vaddpd_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDPD, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM5)), operand3: Some(IndirectScaledIndexed(ESI, EDX, Eight, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 213, 220, 88, 60, 214], OperandSize::Dword)
}

#[test]
fn vaddpd_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDPD, operand1: Some(Direct(ZMM29)), operand2: Some(Direct(ZMM4)), operand3: Some(Direct(ZMM27)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Zero), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 1, 221, 254, 88, 235], OperandSize::Qword)
}

#[test]
fn vaddpd_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDPD, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM24)), operand3: Some(IndirectScaledIndexed(RDX, RDI, Eight, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 189, 198, 88, 12, 250], OperandSize::Qword)
}

#[test]
fn vaddpd_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDPD, operand1: Some(Direct(ZMM28)), operand2: Some(Direct(ZMM1)), operand3: Some(IndirectScaledIndexedDisplaced(RSI, RBX, Eight, 1979026047, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 97, 245, 222, 88, 164, 222, 127, 138, 245, 117], OperandSize::Qword)
}

