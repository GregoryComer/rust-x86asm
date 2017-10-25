use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vcvtps2ph_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2PH, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM6)), operand3: Some(Literal8(54)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 121, 29, 247, 54], OperandSize::Dword)
}

#[test]
fn vcvtps2ph_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2PH, operand1: Some(Indirect(EAX, Some(OperandSize::Qword), None)), operand2: Some(Direct(XMM6)), operand3: Some(Literal8(11)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 121, 29, 48, 11], OperandSize::Dword)
}

#[test]
fn vcvtps2ph_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2PH, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM5)), operand3: Some(Literal8(4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 121, 29, 233, 4], OperandSize::Qword)
}

#[test]
fn vcvtps2ph_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2PH, operand1: Some(IndirectScaledIndexedDisplaced(RDI, RSI, Four, 60261243, Some(OperandSize::Qword), None)), operand2: Some(Direct(XMM0)), operand3: Some(Literal8(31)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 121, 29, 132, 183, 123, 131, 151, 3, 31], OperandSize::Qword)
}

#[test]
fn vcvtps2ph_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2PH, operand1: Some(Direct(XMM0)), operand2: Some(Direct(YMM5)), operand3: Some(Literal8(28)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 125, 29, 232, 28], OperandSize::Dword)
}

#[test]
fn vcvtps2ph_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2PH, operand1: Some(Indirect(EBX, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(YMM7)), operand3: Some(Literal8(97)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 125, 29, 59, 97], OperandSize::Dword)
}

#[test]
fn vcvtps2ph_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2PH, operand1: Some(Direct(XMM5)), operand2: Some(Direct(YMM6)), operand3: Some(Literal8(60)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 125, 29, 245, 60], OperandSize::Qword)
}

#[test]
fn vcvtps2ph_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2PH, operand1: Some(IndirectDisplaced(RAX, 1723190673, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(YMM4)), operand3: Some(Literal8(82)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 125, 29, 160, 145, 205, 181, 102, 82], OperandSize::Qword)
}

#[test]
fn vcvtps2ph_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2PH, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM5)), operand3: Some(Literal8(14)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 243, 125, 140, 29, 236, 14], OperandSize::Dword)
}

#[test]
fn vcvtps2ph_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2PH, operand1: Some(IndirectScaledIndexedDisplaced(EDX, EDX, Four, 1776609392, Some(OperandSize::Qword), None)), operand2: Some(Direct(XMM6)), operand3: Some(Literal8(109)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 121, 29, 180, 146, 112, 232, 228, 105, 109], OperandSize::Dword)
}

#[test]
fn vcvtps2ph_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2PH, operand1: Some(Direct(XMM22)), operand2: Some(Direct(XMM7)), operand3: Some(Literal8(51)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 179, 125, 141, 29, 254, 51], OperandSize::Qword)
}

#[test]
fn vcvtps2ph_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2PH, operand1: Some(IndirectScaledIndexedDisplaced(RSI, RCX, Four, 622984198, Some(OperandSize::Qword), None)), operand2: Some(Direct(XMM24)), operand3: Some(Literal8(38)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 99, 125, 8, 29, 132, 142, 6, 252, 33, 37, 38], OperandSize::Qword)
}

#[test]
fn vcvtps2ph_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2PH, operand1: Some(Direct(XMM4)), operand2: Some(Direct(YMM7)), operand3: Some(Literal8(44)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 243, 125, 175, 29, 252, 44], OperandSize::Dword)
}

#[test]
fn vcvtps2ph_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2PH, operand1: Some(IndirectScaledIndexed(ECX, EDX, Four, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(YMM4)), operand3: Some(Literal8(116)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 125, 29, 36, 145, 116], OperandSize::Dword)
}

#[test]
fn vcvtps2ph_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2PH, operand1: Some(Direct(XMM28)), operand2: Some(Direct(YMM16)), operand3: Some(Literal8(44)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 131, 125, 171, 29, 196, 44], OperandSize::Qword)
}

#[test]
fn vcvtps2ph_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2PH, operand1: Some(IndirectScaledIndexedDisplaced(RSI, RDI, Four, 1218345749, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(YMM31)), operand3: Some(Literal8(58)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 99, 125, 40, 29, 188, 190, 21, 123, 158, 72, 58], OperandSize::Qword)
}

#[test]
fn vcvtps2ph_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2PH, operand1: Some(Direct(YMM5)), operand2: Some(Direct(ZMM0)), operand3: Some(Literal8(42)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K5), broadcast: None }, &[98, 243, 125, 157, 29, 197, 42], OperandSize::Dword)
}

#[test]
fn vcvtps2ph_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2PH, operand1: Some(IndirectScaledDisplaced(ECX, Two, 1273491672, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(ZMM7)), operand3: Some(Literal8(9)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 243, 125, 72, 29, 60, 77, 216, 240, 231, 75, 9], OperandSize::Dword)
}

#[test]
fn vcvtps2ph_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2PH, operand1: Some(Direct(YMM28)), operand2: Some(Direct(ZMM5)), operand3: Some(Literal8(73)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K7), broadcast: None }, &[98, 147, 125, 159, 29, 236, 73], OperandSize::Qword)
}

#[test]
fn vcvtps2ph_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2PH, operand1: Some(Indirect(RSI, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(ZMM1)), operand3: Some(Literal8(87)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 243, 125, 72, 29, 14, 87], OperandSize::Qword)
}

