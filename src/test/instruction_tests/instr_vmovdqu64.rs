use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vmovdqu64_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU64, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 254, 137, 111, 218], OperandSize::Dword)
}

#[test]
fn vmovdqu64_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU64, operand1: Some(Direct(XMM5)), operand2: Some(IndirectScaledIndexed(EDX, EBX, Four, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 254, 142, 111, 44, 154], OperandSize::Dword)
}

#[test]
fn vmovdqu64_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU64, operand1: Some(Direct(XMM26)), operand2: Some(Direct(XMM27)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 1, 254, 143, 111, 211], OperandSize::Qword)
}

#[test]
fn vmovdqu64_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU64, operand1: Some(Direct(XMM28)), operand2: Some(IndirectScaledIndexed(RDX, RSI, Eight, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 97, 254, 140, 111, 36, 242], OperandSize::Qword)
}

#[test]
fn vmovdqu64_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU64, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 254, 169, 111, 232], OperandSize::Dword)
}

#[test]
fn vmovdqu64_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU64, operand1: Some(Direct(YMM2)), operand2: Some(IndirectScaledIndexed(ESI, EAX, Four, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 254, 170, 111, 20, 134], OperandSize::Dword)
}

#[test]
fn vmovdqu64_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU64, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM25)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 145, 254, 175, 111, 225], OperandSize::Qword)
}

#[test]
fn vmovdqu64_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU64, operand1: Some(Direct(YMM4)), operand2: Some(IndirectScaledIndexed(RSI, RBX, Eight, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 254, 171, 111, 36, 222], OperandSize::Qword)
}

#[test]
fn vmovdqu64_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU64, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 254, 201, 111, 226], OperandSize::Dword)
}

#[test]
fn vmovdqu64_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU64, operand1: Some(Direct(ZMM4)), operand2: Some(IndirectScaledIndexedDisplaced(EBX, EAX, Four, 537743376, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 254, 201, 111, 164, 131, 16, 80, 13, 32], OperandSize::Dword)
}

#[test]
fn vmovdqu64_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU64, operand1: Some(Direct(ZMM18)), operand2: Some(Direct(ZMM30)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 129, 254, 204, 111, 214], OperandSize::Qword)
}

#[test]
fn vmovdqu64_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU64, operand1: Some(Direct(ZMM17)), operand2: Some(Indirect(RBX, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 225, 254, 202, 111, 11], OperandSize::Qword)
}

#[test]
fn vmovdqu64_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU64, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 254, 137, 111, 242], OperandSize::Dword)
}

#[test]
fn vmovdqu64_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU64, operand1: Some(Indirect(EAX, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 254, 8, 127, 56], OperandSize::Dword)
}

#[test]
fn vmovdqu64_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU64, operand1: Some(Direct(XMM14)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 113, 254, 142, 111, 245], OperandSize::Qword)
}

#[test]
fn vmovdqu64_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU64, operand1: Some(Indirect(RAX, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 254, 8, 127, 32], OperandSize::Qword)
}

#[test]
fn vmovdqu64_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU64, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 254, 173, 111, 227], OperandSize::Dword)
}

#[test]
fn vmovdqu64_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU64, operand1: Some(Indirect(EDX, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(YMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 254, 40, 127, 42], OperandSize::Dword)
}

#[test]
fn vmovdqu64_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU64, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM16)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 177, 254, 170, 111, 224], OperandSize::Qword)
}

#[test]
fn vmovdqu64_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU64, operand1: Some(Indirect(RCX, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(YMM22)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 225, 254, 40, 127, 49], OperandSize::Qword)
}

#[test]
fn vmovdqu64_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU64, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 254, 207, 111, 242], OperandSize::Dword)
}

#[test]
fn vmovdqu64_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU64, operand1: Some(IndirectScaledDisplaced(EAX, Four, 1532648155, Some(OperandSize::Zmmword), None)), operand2: Some(Direct(ZMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 254, 72, 127, 60, 133, 219, 90, 90, 91], OperandSize::Dword)
}

#[test]
fn vmovdqu64_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU64, operand1: Some(Direct(ZMM31)), operand2: Some(Direct(ZMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 97, 254, 201, 111, 252], OperandSize::Qword)
}

#[test]
fn vmovdqu64_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU64, operand1: Some(IndirectDisplaced(RSI, 837806057, Some(OperandSize::Zmmword), None)), operand2: Some(Direct(ZMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 254, 72, 127, 150, 233, 231, 239, 49], OperandSize::Qword)
}

