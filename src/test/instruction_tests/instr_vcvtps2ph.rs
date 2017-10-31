use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vcvtps2ph_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2PH, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM2)), operand3: Some(Literal8(45)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 121, 29, 210, 45], OperandSize::Dword)
}

#[test]
fn vcvtps2ph_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2PH, operand1: Some(IndirectScaledIndexedDisplaced(EAX, ECX, Eight, 315790836, Some(OperandSize::Qword), None)), operand2: Some(Direct(XMM1)), operand3: Some(Literal8(104)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 121, 29, 140, 200, 244, 149, 210, 18, 104], OperandSize::Dword)
}

#[test]
fn vcvtps2ph_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2PH, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM0)), operand3: Some(Literal8(82)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 121, 29, 194, 82], OperandSize::Qword)
}

#[test]
fn vcvtps2ph_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2PH, operand1: Some(IndirectScaledIndexed(RCX, RDI, Eight, Some(OperandSize::Qword), None)), operand2: Some(Direct(XMM2)), operand3: Some(Literal8(121)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 121, 29, 20, 249, 121], OperandSize::Qword)
}

#[test]
fn vcvtps2ph_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2PH, operand1: Some(Direct(XMM6)), operand2: Some(Direct(YMM4)), operand3: Some(Literal8(127)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 125, 29, 230, 127], OperandSize::Dword)
}

#[test]
fn vcvtps2ph_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2PH, operand1: Some(IndirectScaledIndexed(ECX, EAX, Eight, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(YMM3)), operand3: Some(Literal8(27)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 125, 29, 28, 193, 27], OperandSize::Dword)
}

#[test]
fn vcvtps2ph_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2PH, operand1: Some(Direct(XMM0)), operand2: Some(Direct(YMM0)), operand3: Some(Literal8(12)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 125, 29, 192, 12], OperandSize::Qword)
}

#[test]
fn vcvtps2ph_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2PH, operand1: Some(IndirectScaledDisplaced(RDX, Four, 2019231562, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(YMM7)), operand3: Some(Literal8(123)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 125, 29, 60, 149, 74, 7, 91, 120, 123], OperandSize::Qword)
}

#[test]
fn vcvtps2ph_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2PH, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM2)), operand3: Some(Literal8(52)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 243, 125, 140, 29, 215, 52], OperandSize::Dword)
}

#[test]
fn vcvtps2ph_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2PH, operand1: Some(IndirectDisplaced(EAX, 2009236881, Some(OperandSize::Qword), None)), operand2: Some(Direct(XMM3)), operand3: Some(Literal8(125)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 121, 29, 152, 145, 133, 194, 119, 125], OperandSize::Dword)
}

#[test]
fn vcvtps2ph_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2PH, operand1: Some(Direct(XMM8)), operand2: Some(Direct(XMM10)), operand3: Some(Literal8(66)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 83, 125, 137, 29, 208, 66], OperandSize::Qword)
}

#[test]
fn vcvtps2ph_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2PH, operand1: Some(IndirectScaledIndexed(RDX, RDX, Eight, Some(OperandSize::Qword), None)), operand2: Some(Direct(XMM24)), operand3: Some(Literal8(23)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 99, 125, 8, 29, 4, 210, 23], OperandSize::Qword)
}

#[test]
fn vcvtps2ph_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2PH, operand1: Some(Direct(XMM4)), operand2: Some(Direct(YMM7)), operand3: Some(Literal8(52)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 243, 125, 172, 29, 252, 52], OperandSize::Dword)
}

#[test]
fn vcvtps2ph_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2PH, operand1: Some(IndirectDisplaced(EAX, 645489345, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(YMM3)), operand3: Some(Literal8(116)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 125, 29, 152, 193, 98, 121, 38, 116], OperandSize::Dword)
}

#[test]
fn vcvtps2ph_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2PH, operand1: Some(Direct(XMM6)), operand2: Some(Direct(YMM16)), operand3: Some(Literal8(7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 227, 125, 171, 29, 198, 7], OperandSize::Qword)
}

#[test]
fn vcvtps2ph_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2PH, operand1: Some(IndirectScaledDisplaced(RSI, Four, 763276228, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(YMM13)), operand3: Some(Literal8(110)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 99, 125, 29, 44, 181, 196, 171, 126, 45, 110], OperandSize::Qword)
}

#[test]
fn vcvtps2ph_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2PH, operand1: Some(Direct(YMM5)), operand2: Some(Direct(ZMM1)), operand3: Some(Literal8(123)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K1), broadcast: None }, &[98, 243, 125, 153, 29, 205, 123], OperandSize::Dword)
}

#[test]
fn vcvtps2ph_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2PH, operand1: Some(IndirectScaledIndexed(EBX, EBX, Two, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(ZMM7)), operand3: Some(Literal8(74)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 243, 125, 72, 29, 60, 91, 74], OperandSize::Dword)
}

#[test]
fn vcvtps2ph_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2PH, operand1: Some(Direct(YMM11)), operand2: Some(Direct(ZMM30)), operand3: Some(Literal8(50)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K1), broadcast: None }, &[98, 67, 125, 153, 29, 243, 50], OperandSize::Qword)
}

#[test]
fn vcvtps2ph_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2PH, operand1: Some(IndirectScaledDisplaced(RDI, Four, 443229040, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(ZMM4)), operand3: Some(Literal8(29)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 243, 125, 72, 29, 36, 189, 112, 35, 107, 26, 29], OperandSize::Qword)
}

