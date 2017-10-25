use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpord_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPORD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 77, 140, 235, 248], OperandSize::Dword)
}

#[test]
fn vpord_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPORD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledIndexedDisplaced(ESI, EAX, Eight, 612057063, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 93, 140, 235, 148, 198, 231, 63, 123, 36], OperandSize::Dword)
}

#[test]
fn vpord_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPORD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledIndexedDisplaced(ESI, EBX, Eight, 955823035, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 69, 153, 235, 140, 222, 187, 179, 248, 56], OperandSize::Dword)
}

#[test]
fn vpord_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPORD, operand1: Some(Direct(XMM21)), operand2: Some(Direct(XMM16)), operand3: Some(Direct(XMM31)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 129, 125, 133, 235, 239], OperandSize::Qword)
}

#[test]
fn vpord_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPORD, operand1: Some(Direct(XMM11)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectScaledIndexed(RSI, RBX, Two, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 113, 117, 137, 235, 28, 94], OperandSize::Qword)
}

#[test]
fn vpord_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPORD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM22)), operand3: Some(IndirectScaledIndexedDisplaced(RDI, RDI, Eight, 1297076244, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 77, 148, 235, 172, 255, 20, 208, 79, 77], OperandSize::Qword)
}

#[test]
fn vpord_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPORD, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(YMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 77, 174, 235, 209], OperandSize::Dword)
}

#[test]
fn vpord_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPORD, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectScaledIndexed(EDX, EAX, Four, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 117, 175, 235, 28, 130], OperandSize::Dword)
}

#[test]
fn vpord_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPORD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM0)), operand3: Some(Indirect(ECX, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 125, 191, 235, 41], OperandSize::Dword)
}

#[test]
fn vpord_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPORD, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM12)), operand3: Some(Direct(YMM24)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 145, 29, 170, 235, 240], OperandSize::Qword)
}

#[test]
fn vpord_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPORD, operand1: Some(Direct(YMM30)), operand2: Some(Direct(YMM16)), operand3: Some(IndirectScaledDisplaced(RAX, Two, 763358150, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 97, 125, 164, 235, 52, 69, 198, 235, 127, 45], OperandSize::Qword)
}

#[test]
fn vpord_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPORD, operand1: Some(Direct(YMM31)), operand2: Some(Direct(YMM24)), operand3: Some(IndirectDisplaced(RAX, 1614665744, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 97, 61, 180, 235, 184, 16, 216, 61, 96], OperandSize::Qword)
}

#[test]
fn vpord_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPORD, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM7)), operand3: Some(Direct(ZMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 69, 202, 235, 252], OperandSize::Dword)
}

#[test]
fn vpord_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPORD, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM0)), operand3: Some(IndirectScaledIndexed(ESI, ECX, Two, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 125, 206, 235, 20, 78], OperandSize::Dword)
}

#[test]
fn vpord_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPORD, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM5)), operand3: Some(IndirectDisplaced(EDX, 1476722246, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 241, 85, 222, 235, 162, 70, 254, 4, 88], OperandSize::Dword)
}

#[test]
fn vpord_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPORD, operand1: Some(Direct(ZMM28)), operand2: Some(Direct(ZMM10)), operand3: Some(Direct(ZMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 97, 45, 206, 235, 224], OperandSize::Qword)
}

#[test]
fn vpord_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPORD, operand1: Some(Direct(ZMM21)), operand2: Some(Direct(ZMM9)), operand3: Some(IndirectScaledIndexedDisplaced(RSI, RSI, Two, 1802106504, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 225, 53, 203, 235, 172, 118, 136, 246, 105, 107], OperandSize::Qword)
}

#[test]
fn vpord_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPORD, operand1: Some(Direct(ZMM23)), operand2: Some(Direct(ZMM13)), operand3: Some(IndirectDisplaced(RAX, 1009332957, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 225, 21, 222, 235, 184, 221, 50, 41, 60], OperandSize::Qword)
}

