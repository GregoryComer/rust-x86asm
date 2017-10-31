use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vmovdqa64_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQA64, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 253, 137, 111, 250], OperandSize::Dword)
}

#[test]
fn vmovdqa64_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQA64, operand1: Some(Direct(XMM3)), operand2: Some(Indirect(EBX, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 253, 140, 111, 27], OperandSize::Dword)
}

#[test]
fn vmovdqa64_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQA64, operand1: Some(Direct(XMM14)), operand2: Some(Direct(XMM11)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 81, 253, 143, 111, 243], OperandSize::Qword)
}

#[test]
fn vmovdqa64_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQA64, operand1: Some(Direct(XMM31)), operand2: Some(IndirectScaledDisplaced(RCX, Four, 776159764, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 97, 253, 137, 111, 60, 141, 20, 66, 67, 46], OperandSize::Qword)
}

#[test]
fn vmovdqa64_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQA64, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 253, 170, 111, 248], OperandSize::Dword)
}

#[test]
fn vmovdqa64_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQA64, operand1: Some(Direct(YMM3)), operand2: Some(IndirectDisplaced(ECX, 271948275, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 253, 171, 111, 153, 243, 153, 53, 16], OperandSize::Dword)
}

#[test]
fn vmovdqa64_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQA64, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM18)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 177, 253, 170, 111, 210], OperandSize::Qword)
}

#[test]
fn vmovdqa64_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQA64, operand1: Some(Direct(YMM27)), operand2: Some(IndirectScaledDisplaced(RCX, Four, 1978872842, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 97, 253, 169, 111, 28, 141, 10, 52, 243, 117], OperandSize::Qword)
}

#[test]
fn vmovdqa64_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQA64, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 253, 207, 111, 217], OperandSize::Dword)
}

#[test]
fn vmovdqa64_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQA64, operand1: Some(Direct(ZMM0)), operand2: Some(IndirectDisplaced(ECX, 1466090985, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 253, 204, 111, 129, 233, 197, 98, 87], OperandSize::Dword)
}

#[test]
fn vmovdqa64_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQA64, operand1: Some(Direct(ZMM19)), operand2: Some(Direct(ZMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 225, 253, 203, 111, 219], OperandSize::Qword)
}

#[test]
fn vmovdqa64_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQA64, operand1: Some(Direct(ZMM9)), operand2: Some(IndirectDisplaced(RBX, 1846174866, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 113, 253, 204, 111, 139, 146, 100, 10, 110], OperandSize::Qword)
}

#[test]
fn vmovdqa64_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQA64, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 253, 139, 111, 233], OperandSize::Dword)
}

#[test]
fn vmovdqa64_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQA64, operand1: Some(IndirectDisplaced(EDI, 542838479, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 253, 8, 127, 175, 207, 14, 91, 32], OperandSize::Dword)
}

#[test]
fn vmovdqa64_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQA64, operand1: Some(Direct(XMM20)), operand2: Some(Direct(XMM13)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 193, 253, 139, 111, 229], OperandSize::Qword)
}

#[test]
fn vmovdqa64_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQA64, operand1: Some(Indirect(RDX, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 253, 8, 127, 18], OperandSize::Qword)
}

#[test]
fn vmovdqa64_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQA64, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 253, 171, 111, 203], OperandSize::Dword)
}

#[test]
fn vmovdqa64_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQA64, operand1: Some(IndirectDisplaced(EAX, 2142094322, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(YMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 253, 40, 127, 176, 242, 195, 173, 127], OperandSize::Dword)
}

#[test]
fn vmovdqa64_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQA64, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM22)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 177, 253, 170, 111, 254], OperandSize::Qword)
}

#[test]
fn vmovdqa64_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQA64, operand1: Some(IndirectScaledIndexedDisplaced(RCX, RDI, Two, 1436770679, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(YMM31)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 97, 253, 40, 127, 188, 121, 119, 97, 163, 85], OperandSize::Qword)
}

#[test]
fn vmovdqa64_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQA64, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 253, 206, 111, 202], OperandSize::Dword)
}

#[test]
fn vmovdqa64_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQA64, operand1: Some(IndirectScaledDisplaced(EAX, Four, 749789745, Some(OperandSize::Zmmword), None)), operand2: Some(Direct(ZMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 253, 72, 127, 20, 133, 49, 226, 176, 44], OperandSize::Dword)
}

#[test]
fn vmovdqa64_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQA64, operand1: Some(Direct(ZMM8)), operand2: Some(Direct(ZMM23)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 49, 253, 202, 111, 199], OperandSize::Qword)
}

#[test]
fn vmovdqa64_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQA64, operand1: Some(IndirectScaledDisplaced(RAX, Four, 382339426, Some(OperandSize::Zmmword), None)), operand2: Some(Direct(ZMM23)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 225, 253, 72, 127, 60, 133, 98, 9, 202, 22], OperandSize::Qword)
}

