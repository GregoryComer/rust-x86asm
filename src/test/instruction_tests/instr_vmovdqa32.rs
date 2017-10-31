use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vmovdqa32_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQA32, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 125, 140, 111, 217], OperandSize::Dword)
}

#[test]
fn vmovdqa32_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQA32, operand1: Some(Direct(XMM0)), operand2: Some(IndirectScaledIndexed(EBX, EAX, Eight, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 125, 141, 111, 4, 195], OperandSize::Dword)
}

#[test]
fn vmovdqa32_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQA32, operand1: Some(Direct(XMM14)), operand2: Some(Direct(XMM18)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 49, 125, 137, 111, 242], OperandSize::Qword)
}

#[test]
fn vmovdqa32_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQA32, operand1: Some(Direct(XMM7)), operand2: Some(Indirect(RCX, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 125, 143, 111, 57], OperandSize::Qword)
}

#[test]
fn vmovdqa32_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQA32, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 125, 172, 111, 214], OperandSize::Dword)
}

#[test]
fn vmovdqa32_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQA32, operand1: Some(Direct(YMM0)), operand2: Some(IndirectScaledIndexedDisplaced(ESI, ESI, Eight, 648772662, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 125, 170, 111, 132, 246, 54, 124, 171, 38], OperandSize::Dword)
}

#[test]
fn vmovdqa32_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQA32, operand1: Some(Direct(YMM18)), operand2: Some(Direct(YMM9)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 193, 125, 174, 111, 209], OperandSize::Qword)
}

#[test]
fn vmovdqa32_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQA32, operand1: Some(Direct(YMM8)), operand2: Some(Indirect(RDX, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 113, 125, 174, 111, 2], OperandSize::Qword)
}

#[test]
fn vmovdqa32_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQA32, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 125, 202, 111, 206], OperandSize::Dword)
}

#[test]
fn vmovdqa32_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQA32, operand1: Some(Direct(ZMM5)), operand2: Some(IndirectScaledIndexedDisplaced(EDX, EDI, Two, 1626763404, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 125, 206, 111, 172, 122, 140, 112, 246, 96], OperandSize::Dword)
}

#[test]
fn vmovdqa32_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQA32, operand1: Some(Direct(ZMM14)), operand2: Some(Direct(ZMM16)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 49, 125, 201, 111, 240], OperandSize::Qword)
}

#[test]
fn vmovdqa32_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQA32, operand1: Some(Direct(ZMM20)), operand2: Some(Indirect(RBX, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 225, 125, 203, 111, 35], OperandSize::Qword)
}

#[test]
fn vmovdqa32_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQA32, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 125, 141, 111, 203], OperandSize::Dword)
}

#[test]
fn vmovdqa32_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQA32, operand1: Some(Indirect(EBX, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 125, 8, 127, 3], OperandSize::Dword)
}

#[test]
fn vmovdqa32_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQA32, operand1: Some(Direct(XMM11)), operand2: Some(Direct(XMM23)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 49, 125, 139, 111, 223], OperandSize::Qword)
}

#[test]
fn vmovdqa32_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQA32, operand1: Some(IndirectDisplaced(RDX, 1649175466, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 125, 8, 127, 154, 170, 107, 76, 98], OperandSize::Qword)
}

#[test]
fn vmovdqa32_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQA32, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 125, 170, 111, 207], OperandSize::Dword)
}

#[test]
fn vmovdqa32_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQA32, operand1: Some(IndirectScaledDisplaced(EDI, Four, 567680055, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(YMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 125, 40, 127, 28, 189, 55, 28, 214, 33], OperandSize::Dword)
}

#[test]
fn vmovdqa32_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQA32, operand1: Some(Direct(YMM21)), operand2: Some(Direct(YMM17)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 161, 125, 170, 111, 233], OperandSize::Qword)
}

#[test]
fn vmovdqa32_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQA32, operand1: Some(IndirectScaledIndexed(RDI, RDI, Eight, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(YMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 125, 40, 127, 60, 255], OperandSize::Qword)
}

#[test]
fn vmovdqa32_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQA32, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 125, 203, 111, 238], OperandSize::Dword)
}

#[test]
fn vmovdqa32_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQA32, operand1: Some(IndirectScaledIndexedDisplaced(ESI, EDX, Eight, 317093348, Some(OperandSize::Zmmword), None)), operand2: Some(Direct(ZMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 125, 72, 127, 132, 214, 228, 117, 230, 18], OperandSize::Dword)
}

#[test]
fn vmovdqa32_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQA32, operand1: Some(Direct(ZMM9)), operand2: Some(Direct(ZMM14)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 81, 125, 202, 111, 206], OperandSize::Qword)
}

#[test]
fn vmovdqa32_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQA32, operand1: Some(Indirect(RBX, Some(OperandSize::Zmmword), None)), operand2: Some(Direct(ZMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 125, 72, 127, 51], OperandSize::Qword)
}

