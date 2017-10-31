use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpord_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPORD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 93, 137, 235, 251], OperandSize::Dword)
}

#[test]
fn vpord_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPORD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectScaledIndexed(ECX, EDI, Four, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 77, 143, 235, 12, 185], OperandSize::Dword)
}

#[test]
fn vpord_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPORD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledDisplaced(EDX, Two, 1867736829, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 125, 154, 235, 36, 85, 253, 102, 83, 111], OperandSize::Dword)
}

#[test]
fn vpord_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPORD, operand1: Some(Direct(XMM28)), operand2: Some(Direct(XMM10)), operand3: Some(Direct(XMM21)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 33, 45, 140, 235, 229], OperandSize::Qword)
}

#[test]
fn vpord_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPORD, operand1: Some(Direct(XMM13)), operand2: Some(Direct(XMM19)), operand3: Some(IndirectScaledIndexed(RCX, RSI, Eight, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 113, 101, 131, 235, 44, 241], OperandSize::Qword)
}

#[test]
fn vpord_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPORD, operand1: Some(Direct(XMM19)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledDisplaced(RDI, Four, 1781212733, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 225, 85, 159, 235, 28, 189, 61, 38, 43, 106], OperandSize::Qword)
}

#[test]
fn vpord_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPORD, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 69, 170, 235, 243], OperandSize::Dword)
}

#[test]
fn vpord_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPORD, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectScaledDisplaced(EBX, Eight, 106127727, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 77, 172, 235, 4, 221, 111, 97, 83, 6], OperandSize::Dword)
}

#[test]
fn vpord_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPORD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectScaledDisplaced(EBX, Four, 1808464946, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 109, 190, 235, 60, 157, 50, 252, 202, 107], OperandSize::Dword)
}

#[test]
fn vpord_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPORD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM27)), operand3: Some(Direct(YMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 37, 166, 235, 253], OperandSize::Qword)
}

#[test]
fn vpord_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPORD, operand1: Some(Direct(YMM21)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectDisplaced(RSI, 2069383801, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 225, 69, 169, 235, 174, 121, 74, 88, 123], OperandSize::Qword)
}

#[test]
fn vpord_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPORD, operand1: Some(Direct(YMM26)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectScaledDisplaced(RCX, Two, 1929530565, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 97, 109, 189, 235, 20, 77, 197, 76, 2, 115], OperandSize::Qword)
}

#[test]
fn vpord_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPORD, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM0)), operand3: Some(Direct(ZMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 125, 203, 235, 210], OperandSize::Dword)
}

#[test]
fn vpord_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPORD, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM0)), operand3: Some(IndirectScaledDisplaced(EDX, Four, 1071722966, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 125, 203, 235, 60, 149, 214, 49, 225, 63], OperandSize::Dword)
}

#[test]
fn vpord_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPORD, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM2)), operand3: Some(IndirectScaledIndexed(EAX, ESI, Eight, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 241, 109, 223, 235, 20, 240], OperandSize::Dword)
}

#[test]
fn vpord_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPORD, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM18)), operand3: Some(Direct(ZMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 109, 193, 235, 204], OperandSize::Qword)
}

#[test]
fn vpord_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPORD, operand1: Some(Direct(ZMM12)), operand2: Some(Direct(ZMM26)), operand3: Some(IndirectScaledIndexed(RCX, RDI, Eight, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 113, 45, 198, 235, 36, 249], OperandSize::Qword)
}

#[test]
fn vpord_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPORD, operand1: Some(Direct(ZMM28)), operand2: Some(Direct(ZMM4)), operand3: Some(IndirectScaledIndexed(RBX, RSI, Two, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 97, 93, 219, 235, 36, 115], OperandSize::Qword)
}

