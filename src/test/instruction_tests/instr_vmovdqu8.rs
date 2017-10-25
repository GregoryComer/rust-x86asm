use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vmovdqu8_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU8, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 127, 142, 111, 224], OperandSize::Dword)
}

#[test]
fn vmovdqu8_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU8, operand1: Some(Direct(XMM0)), operand2: Some(IndirectScaledDisplaced(EAX, Two, 87632457, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 127, 143, 111, 4, 69, 73, 42, 57, 5], OperandSize::Dword)
}

#[test]
fn vmovdqu8_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU8, operand1: Some(Direct(XMM12)), operand2: Some(Direct(XMM30)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 17, 127, 143, 111, 230], OperandSize::Qword)
}

#[test]
fn vmovdqu8_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU8, operand1: Some(Direct(XMM4)), operand2: Some(IndirectScaledIndexed(RCX, RSI, Two, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 127, 139, 111, 36, 113], OperandSize::Qword)
}

#[test]
fn vmovdqu8_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU8, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 127, 170, 111, 221], OperandSize::Dword)
}

#[test]
fn vmovdqu8_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU8, operand1: Some(Direct(YMM3)), operand2: Some(Indirect(EDI, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 127, 169, 111, 31], OperandSize::Dword)
}

#[test]
fn vmovdqu8_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU8, operand1: Some(Direct(YMM21)), operand2: Some(Direct(YMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 225, 127, 170, 111, 236], OperandSize::Qword)
}

#[test]
fn vmovdqu8_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU8, operand1: Some(Direct(YMM27)), operand2: Some(IndirectDisplaced(RDI, 1088704711, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 97, 127, 173, 111, 159, 199, 80, 228, 64], OperandSize::Qword)
}

#[test]
fn vmovdqu8_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU8, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 127, 207, 111, 247], OperandSize::Dword)
}

#[test]
fn vmovdqu8_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU8, operand1: Some(Direct(ZMM7)), operand2: Some(IndirectScaledIndexed(EBX, ESI, Eight, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 127, 205, 111, 60, 243], OperandSize::Dword)
}

#[test]
fn vmovdqu8_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU8, operand1: Some(Direct(ZMM18)), operand2: Some(Direct(ZMM26)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 129, 127, 207, 111, 210], OperandSize::Qword)
}

#[test]
fn vmovdqu8_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU8, operand1: Some(Direct(ZMM27)), operand2: Some(IndirectDisplaced(RSI, 1552542946, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 97, 127, 206, 111, 158, 226, 236, 137, 92], OperandSize::Qword)
}

#[test]
fn vmovdqu8_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU8, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 127, 137, 111, 194], OperandSize::Dword)
}

#[test]
fn vmovdqu8_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU8, operand1: Some(Indirect(EDI, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 127, 8, 127, 31], OperandSize::Dword)
}

#[test]
fn vmovdqu8_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU8, operand1: Some(Direct(XMM20)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 225, 127, 137, 111, 224], OperandSize::Qword)
}

#[test]
fn vmovdqu8_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU8, operand1: Some(IndirectScaledIndexed(RAX, RCX, Four, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM21)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 225, 127, 8, 127, 44, 136], OperandSize::Qword)
}

#[test]
fn vmovdqu8_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU8, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 127, 175, 111, 204], OperandSize::Dword)
}

#[test]
fn vmovdqu8_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU8, operand1: Some(IndirectDisplaced(ESI, 1758073141, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(YMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 127, 40, 127, 190, 53, 17, 202, 104], OperandSize::Dword)
}

#[test]
fn vmovdqu8_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU8, operand1: Some(Direct(YMM18)), operand2: Some(Direct(YMM28)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 129, 127, 169, 111, 212], OperandSize::Qword)
}

#[test]
fn vmovdqu8_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU8, operand1: Some(IndirectDisplaced(RAX, 1730664475, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(YMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 127, 40, 127, 136, 27, 216, 39, 103], OperandSize::Qword)
}

#[test]
fn vmovdqu8_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU8, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 127, 201, 111, 212], OperandSize::Dword)
}

#[test]
fn vmovdqu8_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU8, operand1: Some(IndirectScaledIndexed(EAX, ECX, Two, Some(OperandSize::Zmmword), None)), operand2: Some(Direct(ZMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 127, 72, 127, 12, 72], OperandSize::Dword)
}

#[test]
fn vmovdqu8_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU8, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM16)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 177, 127, 201, 111, 208], OperandSize::Qword)
}

#[test]
fn vmovdqu8_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU8, operand1: Some(IndirectScaledDisplaced(RDI, Four, 1599398888, Some(OperandSize::Zmmword), None)), operand2: Some(Direct(ZMM22)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 225, 127, 72, 127, 52, 189, 232, 227, 84, 95], OperandSize::Qword)
}

