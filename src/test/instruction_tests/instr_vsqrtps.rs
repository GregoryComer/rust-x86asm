use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vsqrtps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VSQRTPS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 81, 222], OperandSize::Dword)
}

#[test]
fn vsqrtps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VSQRTPS, operand1: Some(Direct(XMM2)), operand2: Some(IndirectScaledDisplaced(ESI, Eight, 1581496791, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 81, 20, 245, 215, 185, 67, 94], OperandSize::Dword)
}

#[test]
fn vsqrtps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VSQRTPS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 81, 204], OperandSize::Qword)
}

#[test]
fn vsqrtps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VSQRTPS, operand1: Some(Direct(XMM5)), operand2: Some(Indirect(RAX, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 81, 40], OperandSize::Qword)
}

#[test]
fn vsqrtps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VSQRTPS, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 252, 81, 198], OperandSize::Dword)
}

#[test]
fn vsqrtps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VSQRTPS, operand1: Some(Direct(YMM2)), operand2: Some(IndirectScaledDisplaced(EDX, Eight, 724583543, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 252, 81, 20, 213, 119, 68, 48, 43], OperandSize::Dword)
}

#[test]
fn vsqrtps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VSQRTPS, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 252, 81, 196], OperandSize::Qword)
}

#[test]
fn vsqrtps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VSQRTPS, operand1: Some(Direct(YMM1)), operand2: Some(Indirect(RBX, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 252, 81, 11], OperandSize::Qword)
}

#[test]
fn vsqrtps_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VSQRTPS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 124, 141, 81, 223], OperandSize::Dword)
}

#[test]
fn vsqrtps_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VSQRTPS, operand1: Some(Direct(XMM0)), operand2: Some(IndirectDisplaced(EBX, 877757193, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 124, 141, 81, 131, 9, 131, 81, 52], OperandSize::Dword)
}

#[test]
fn vsqrtps_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VSQRTPS, operand1: Some(Direct(XMM7)), operand2: Some(IndirectScaledDisplaced(ECX, Two, 528785572, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 124, 159, 81, 60, 77, 164, 160, 132, 31], OperandSize::Dword)
}

#[test]
fn vsqrtps_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VSQRTPS, operand1: Some(Direct(XMM14)), operand2: Some(Direct(XMM19)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 49, 124, 141, 81, 243], OperandSize::Qword)
}

#[test]
fn vsqrtps_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VSQRTPS, operand1: Some(Direct(XMM29)), operand2: Some(IndirectScaledIndexed(RSI, RSI, Eight, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 97, 124, 142, 81, 44, 246], OperandSize::Qword)
}

#[test]
fn vsqrtps_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VSQRTPS, operand1: Some(Direct(XMM16)), operand2: Some(Indirect(RDI, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 225, 124, 158, 81, 7], OperandSize::Qword)
}

#[test]
fn vsqrtps_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VSQRTPS, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 124, 170, 81, 248], OperandSize::Dword)
}

#[test]
fn vsqrtps_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VSQRTPS, operand1: Some(Direct(YMM4)), operand2: Some(IndirectScaledIndexed(EAX, ECX, Eight, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 124, 175, 81, 36, 200], OperandSize::Dword)
}

#[test]
fn vsqrtps_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VSQRTPS, operand1: Some(Direct(YMM4)), operand2: Some(IndirectScaledDisplaced(ECX, Four, 488054474, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 124, 191, 81, 36, 141, 202, 30, 23, 29], OperandSize::Dword)
}

#[test]
fn vsqrtps_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VSQRTPS, operand1: Some(Direct(YMM23)), operand2: Some(Direct(YMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 225, 124, 175, 81, 250], OperandSize::Qword)
}

#[test]
fn vsqrtps_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VSQRTPS, operand1: Some(Direct(YMM1)), operand2: Some(IndirectScaledDisplaced(RBX, Eight, 1146990658, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 124, 172, 81, 12, 221, 66, 176, 93, 68], OperandSize::Qword)
}

#[test]
fn vsqrtps_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VSQRTPS, operand1: Some(Direct(YMM17)), operand2: Some(IndirectDisplaced(RAX, 1498110239, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 225, 124, 188, 81, 136, 31, 89, 75, 89], OperandSize::Qword)
}

#[test]
fn vsqrtps_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VSQRTPS, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM7)), operand3: None, operand4: None, lock: false, rounding_mode: Some(RoundingMode::Up), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 124, 223, 81, 231], OperandSize::Dword)
}

#[test]
fn vsqrtps_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VSQRTPS, operand1: Some(Direct(ZMM1)), operand2: Some(IndirectScaledIndexedDisplaced(EAX, EAX, Four, 125430951, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 124, 207, 81, 140, 128, 167, 236, 121, 7], OperandSize::Dword)
}

#[test]
fn vsqrtps_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VSQRTPS, operand1: Some(Direct(ZMM4)), operand2: Some(IndirectDisplaced(EDX, 1387943257, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 241, 124, 221, 81, 162, 89, 85, 186, 82], OperandSize::Dword)
}

#[test]
fn vsqrtps_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VSQRTPS, operand1: Some(Direct(ZMM28)), operand2: Some(Direct(ZMM24)), operand3: None, operand4: None, lock: false, rounding_mode: Some(RoundingMode::Nearest), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 1, 124, 155, 81, 224], OperandSize::Qword)
}

#[test]
fn vsqrtps_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VSQRTPS, operand1: Some(Direct(ZMM13)), operand2: Some(IndirectScaledIndexedDisplaced(RSI, RCX, Eight, 281311378, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 113, 124, 207, 81, 172, 206, 146, 120, 196, 16], OperandSize::Qword)
}

#[test]
fn vsqrtps_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VSQRTPS, operand1: Some(Direct(ZMM29)), operand2: Some(Indirect(RCX, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 97, 124, 222, 81, 41], OperandSize::Qword)
}

