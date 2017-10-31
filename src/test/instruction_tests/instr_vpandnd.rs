use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpandnd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDND, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 125, 141, 223, 248], OperandSize::Dword)
}

#[test]
fn vpandnd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDND, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledDisplaced(ECX, Two, 925063785, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 125, 143, 223, 4, 77, 105, 90, 35, 55], OperandSize::Dword)
}

#[test]
fn vpandnd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDND, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectScaledIndexed(EAX, EAX, Four, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 117, 156, 223, 28, 128], OperandSize::Dword)
}

#[test]
fn vpandnd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDND, operand1: Some(Direct(XMM10)), operand2: Some(Direct(XMM26)), operand3: Some(Direct(XMM28)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 17, 45, 135, 223, 212], OperandSize::Qword)
}

#[test]
fn vpandnd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDND, operand1: Some(Direct(XMM15)), operand2: Some(Direct(XMM23)), operand3: Some(Indirect(RDX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 113, 69, 135, 223, 58], OperandSize::Qword)
}

#[test]
fn vpandnd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDND, operand1: Some(Direct(XMM14)), operand2: Some(Direct(XMM23)), operand3: Some(IndirectScaledIndexedDisplaced(RDX, RAX, Four, 1837152296, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 113, 69, 150, 223, 180, 130, 40, 184, 128, 109], OperandSize::Qword)
}

#[test]
fn vpandnd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDND, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 117, 174, 223, 238], OperandSize::Dword)
}

#[test]
fn vpandnd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDND, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectScaledIndexed(EBX, EDI, Two, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 109, 170, 223, 52, 123], OperandSize::Dword)
}

#[test]
fn vpandnd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDND, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM5)), operand3: Some(Indirect(EBX, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 85, 188, 223, 27], OperandSize::Dword)
}

#[test]
fn vpandnd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDND, operand1: Some(Direct(YMM15)), operand2: Some(Direct(YMM18)), operand3: Some(Direct(YMM12)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 81, 109, 163, 223, 252], OperandSize::Qword)
}

#[test]
fn vpandnd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDND, operand1: Some(Direct(YMM24)), operand2: Some(Direct(YMM23)), operand3: Some(Indirect(RDI, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 97, 69, 164, 223, 7], OperandSize::Qword)
}

#[test]
fn vpandnd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDND, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM29)), operand3: Some(Indirect(RSI, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 21, 178, 223, 22], OperandSize::Qword)
}

#[test]
fn vpandnd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDND, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM2)), operand3: Some(Direct(ZMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 109, 206, 223, 195], OperandSize::Dword)
}

#[test]
fn vpandnd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDND, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM7)), operand3: Some(IndirectScaledIndexed(EAX, EDI, Eight, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 69, 205, 223, 52, 248], OperandSize::Dword)
}

#[test]
fn vpandnd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDND, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM5)), operand3: Some(Indirect(ECX, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 241, 85, 223, 223, 9], OperandSize::Dword)
}

#[test]
fn vpandnd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDND, operand1: Some(Direct(ZMM8)), operand2: Some(Direct(ZMM13)), operand3: Some(Direct(ZMM11)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 81, 21, 205, 223, 195], OperandSize::Qword)
}

#[test]
fn vpandnd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDND, operand1: Some(Direct(ZMM12)), operand2: Some(Direct(ZMM2)), operand3: Some(Indirect(RDI, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 113, 109, 203, 223, 39], OperandSize::Qword)
}

#[test]
fn vpandnd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDND, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM25)), operand3: Some(IndirectScaledDisplaced(RAX, Two, 34264151, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 241, 53, 210, 223, 4, 69, 87, 212, 10, 2], OperandSize::Qword)
}

