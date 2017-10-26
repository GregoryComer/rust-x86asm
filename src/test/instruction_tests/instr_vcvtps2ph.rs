use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vcvtps2ph_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2PH, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM4)), operand3: Some(Literal8(85)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 121, 29, 231, 85], OperandSize::Dword)
}

#[test]
fn vcvtps2ph_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2PH, operand1: Some(IndirectScaledDisplaced(EBX, Four, 1370587828, Some(OperandSize::Qword), None)), operand2: Some(Direct(XMM4)), operand3: Some(Literal8(21)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 121, 29, 36, 157, 180, 130, 177, 81, 21], OperandSize::Dword)
}

#[test]
fn vcvtps2ph_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2PH, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM0)), operand3: Some(Literal8(65)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 121, 29, 194, 65], OperandSize::Qword)
}

#[test]
fn vcvtps2ph_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2PH, operand1: Some(IndirectScaledDisplaced(RSI, Eight, 653817736, Some(OperandSize::Qword), None)), operand2: Some(Direct(XMM7)), operand3: Some(Literal8(26)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 121, 29, 60, 245, 136, 119, 248, 38, 26], OperandSize::Qword)
}

#[test]
fn vcvtps2ph_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2PH, operand1: Some(Direct(XMM4)), operand2: Some(Direct(YMM6)), operand3: Some(Literal8(23)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 125, 29, 244, 23], OperandSize::Dword)
}

#[test]
fn vcvtps2ph_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2PH, operand1: Some(IndirectScaledIndexed(EBX, EBX, Eight, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(YMM6)), operand3: Some(Literal8(37)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 125, 29, 52, 219, 37], OperandSize::Dword)
}

#[test]
fn vcvtps2ph_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2PH, operand1: Some(Direct(XMM2)), operand2: Some(Direct(YMM0)), operand3: Some(Literal8(118)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 125, 29, 194, 118], OperandSize::Qword)
}

#[test]
fn vcvtps2ph_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2PH, operand1: Some(IndirectScaledIndexedDisplaced(RSI, RSI, Eight, 1571754207, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(YMM0)), operand3: Some(Literal8(123)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 125, 29, 132, 246, 223, 16, 175, 93, 123], OperandSize::Qword)
}

#[test]
fn vcvtps2ph_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2PH, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM0)), operand3: Some(Literal8(34)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 243, 125, 143, 29, 193, 34], OperandSize::Dword)
}

#[test]
fn vcvtps2ph_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2PH, operand1: Some(IndirectScaledDisplaced(EDX, Eight, 1925844786, Some(OperandSize::Qword), None)), operand2: Some(Direct(XMM5)), operand3: Some(Literal8(65)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 121, 29, 44, 213, 50, 15, 202, 114, 65], OperandSize::Dword)
}

#[test]
fn vcvtps2ph_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2PH, operand1: Some(Direct(XMM16)), operand2: Some(Direct(XMM18)), operand3: Some(Literal8(99)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 163, 125, 138, 29, 208, 99], OperandSize::Qword)
}

#[test]
fn vcvtps2ph_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2PH, operand1: Some(IndirectDisplaced(RDI, 466446711, Some(OperandSize::Qword), None)), operand2: Some(Direct(XMM27)), operand3: Some(Literal8(98)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 99, 125, 8, 29, 159, 119, 105, 205, 27, 98], OperandSize::Qword)
}

#[test]
fn vcvtps2ph_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2PH, operand1: Some(Direct(XMM6)), operand2: Some(Direct(YMM5)), operand3: Some(Literal8(13)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 243, 125, 174, 29, 238, 13], OperandSize::Dword)
}

#[test]
fn vcvtps2ph_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2PH, operand1: Some(Indirect(EAX, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(YMM6)), operand3: Some(Literal8(115)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 125, 29, 48, 115], OperandSize::Dword)
}

#[test]
fn vcvtps2ph_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2PH, operand1: Some(Direct(XMM16)), operand2: Some(Direct(YMM20)), operand3: Some(Literal8(104)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 163, 125, 175, 29, 224, 104], OperandSize::Qword)
}

#[test]
fn vcvtps2ph_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2PH, operand1: Some(Indirect(RSI, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(YMM3)), operand3: Some(Literal8(25)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 125, 29, 30, 25], OperandSize::Qword)
}

#[test]
fn vcvtps2ph_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2PH, operand1: Some(Direct(YMM6)), operand2: Some(Direct(ZMM4)), operand3: Some(Literal8(0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K3), broadcast: None }, &[98, 243, 125, 155, 29, 230, 0], OperandSize::Dword)
}

#[test]
fn vcvtps2ph_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2PH, operand1: Some(IndirectDisplaced(EDI, 1049814096, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(ZMM2)), operand3: Some(Literal8(74)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 243, 125, 72, 29, 151, 80, 228, 146, 62, 74], OperandSize::Dword)
}

#[test]
fn vcvtps2ph_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2PH, operand1: Some(Direct(YMM14)), operand2: Some(Direct(ZMM25)), operand3: Some(Literal8(22)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K6), broadcast: None }, &[98, 67, 125, 158, 29, 206, 22], OperandSize::Qword)
}

#[test]
fn vcvtps2ph_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2PH, operand1: Some(IndirectDisplaced(RDI, 1541755878, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(ZMM24)), operand3: Some(Literal8(125)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 99, 125, 72, 29, 135, 230, 83, 229, 91, 125], OperandSize::Qword)
}

