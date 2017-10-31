use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vmovdqu16_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU16, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 255, 142, 111, 196], OperandSize::Dword)
}

#[test]
fn vmovdqu16_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU16, operand1: Some(Direct(XMM0)), operand2: Some(IndirectScaledDisplaced(EDI, Eight, 66105615, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 255, 137, 111, 4, 253, 15, 177, 240, 3], OperandSize::Dword)
}

#[test]
fn vmovdqu16_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU16, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM23)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 177, 255, 142, 111, 215], OperandSize::Qword)
}

#[test]
fn vmovdqu16_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU16, operand1: Some(Direct(XMM24)), operand2: Some(IndirectScaledIndexed(RSI, RDI, Four, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 97, 255, 142, 111, 4, 190], OperandSize::Qword)
}

#[test]
fn vmovdqu16_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU16, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 255, 171, 111, 219], OperandSize::Dword)
}

#[test]
fn vmovdqu16_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU16, operand1: Some(Direct(YMM5)), operand2: Some(Indirect(EDI, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 255, 169, 111, 47], OperandSize::Dword)
}

#[test]
fn vmovdqu16_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU16, operand1: Some(Direct(YMM23)), operand2: Some(Direct(YMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 225, 255, 169, 111, 249], OperandSize::Qword)
}

#[test]
fn vmovdqu16_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU16, operand1: Some(Direct(YMM17)), operand2: Some(IndirectScaledIndexed(RSI, RBX, Two, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 225, 255, 171, 111, 12, 94], OperandSize::Qword)
}

#[test]
fn vmovdqu16_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU16, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 255, 204, 111, 223], OperandSize::Dword)
}

#[test]
fn vmovdqu16_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU16, operand1: Some(Direct(ZMM0)), operand2: Some(IndirectScaledIndexedDisplaced(ESI, EAX, Two, 1304480709, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 255, 206, 111, 132, 70, 197, 203, 192, 77], OperandSize::Dword)
}

#[test]
fn vmovdqu16_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU16, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 255, 205, 111, 239], OperandSize::Qword)
}

#[test]
fn vmovdqu16_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU16, operand1: Some(Direct(ZMM3)), operand2: Some(IndirectScaledIndexed(RAX, RBX, Two, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 255, 204, 111, 28, 88], OperandSize::Qword)
}

#[test]
fn vmovdqu16_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU16, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 255, 138, 111, 231], OperandSize::Dword)
}

#[test]
fn vmovdqu16_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU16, operand1: Some(IndirectDisplaced(ECX, 1930498483, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 255, 8, 127, 177, 179, 17, 17, 115], OperandSize::Dword)
}

#[test]
fn vmovdqu16_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU16, operand1: Some(Direct(XMM21)), operand2: Some(Direct(XMM28)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 129, 255, 139, 111, 236], OperandSize::Qword)
}

#[test]
fn vmovdqu16_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU16, operand1: Some(IndirectScaledIndexed(RSI, RCX, Four, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM11)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 113, 255, 8, 127, 28, 142], OperandSize::Qword)
}

#[test]
fn vmovdqu16_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU16, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 255, 171, 111, 198], OperandSize::Dword)
}

#[test]
fn vmovdqu16_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU16, operand1: Some(IndirectScaledIndexed(ESI, EDX, Eight, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(YMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 255, 40, 127, 44, 214], OperandSize::Dword)
}

#[test]
fn vmovdqu16_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU16, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM25)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 145, 255, 174, 111, 201], OperandSize::Qword)
}

#[test]
fn vmovdqu16_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU16, operand1: Some(IndirectScaledDisplaced(RCX, Two, 524068126, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(YMM8)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 113, 255, 40, 127, 4, 77, 30, 165, 60, 31], OperandSize::Qword)
}

#[test]
fn vmovdqu16_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU16, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 255, 204, 111, 237], OperandSize::Dword)
}

#[test]
fn vmovdqu16_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU16, operand1: Some(IndirectDisplaced(ESI, 350600582, Some(OperandSize::Zmmword), None)), operand2: Some(Direct(ZMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 255, 72, 127, 166, 134, 189, 229, 20], OperandSize::Dword)
}

#[test]
fn vmovdqu16_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU16, operand1: Some(Direct(ZMM21)), operand2: Some(Direct(ZMM12)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 193, 255, 201, 111, 236], OperandSize::Qword)
}

#[test]
fn vmovdqu16_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU16, operand1: Some(IndirectScaledDisplaced(RSI, Eight, 1178507150, Some(OperandSize::Zmmword), None)), operand2: Some(Direct(ZMM16)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 225, 255, 72, 127, 4, 245, 142, 151, 62, 70], OperandSize::Qword)
}

