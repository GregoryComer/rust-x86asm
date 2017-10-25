use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn valignd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VALIGND, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM5)), operand4: Some(Literal8(60)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 243, 69, 141, 3, 253, 60], OperandSize::Dword)
}

#[test]
fn valignd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VALIGND, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM5)), operand3: Some(Indirect(EBX, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(25)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 243, 85, 143, 3, 3, 25], OperandSize::Dword)
}

#[test]
fn valignd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VALIGND, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectDisplaced(ESI, 614190997, Some(OperandSize::Dword), None)), operand4: Some(Literal8(94)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 243, 85, 159, 3, 190, 149, 207, 155, 36, 94], OperandSize::Dword)
}

#[test]
fn valignd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VALIGND, operand1: Some(Direct(XMM31)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM15)), operand4: Some(Literal8(31)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 67, 101, 143, 3, 255, 31], OperandSize::Qword)
}

#[test]
fn valignd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VALIGND, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM20)), operand3: Some(IndirectScaledIndexed(RSI, RDI, Eight, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(60)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 243, 93, 130, 3, 44, 254, 60], OperandSize::Qword)
}

#[test]
fn valignd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VALIGND, operand1: Some(Direct(XMM9)), operand2: Some(Direct(XMM22)), operand3: Some(IndirectScaledIndexed(RSI, RDX, Four, Some(OperandSize::Dword), None)), operand4: Some(Literal8(50)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 115, 77, 151, 3, 12, 150, 50], OperandSize::Qword)
}

#[test]
fn valignd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VALIGND, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(YMM7)), operand4: Some(Literal8(25)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 243, 101, 171, 3, 239, 25], OperandSize::Dword)
}

#[test]
fn valignd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VALIGND, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectScaledIndexed(EDX, EDI, Two, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(55)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 243, 85, 170, 3, 44, 122, 55], OperandSize::Dword)
}

#[test]
fn valignd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VALIGND, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM0)), operand3: Some(Indirect(EDX, Some(OperandSize::Dword), None)), operand4: Some(Literal8(78)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 243, 125, 191, 3, 34, 78], OperandSize::Dword)
}

#[test]
fn valignd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VALIGND, operand1: Some(Direct(YMM18)), operand2: Some(Direct(YMM23)), operand3: Some(Direct(YMM0)), operand4: Some(Literal8(93)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 227, 69, 166, 3, 208, 93], OperandSize::Qword)
}

#[test]
fn valignd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VALIGND, operand1: Some(Direct(YMM31)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectScaledIndexed(RSI, RSI, Four, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(58)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 99, 101, 175, 3, 60, 182, 58], OperandSize::Qword)
}

#[test]
fn valignd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VALIGND, operand1: Some(Direct(YMM26)), operand2: Some(Direct(YMM22)), operand3: Some(Indirect(RAX, Some(OperandSize::Dword), None)), operand4: Some(Literal8(104)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 99, 77, 183, 3, 16, 104], OperandSize::Qword)
}

#[test]
fn valignd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VALIGND, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM3)), operand3: Some(Direct(ZMM5)), operand4: Some(Literal8(50)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 243, 101, 202, 3, 197, 50], OperandSize::Dword)
}

#[test]
fn valignd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VALIGND, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM0)), operand3: Some(IndirectScaledDisplaced(EDI, Eight, 1334502879, Some(OperandSize::Zmmword), None)), operand4: Some(Literal8(122)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 243, 125, 207, 3, 52, 253, 223, 229, 138, 79, 122], OperandSize::Dword)
}

#[test]
fn valignd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VALIGND, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM4)), operand3: Some(IndirectScaledIndexedDisplaced(EAX, EBX, Two, 2022847208, Some(OperandSize::Dword), None)), operand4: Some(Literal8(43)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 243, 93, 219, 3, 140, 88, 232, 50, 146, 120, 43], OperandSize::Dword)
}

#[test]
fn valignd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VALIGND, operand1: Some(Direct(ZMM22)), operand2: Some(Direct(ZMM13)), operand3: Some(Direct(ZMM1)), operand4: Some(Literal8(108)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 227, 21, 207, 3, 241, 108], OperandSize::Qword)
}

#[test]
fn valignd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VALIGND, operand1: Some(Direct(ZMM23)), operand2: Some(Direct(ZMM3)), operand3: Some(Indirect(RAX, Some(OperandSize::Zmmword), None)), operand4: Some(Literal8(24)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 227, 101, 207, 3, 56, 24], OperandSize::Qword)
}

#[test]
fn valignd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VALIGND, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM6)), operand3: Some(IndirectScaledDisplaced(RDI, Eight, 234498045, Some(OperandSize::Dword), None)), operand4: Some(Literal8(125)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 243, 77, 222, 3, 28, 253, 253, 39, 250, 13, 125], OperandSize::Qword)
}

