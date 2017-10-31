use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vmovdqu8_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU8, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 127, 140, 111, 202], OperandSize::Dword)
}

#[test]
fn vmovdqu8_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU8, operand1: Some(Direct(XMM5)), operand2: Some(IndirectDisplaced(ESI, 1677745240, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 127, 140, 111, 174, 88, 92, 0, 100], OperandSize::Dword)
}

#[test]
fn vmovdqu8_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU8, operand1: Some(Direct(XMM31)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 97, 127, 138, 111, 253], OperandSize::Qword)
}

#[test]
fn vmovdqu8_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU8, operand1: Some(Direct(XMM0)), operand2: Some(IndirectScaledIndexed(RCX, RDX, Two, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 127, 140, 111, 4, 81], OperandSize::Qword)
}

#[test]
fn vmovdqu8_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU8, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 127, 172, 111, 207], OperandSize::Dword)
}

#[test]
fn vmovdqu8_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU8, operand1: Some(Direct(YMM1)), operand2: Some(Indirect(EDX, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 127, 175, 111, 10], OperandSize::Dword)
}

#[test]
fn vmovdqu8_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU8, operand1: Some(Direct(YMM30)), operand2: Some(Direct(YMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 97, 127, 172, 111, 245], OperandSize::Qword)
}

#[test]
fn vmovdqu8_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU8, operand1: Some(Direct(YMM4)), operand2: Some(Indirect(RSI, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 127, 174, 111, 38], OperandSize::Qword)
}

#[test]
fn vmovdqu8_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU8, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 127, 201, 111, 220], OperandSize::Dword)
}

#[test]
fn vmovdqu8_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU8, operand1: Some(Direct(ZMM3)), operand2: Some(Indirect(ECX, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 127, 206, 111, 25], OperandSize::Dword)
}

#[test]
fn vmovdqu8_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU8, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 127, 202, 111, 248], OperandSize::Qword)
}

#[test]
fn vmovdqu8_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU8, operand1: Some(Direct(ZMM11)), operand2: Some(Indirect(RCX, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 113, 127, 205, 111, 25], OperandSize::Qword)
}

#[test]
fn vmovdqu8_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU8, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 127, 141, 111, 245], OperandSize::Dword)
}

#[test]
fn vmovdqu8_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU8, operand1: Some(IndirectDisplaced(EDX, 1480894320, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 127, 8, 127, 170, 112, 167, 68, 88], OperandSize::Dword)
}

#[test]
fn vmovdqu8_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU8, operand1: Some(Direct(XMM17)), operand2: Some(Direct(XMM19)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 161, 127, 143, 111, 203], OperandSize::Qword)
}

#[test]
fn vmovdqu8_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU8, operand1: Some(IndirectScaledIndexed(RSI, RAX, Four, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM26)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 97, 127, 8, 127, 20, 134], OperandSize::Qword)
}

#[test]
fn vmovdqu8_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU8, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 127, 174, 111, 213], OperandSize::Dword)
}

#[test]
fn vmovdqu8_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU8, operand1: Some(IndirectScaledIndexed(EBX, EAX, Eight, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(YMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 127, 40, 127, 4, 195], OperandSize::Dword)
}

#[test]
fn vmovdqu8_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU8, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM23)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 177, 127, 169, 111, 255], OperandSize::Qword)
}

#[test]
fn vmovdqu8_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU8, operand1: Some(IndirectScaledDisplaced(RCX, Two, 115230090, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(YMM27)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 97, 127, 40, 127, 28, 77, 138, 69, 222, 6], OperandSize::Qword)
}

#[test]
fn vmovdqu8_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU8, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 127, 206, 111, 207], OperandSize::Dword)
}

#[test]
fn vmovdqu8_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU8, operand1: Some(IndirectScaledDisplaced(ECX, Four, 1633032807, Some(OperandSize::Zmmword), None)), operand2: Some(Direct(ZMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 127, 72, 127, 44, 141, 103, 26, 86, 97], OperandSize::Dword)
}

#[test]
fn vmovdqu8_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU8, operand1: Some(Direct(ZMM31)), operand2: Some(Direct(ZMM22)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 33, 127, 205, 111, 254], OperandSize::Qword)
}

#[test]
fn vmovdqu8_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU8, operand1: Some(IndirectScaledIndexed(RCX, RDX, Four, Some(OperandSize::Zmmword), None)), operand2: Some(Direct(ZMM25)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 97, 127, 72, 127, 12, 145], OperandSize::Qword)
}

