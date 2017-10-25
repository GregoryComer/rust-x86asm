use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpandnd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDND, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 109, 143, 223, 204], OperandSize::Dword)
}

#[test]
fn vpandnd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDND, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledIndexed(EDI, EAX, Four, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 101, 140, 223, 60, 135], OperandSize::Dword)
}

#[test]
fn vpandnd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDND, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledIndexed(EBX, ECX, Four, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 85, 159, 223, 4, 139], OperandSize::Dword)
}

#[test]
fn vpandnd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDND, operand1: Some(Direct(XMM17)), operand2: Some(Direct(XMM22)), operand3: Some(Direct(XMM18)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 161, 77, 133, 223, 202], OperandSize::Qword)
}

#[test]
fn vpandnd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDND, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM29)), operand3: Some(Indirect(RAX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 21, 132, 223, 0], OperandSize::Qword)
}

#[test]
fn vpandnd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDND, operand1: Some(Direct(XMM18)), operand2: Some(Direct(XMM19)), operand3: Some(IndirectScaledIndexed(RSI, RSI, Two, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 225, 101, 148, 223, 20, 118], OperandSize::Qword)
}

#[test]
fn vpandnd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDND, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(YMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 85, 170, 223, 219], OperandSize::Dword)
}

#[test]
fn vpandnd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDND, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectScaledDisplaced(EBX, Two, 804445520, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 101, 170, 223, 20, 93, 80, 221, 242, 47], OperandSize::Dword)
}

#[test]
fn vpandnd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDND, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectScaledIndexed(EAX, ECX, Four, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 85, 185, 223, 20, 136], OperandSize::Dword)
}

#[test]
fn vpandnd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDND, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM8)), operand3: Some(Direct(YMM29)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 145, 61, 172, 223, 245], OperandSize::Qword)
}

#[test]
fn vpandnd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDND, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectScaledIndexedDisplaced(RSI, RCX, Four, 271357790, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 109, 170, 223, 132, 142, 94, 151, 44, 16], OperandSize::Qword)
}

#[test]
fn vpandnd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDND, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM25)), operand3: Some(IndirectDisplaced(RCX, 1172597368, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 53, 183, 223, 177, 120, 106, 228, 69], OperandSize::Qword)
}

#[test]
fn vpandnd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDND, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM6)), operand3: Some(Direct(ZMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 77, 205, 223, 203], OperandSize::Dword)
}

#[test]
fn vpandnd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDND, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM6)), operand3: Some(IndirectScaledDisplaced(EDI, Eight, 963884518, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 77, 203, 223, 20, 253, 230, 181, 115, 57], OperandSize::Dword)
}

#[test]
fn vpandnd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDND, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM3)), operand3: Some(IndirectScaledIndexedDisplaced(EBX, EDI, Eight, 1370801102, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 241, 101, 220, 223, 148, 251, 206, 195, 180, 81], OperandSize::Dword)
}

#[test]
fn vpandnd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDND, operand1: Some(Direct(ZMM19)), operand2: Some(Direct(ZMM4)), operand3: Some(Direct(ZMM15)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 193, 93, 203, 223, 223], OperandSize::Qword)
}

#[test]
fn vpandnd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDND, operand1: Some(Direct(ZMM31)), operand2: Some(Direct(ZMM31)), operand3: Some(Indirect(RDI, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 97, 5, 194, 223, 63], OperandSize::Qword)
}

#[test]
fn vpandnd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDND, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM31)), operand3: Some(IndirectScaledIndexedDisplaced(RSI, RDI, Two, 355106309, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 241, 5, 209, 223, 148, 126, 5, 126, 42, 21], OperandSize::Qword)
}

