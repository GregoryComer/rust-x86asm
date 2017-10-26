use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vmovdqu32_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU32, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 126, 141, 111, 240], OperandSize::Dword)
}

#[test]
fn vmovdqu32_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU32, operand1: Some(Direct(XMM5)), operand2: Some(IndirectScaledIndexed(EBX, EBX, Two, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 126, 143, 111, 44, 91], OperandSize::Dword)
}

#[test]
fn vmovdqu32_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU32, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM8)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 209, 126, 139, 111, 216], OperandSize::Qword)
}

#[test]
fn vmovdqu32_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU32, operand1: Some(Direct(XMM30)), operand2: Some(IndirectScaledIndexed(RDI, RDI, Four, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 97, 126, 140, 111, 52, 191], OperandSize::Qword)
}

#[test]
fn vmovdqu32_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU32, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 126, 175, 111, 227], OperandSize::Dword)
}

#[test]
fn vmovdqu32_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU32, operand1: Some(Direct(YMM0)), operand2: Some(IndirectScaledIndexedDisplaced(EDX, ECX, Eight, 1760472377, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 126, 171, 111, 132, 202, 57, 173, 238, 104], OperandSize::Dword)
}

#[test]
fn vmovdqu32_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU32, operand1: Some(Direct(YMM28)), operand2: Some(Direct(YMM12)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 65, 126, 173, 111, 228], OperandSize::Qword)
}

#[test]
fn vmovdqu32_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU32, operand1: Some(Direct(YMM2)), operand2: Some(IndirectScaledIndexedDisplaced(RDI, RSI, Four, 811710477, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 126, 170, 111, 148, 183, 13, 184, 97, 48], OperandSize::Qword)
}

#[test]
fn vmovdqu32_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU32, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 126, 206, 111, 216], OperandSize::Dword)
}

#[test]
fn vmovdqu32_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU32, operand1: Some(Direct(ZMM5)), operand2: Some(IndirectScaledIndexed(EAX, ECX, Eight, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 126, 203, 111, 44, 200], OperandSize::Dword)
}

#[test]
fn vmovdqu32_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU32, operand1: Some(Direct(ZMM29)), operand2: Some(Direct(ZMM25)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 1, 126, 202, 111, 233], OperandSize::Qword)
}

#[test]
fn vmovdqu32_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU32, operand1: Some(Direct(ZMM4)), operand2: Some(IndirectScaledIndexed(RDI, RDX, Four, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 126, 206, 111, 36, 151], OperandSize::Qword)
}

#[test]
fn vmovdqu32_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU32, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 126, 142, 111, 255], OperandSize::Dword)
}

#[test]
fn vmovdqu32_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU32, operand1: Some(IndirectScaledIndexedDisplaced(EBX, EAX, Eight, 1591048873, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 126, 8, 127, 164, 195, 169, 122, 213, 94], OperandSize::Dword)
}

#[test]
fn vmovdqu32_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU32, operand1: Some(Direct(XMM8)), operand2: Some(Direct(XMM21)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 49, 126, 137, 111, 197], OperandSize::Qword)
}

#[test]
fn vmovdqu32_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU32, operand1: Some(IndirectDisplaced(RBX, 222805642, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM28)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 97, 126, 8, 127, 163, 138, 190, 71, 13], OperandSize::Qword)
}

#[test]
fn vmovdqu32_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU32, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 126, 174, 111, 217], OperandSize::Dword)
}

#[test]
fn vmovdqu32_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU32, operand1: Some(Indirect(EBX, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(YMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 126, 40, 127, 27], OperandSize::Dword)
}

#[test]
fn vmovdqu32_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU32, operand1: Some(Direct(YMM11)), operand2: Some(Direct(YMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 113, 126, 171, 111, 222], OperandSize::Qword)
}

#[test]
fn vmovdqu32_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU32, operand1: Some(IndirectScaledIndexed(RDI, RAX, Four, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(YMM22)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 225, 126, 40, 127, 52, 135], OperandSize::Qword)
}

#[test]
fn vmovdqu32_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU32, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 126, 201, 111, 237], OperandSize::Dword)
}

#[test]
fn vmovdqu32_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU32, operand1: Some(IndirectScaledDisplaced(EDX, Four, 609691239, Some(OperandSize::Zmmword), None)), operand2: Some(Direct(ZMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 126, 72, 127, 44, 149, 103, 38, 87, 36], OperandSize::Dword)
}

#[test]
fn vmovdqu32_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU32, operand1: Some(Direct(ZMM25)), operand2: Some(Direct(ZMM24)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 1, 126, 201, 111, 200], OperandSize::Qword)
}

#[test]
fn vmovdqu32_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU32, operand1: Some(IndirectScaledDisplaced(RSI, Two, 36569264, Some(OperandSize::Zmmword), None)), operand2: Some(Direct(ZMM21)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 225, 126, 72, 127, 44, 117, 176, 0, 46, 2], OperandSize::Qword)
}

