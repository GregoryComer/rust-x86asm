use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn valignd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VALIGND, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM4)), operand4: Some(Literal8(94)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 243, 101, 140, 3, 228, 94], OperandSize::Dword)
}

#[test]
fn valignd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VALIGND, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledIndexed(EDX, EDI, Two, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(1)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 243, 109, 141, 3, 20, 122, 1], OperandSize::Dword)
}

#[test]
fn valignd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VALIGND, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledDisplaced(ESI, Four, 549735401, Some(OperandSize::Dword), None)), operand4: Some(Literal8(58)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 243, 109, 157, 3, 20, 181, 233, 75, 196, 32, 58], OperandSize::Dword)
}

#[test]
fn valignd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VALIGND, operand1: Some(Direct(XMM17)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM20)), operand4: Some(Literal8(72)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 163, 101, 139, 3, 204, 72], OperandSize::Qword)
}

#[test]
fn valignd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VALIGND, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM19)), operand3: Some(Indirect(RDX, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(119)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 243, 101, 134, 3, 10, 119], OperandSize::Qword)
}

#[test]
fn valignd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VALIGND, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM27)), operand3: Some(Indirect(RCX, Some(OperandSize::Dword), None)), operand4: Some(Literal8(100)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 243, 37, 150, 3, 25, 100], OperandSize::Qword)
}

#[test]
fn valignd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VALIGND, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM7)), operand4: Some(Literal8(99)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 243, 109, 169, 3, 215, 99], OperandSize::Dword)
}

#[test]
fn valignd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VALIGND, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectScaledIndexedDisplaced(EDI, ECX, Eight, 477456382, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(15)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 243, 85, 173, 3, 156, 207, 254, 103, 117, 28, 15], OperandSize::Dword)
}

#[test]
fn valignd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VALIGND, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM7)), operand3: Some(Indirect(ECX, Some(OperandSize::Dword), None)), operand4: Some(Literal8(21)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 243, 69, 187, 3, 1, 21], OperandSize::Dword)
}

#[test]
fn valignd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VALIGND, operand1: Some(Direct(YMM11)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(YMM7)), operand4: Some(Literal8(101)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 115, 125, 169, 3, 223, 101], OperandSize::Qword)
}

#[test]
fn valignd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VALIGND, operand1: Some(Direct(YMM8)), operand2: Some(Direct(YMM17)), operand3: Some(IndirectScaledIndexedDisplaced(RSI, RAX, Four, 1334094971, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(9)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 115, 117, 163, 3, 132, 134, 123, 172, 132, 79, 9], OperandSize::Qword)
}

#[test]
fn valignd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VALIGND, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM28)), operand3: Some(IndirectScaledIndexedDisplaced(RDX, RBX, Eight, 2002108678, Some(OperandSize::Dword), None)), operand4: Some(Literal8(52)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 243, 29, 179, 3, 156, 218, 6, 193, 85, 119, 52], OperandSize::Qword)
}

#[test]
fn valignd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VALIGND, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM6)), operand3: Some(Direct(ZMM1)), operand4: Some(Literal8(10)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 243, 77, 203, 3, 201, 10], OperandSize::Dword)
}

#[test]
fn valignd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VALIGND, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM3)), operand3: Some(IndirectScaledIndexedDisplaced(EBX, EAX, Two, 277092900, Some(OperandSize::Zmmword), None)), operand4: Some(Literal8(94)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 243, 101, 204, 3, 156, 67, 36, 26, 132, 16, 94], OperandSize::Dword)
}

#[test]
fn valignd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VALIGND, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM7)), operand3: Some(IndirectScaledIndexedDisplaced(ESI, EBX, Two, 893206137, Some(OperandSize::Dword), None)), operand4: Some(Literal8(0)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 243, 69, 218, 3, 132, 94, 121, 62, 61, 53, 0], OperandSize::Dword)
}

#[test]
fn valignd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VALIGND, operand1: Some(Direct(ZMM20)), operand2: Some(Direct(ZMM18)), operand3: Some(Direct(ZMM23)), operand4: Some(Literal8(24)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 163, 109, 198, 3, 231, 24], OperandSize::Qword)
}

#[test]
fn valignd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VALIGND, operand1: Some(Direct(ZMM8)), operand2: Some(Direct(ZMM7)), operand3: Some(IndirectScaledIndexed(RSI, RBX, Four, Some(OperandSize::Zmmword), None)), operand4: Some(Literal8(24)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 115, 69, 201, 3, 4, 158, 24], OperandSize::Qword)
}

#[test]
fn valignd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VALIGND, operand1: Some(Direct(ZMM15)), operand2: Some(Direct(ZMM27)), operand3: Some(IndirectScaledDisplaced(RCX, Two, 615966883, Some(OperandSize::Dword), None)), operand4: Some(Literal8(18)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 115, 37, 215, 3, 60, 77, 163, 232, 182, 36, 18], OperandSize::Qword)
}

