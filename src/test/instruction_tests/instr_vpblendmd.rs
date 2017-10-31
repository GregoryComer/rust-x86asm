use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpblendmd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDMD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 69, 137, 100, 209], OperandSize::Dword)
}

#[test]
fn vpblendmd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDMD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectDisplaced(ECX, 1420170697, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 93, 142, 100, 185, 201, 21, 166, 84], OperandSize::Dword)
}

#[test]
fn vpblendmd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDMD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledIndexed(EAX, EBX, Two, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 69, 155, 100, 20, 88], OperandSize::Dword)
}

#[test]
fn vpblendmd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDMD, operand1: Some(Direct(XMM18)), operand2: Some(Direct(XMM17)), operand3: Some(Direct(XMM27)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 130, 117, 133, 100, 211], OperandSize::Qword)
}

#[test]
fn vpblendmd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDMD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledIndexed(RAX, RDX, Four, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 93, 140, 100, 52, 144], OperandSize::Qword)
}

#[test]
fn vpblendmd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDMD, operand1: Some(Direct(XMM14)), operand2: Some(Direct(XMM15)), operand3: Some(IndirectScaledIndexed(RSI, RSI, Two, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 114, 5, 153, 100, 52, 118], OperandSize::Qword)
}

#[test]
fn vpblendmd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDMD, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 109, 171, 100, 243], OperandSize::Dword)
}

#[test]
fn vpblendmd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDMD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectDisplaced(EDX, 1026436810, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 69, 169, 100, 186, 202, 46, 46, 61], OperandSize::Dword)
}

#[test]
fn vpblendmd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDMD, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectScaledIndexedDisplaced(EAX, EBX, Two, 1523556114, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 117, 186, 100, 156, 88, 18, 159, 207, 90], OperandSize::Dword)
}

#[test]
fn vpblendmd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDMD, operand1: Some(Direct(YMM27)), operand2: Some(Direct(YMM25)), operand3: Some(Direct(YMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 98, 53, 166, 100, 220], OperandSize::Qword)
}

#[test]
fn vpblendmd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDMD, operand1: Some(Direct(YMM31)), operand2: Some(Direct(YMM14)), operand3: Some(Indirect(RAX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 98, 13, 171, 100, 56], OperandSize::Qword)
}

#[test]
fn vpblendmd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDMD, operand1: Some(Direct(YMM8)), operand2: Some(Direct(YMM12)), operand3: Some(IndirectScaledIndexedDisplaced(RBX, RAX, Eight, 1832705277, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 114, 29, 187, 100, 132, 195, 253, 220, 60, 109], OperandSize::Qword)
}

#[test]
fn vpblendmd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDMD, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM7)), operand3: Some(Direct(ZMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 69, 203, 100, 210], OperandSize::Dword)
}

#[test]
fn vpblendmd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDMD, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM3)), operand3: Some(IndirectScaledIndexed(EBX, EBX, Two, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 101, 204, 100, 20, 91], OperandSize::Dword)
}

#[test]
fn vpblendmd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDMD, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM5)), operand3: Some(IndirectScaledIndexed(ESI, EBX, Four, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 242, 85, 221, 100, 44, 158], OperandSize::Dword)
}

#[test]
fn vpblendmd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDMD, operand1: Some(Direct(ZMM23)), operand2: Some(Direct(ZMM2)), operand3: Some(Direct(ZMM14)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 194, 109, 204, 100, 254], OperandSize::Qword)
}

#[test]
fn vpblendmd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDMD, operand1: Some(Direct(ZMM28)), operand2: Some(Direct(ZMM13)), operand3: Some(IndirectScaledIndexed(RDX, RCX, Four, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 98, 21, 207, 100, 36, 138], OperandSize::Qword)
}

#[test]
fn vpblendmd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDMD, operand1: Some(Direct(ZMM26)), operand2: Some(Direct(ZMM30)), operand3: Some(IndirectDisplaced(RDI, 864017018, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 98, 13, 209, 100, 151, 122, 218, 127, 51], OperandSize::Qword)
}

