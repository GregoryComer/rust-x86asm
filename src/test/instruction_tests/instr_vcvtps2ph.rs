use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vcvtps2ph_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2PH, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM7)), operand3: Some(Literal8(48)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 121, 29, 250, 48], OperandSize::Dword)
}

#[test]
fn vcvtps2ph_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2PH, operand1: Some(Indirect(ESI, Some(OperandSize::Qword), None)), operand2: Some(Direct(XMM5)), operand3: Some(Literal8(78)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 121, 29, 46, 78], OperandSize::Dword)
}

#[test]
fn vcvtps2ph_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2PH, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM6)), operand3: Some(Literal8(95)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 121, 29, 245, 95], OperandSize::Qword)
}

#[test]
fn vcvtps2ph_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2PH, operand1: Some(IndirectScaledIndexedDisplaced(RDI, RSI, Four, 232452886, Some(OperandSize::Qword), None)), operand2: Some(Direct(XMM5)), operand3: Some(Literal8(96)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 121, 29, 172, 183, 22, 243, 218, 13, 96], OperandSize::Qword)
}

#[test]
fn vcvtps2ph_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2PH, operand1: Some(Direct(XMM2)), operand2: Some(Direct(YMM1)), operand3: Some(Literal8(60)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 125, 29, 202, 60], OperandSize::Dword)
}

#[test]
fn vcvtps2ph_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2PH, operand1: Some(IndirectScaledIndexed(EDI, EDI, Two, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(YMM4)), operand3: Some(Literal8(31)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 125, 29, 36, 127, 31], OperandSize::Dword)
}

#[test]
fn vcvtps2ph_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2PH, operand1: Some(Direct(XMM6)), operand2: Some(Direct(YMM4)), operand3: Some(Literal8(33)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 125, 29, 230, 33], OperandSize::Qword)
}

#[test]
fn vcvtps2ph_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2PH, operand1: Some(IndirectScaledIndexed(RBX, RCX, Four, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(YMM0)), operand3: Some(Literal8(71)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 125, 29, 4, 139, 71], OperandSize::Qword)
}

#[test]
fn vcvtps2ph_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2PH, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM0)), operand3: Some(Literal8(8)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 243, 125, 141, 29, 192, 8], OperandSize::Dword)
}

#[test]
fn vcvtps2ph_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2PH, operand1: Some(Indirect(ESI, Some(OperandSize::Qword), None)), operand2: Some(Direct(XMM1)), operand3: Some(Literal8(47)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 121, 29, 14, 47], OperandSize::Dword)
}

#[test]
fn vcvtps2ph_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2PH, operand1: Some(Direct(XMM31)), operand2: Some(Direct(XMM26)), operand3: Some(Literal8(117)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 3, 125, 137, 29, 215, 117], OperandSize::Qword)
}

#[test]
fn vcvtps2ph_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2PH, operand1: Some(IndirectScaledIndexed(RSI, RSI, Eight, Some(OperandSize::Qword), None)), operand2: Some(Direct(XMM23)), operand3: Some(Literal8(48)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 227, 125, 8, 29, 60, 246, 48], OperandSize::Qword)
}

#[test]
fn vcvtps2ph_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2PH, operand1: Some(Direct(XMM7)), operand2: Some(Direct(YMM5)), operand3: Some(Literal8(86)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 243, 125, 172, 29, 239, 86], OperandSize::Dword)
}

#[test]
fn vcvtps2ph_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2PH, operand1: Some(IndirectScaledIndexed(EDI, ESI, Four, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(YMM4)), operand3: Some(Literal8(6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 125, 29, 36, 183, 6], OperandSize::Dword)
}

#[test]
fn vcvtps2ph_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2PH, operand1: Some(Direct(XMM0)), operand2: Some(Direct(YMM5)), operand3: Some(Literal8(51)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 243, 125, 173, 29, 232, 51], OperandSize::Qword)
}

#[test]
fn vcvtps2ph_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2PH, operand1: Some(IndirectDisplaced(RSI, 1071108643, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(YMM9)), operand3: Some(Literal8(25)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 99, 125, 29, 142, 35, 210, 215, 63, 25], OperandSize::Qword)
}

#[test]
fn vcvtps2ph_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2PH, operand1: Some(Direct(YMM7)), operand2: Some(Direct(ZMM4)), operand3: Some(Literal8(97)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K4), broadcast: None }, &[98, 243, 125, 156, 29, 231, 97], OperandSize::Dword)
}

#[test]
fn vcvtps2ph_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2PH, operand1: Some(IndirectScaledDisplaced(ECX, Eight, 494928908, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(ZMM4)), operand3: Some(Literal8(20)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 243, 125, 72, 29, 36, 205, 12, 4, 128, 29, 20], OperandSize::Dword)
}

#[test]
fn vcvtps2ph_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2PH, operand1: Some(Direct(YMM7)), operand2: Some(Direct(ZMM8)), operand3: Some(Literal8(103)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K5), broadcast: None }, &[98, 115, 125, 157, 29, 199, 103], OperandSize::Qword)
}

#[test]
fn vcvtps2ph_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2PH, operand1: Some(IndirectDisplaced(RAX, 1695753557, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(ZMM27)), operand3: Some(Literal8(63)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 99, 125, 72, 29, 152, 85, 37, 19, 101, 63], OperandSize::Qword)
}

