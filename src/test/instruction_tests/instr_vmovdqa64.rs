use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vmovdqa64_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQA64, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 253, 139, 111, 215], OperandSize::Dword)
}

#[test]
fn vmovdqa64_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQA64, operand1: Some(Direct(XMM6)), operand2: Some(IndirectScaledIndexed(EAX, ESI, Two, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 253, 137, 111, 52, 112], OperandSize::Dword)
}

#[test]
fn vmovdqa64_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQA64, operand1: Some(Direct(XMM10)), operand2: Some(Direct(XMM13)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 81, 253, 138, 111, 213], OperandSize::Qword)
}

#[test]
fn vmovdqa64_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQA64, operand1: Some(Direct(XMM7)), operand2: Some(IndirectScaledDisplaced(RDX, Eight, 835151586, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 253, 140, 111, 60, 213, 226, 102, 199, 49], OperandSize::Qword)
}

#[test]
fn vmovdqa64_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQA64, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 253, 172, 111, 194], OperandSize::Dword)
}

#[test]
fn vmovdqa64_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQA64, operand1: Some(Direct(YMM7)), operand2: Some(IndirectScaledDisplaced(EDI, Eight, 505251752, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 253, 174, 111, 60, 253, 168, 135, 29, 30], OperandSize::Dword)
}

#[test]
fn vmovdqa64_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQA64, operand1: Some(Direct(YMM14)), operand2: Some(Direct(YMM13)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 81, 253, 175, 111, 245], OperandSize::Qword)
}

#[test]
fn vmovdqa64_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQA64, operand1: Some(Direct(YMM3)), operand2: Some(IndirectScaledIndexedDisplaced(RDI, RDI, Two, 1202891321, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 253, 172, 111, 156, 127, 57, 170, 178, 71], OperandSize::Qword)
}

#[test]
fn vmovdqa64_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQA64, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 253, 206, 111, 241], OperandSize::Dword)
}

#[test]
fn vmovdqa64_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQA64, operand1: Some(Direct(ZMM7)), operand2: Some(IndirectScaledIndexed(ECX, EAX, Eight, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 253, 206, 111, 60, 193], OperandSize::Dword)
}

#[test]
fn vmovdqa64_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQA64, operand1: Some(Direct(ZMM25)), operand2: Some(Direct(ZMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 97, 253, 204, 111, 205], OperandSize::Qword)
}

#[test]
fn vmovdqa64_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQA64, operand1: Some(Direct(ZMM14)), operand2: Some(IndirectScaledDisplaced(RBX, Four, 1902431642, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 113, 253, 205, 111, 52, 157, 154, 205, 100, 113], OperandSize::Qword)
}

#[test]
fn vmovdqa64_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQA64, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 253, 138, 111, 200], OperandSize::Dword)
}

#[test]
fn vmovdqa64_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQA64, operand1: Some(IndirectScaledIndexed(ESI, EAX, Two, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 253, 8, 127, 52, 70], OperandSize::Dword)
}

#[test]
fn vmovdqa64_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQA64, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM25)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 145, 253, 137, 111, 233], OperandSize::Qword)
}

#[test]
fn vmovdqa64_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQA64, operand1: Some(IndirectScaledIndexed(RSI, RDI, Two, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM14)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 113, 253, 8, 127, 52, 126], OperandSize::Qword)
}

#[test]
fn vmovdqa64_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQA64, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 253, 174, 111, 237], OperandSize::Dword)
}

#[test]
fn vmovdqa64_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQA64, operand1: Some(IndirectDisplaced(ECX, 337548174, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(YMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 253, 40, 127, 145, 142, 147, 30, 20], OperandSize::Dword)
}

#[test]
fn vmovdqa64_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQA64, operand1: Some(Direct(YMM28)), operand2: Some(Direct(YMM17)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 33, 253, 172, 111, 225], OperandSize::Qword)
}

#[test]
fn vmovdqa64_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQA64, operand1: Some(IndirectDisplaced(RCX, 961932012, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(YMM19)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 225, 253, 40, 127, 153, 236, 234, 85, 57], OperandSize::Qword)
}

#[test]
fn vmovdqa64_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQA64, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 253, 206, 111, 228], OperandSize::Dword)
}

#[test]
fn vmovdqa64_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQA64, operand1: Some(IndirectScaledIndexedDisplaced(EDI, EDI, Four, 1794937155, Some(OperandSize::Zmmword), None)), operand2: Some(Direct(ZMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 253, 72, 127, 156, 191, 67, 145, 252, 106], OperandSize::Dword)
}

#[test]
fn vmovdqa64_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQA64, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM23)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 177, 253, 202, 111, 255], OperandSize::Qword)
}

#[test]
fn vmovdqa64_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQA64, operand1: Some(Indirect(RCX, Some(OperandSize::Zmmword), None)), operand2: Some(Direct(ZMM8)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 113, 253, 72, 127, 1], OperandSize::Qword)
}

