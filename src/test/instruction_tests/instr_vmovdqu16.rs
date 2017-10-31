use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vmovdqu16_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU16, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 255, 137, 111, 210], OperandSize::Dword)
}

#[test]
fn vmovdqu16_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU16, operand1: Some(Direct(XMM2)), operand2: Some(IndirectScaledDisplaced(ESI, Four, 405380796, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 255, 143, 111, 20, 181, 188, 158, 41, 24], OperandSize::Dword)
}

#[test]
fn vmovdqu16_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU16, operand1: Some(Direct(XMM24)), operand2: Some(Direct(XMM12)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 65, 255, 142, 111, 196], OperandSize::Qword)
}

#[test]
fn vmovdqu16_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU16, operand1: Some(Direct(XMM16)), operand2: Some(IndirectScaledIndexed(RCX, RBX, Eight, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 225, 255, 138, 111, 4, 217], OperandSize::Qword)
}

#[test]
fn vmovdqu16_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU16, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 255, 171, 111, 252], OperandSize::Dword)
}

#[test]
fn vmovdqu16_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU16, operand1: Some(Direct(YMM0)), operand2: Some(Indirect(EDI, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 255, 174, 111, 7], OperandSize::Dword)
}

#[test]
fn vmovdqu16_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU16, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 255, 175, 111, 217], OperandSize::Qword)
}

#[test]
fn vmovdqu16_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU16, operand1: Some(Direct(YMM24)), operand2: Some(IndirectScaledIndexedDisplaced(RCX, RCX, Two, 37970010, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 97, 255, 170, 111, 132, 73, 90, 96, 67, 2], OperandSize::Qword)
}

#[test]
fn vmovdqu16_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU16, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 255, 205, 111, 220], OperandSize::Dword)
}

#[test]
fn vmovdqu16_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU16, operand1: Some(Direct(ZMM4)), operand2: Some(IndirectDisplaced(ESI, 327800609, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 255, 207, 111, 166, 33, 215, 137, 19], OperandSize::Dword)
}

#[test]
fn vmovdqu16_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU16, operand1: Some(Direct(ZMM12)), operand2: Some(Direct(ZMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 113, 255, 204, 111, 229], OperandSize::Qword)
}

#[test]
fn vmovdqu16_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU16, operand1: Some(Direct(ZMM5)), operand2: Some(IndirectScaledIndexed(RDI, RBX, Four, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 255, 205, 111, 44, 159], OperandSize::Qword)
}

#[test]
fn vmovdqu16_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU16, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 255, 143, 111, 204], OperandSize::Dword)
}

#[test]
fn vmovdqu16_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU16, operand1: Some(IndirectScaledDisplaced(EAX, Two, 1617338213, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 255, 8, 127, 60, 69, 101, 159, 102, 96], OperandSize::Dword)
}

#[test]
fn vmovdqu16_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU16, operand1: Some(Direct(XMM15)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 113, 255, 143, 111, 254], OperandSize::Qword)
}

#[test]
fn vmovdqu16_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU16, operand1: Some(IndirectScaledIndexedDisplaced(RCX, RBX, Four, 1494935494, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 255, 8, 127, 132, 153, 198, 231, 26, 89], OperandSize::Qword)
}

#[test]
fn vmovdqu16_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU16, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 255, 174, 111, 192], OperandSize::Dword)
}

#[test]
fn vmovdqu16_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU16, operand1: Some(IndirectScaledDisplaced(EDI, Four, 999227303, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(YMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 255, 40, 127, 44, 189, 167, 255, 142, 59], OperandSize::Dword)
}

#[test]
fn vmovdqu16_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU16, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM23)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 177, 255, 170, 111, 199], OperandSize::Qword)
}

#[test]
fn vmovdqu16_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU16, operand1: Some(Indirect(RSI, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(YMM9)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 113, 255, 40, 127, 14], OperandSize::Qword)
}

#[test]
fn vmovdqu16_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU16, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 255, 207, 111, 214], OperandSize::Dword)
}

#[test]
fn vmovdqu16_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU16, operand1: Some(Indirect(ECX, Some(OperandSize::Zmmword), None)), operand2: Some(Direct(ZMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 255, 72, 127, 49], OperandSize::Dword)
}

#[test]
fn vmovdqu16_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU16, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM21)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 177, 255, 203, 111, 237], OperandSize::Qword)
}

#[test]
fn vmovdqu16_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU16, operand1: Some(IndirectScaledIndexed(RDI, RBX, Eight, Some(OperandSize::Zmmword), None)), operand2: Some(Direct(ZMM9)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 113, 255, 72, 127, 12, 223], OperandSize::Qword)
}

