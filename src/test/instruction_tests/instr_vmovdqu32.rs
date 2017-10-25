use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vmovdqu32_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU32, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 126, 141, 111, 218], OperandSize::Dword)
}

#[test]
fn vmovdqu32_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU32, operand1: Some(Direct(XMM0)), operand2: Some(Indirect(EAX, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 126, 138, 111, 0], OperandSize::Dword)
}

#[test]
fn vmovdqu32_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU32, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 126, 139, 111, 224], OperandSize::Qword)
}

#[test]
fn vmovdqu32_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU32, operand1: Some(Direct(XMM14)), operand2: Some(IndirectScaledDisplaced(RAX, Two, 1449014587, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 113, 126, 138, 111, 52, 69, 59, 53, 94, 86], OperandSize::Qword)
}

#[test]
fn vmovdqu32_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU32, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 126, 169, 111, 210], OperandSize::Dword)
}

#[test]
fn vmovdqu32_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU32, operand1: Some(Direct(YMM6)), operand2: Some(IndirectScaledIndexed(EDI, EBX, Eight, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 126, 174, 111, 52, 223], OperandSize::Dword)
}

#[test]
fn vmovdqu32_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU32, operand1: Some(Direct(YMM21)), operand2: Some(Direct(YMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 225, 126, 172, 111, 239], OperandSize::Qword)
}

#[test]
fn vmovdqu32_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU32, operand1: Some(Direct(YMM15)), operand2: Some(Indirect(RBX, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 113, 126, 169, 111, 59], OperandSize::Qword)
}

#[test]
fn vmovdqu32_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU32, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 126, 206, 111, 233], OperandSize::Dword)
}

#[test]
fn vmovdqu32_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU32, operand1: Some(Direct(ZMM5)), operand2: Some(Indirect(EDX, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 126, 202, 111, 42], OperandSize::Dword)
}

#[test]
fn vmovdqu32_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU32, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM30)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 145, 126, 202, 111, 246], OperandSize::Qword)
}

#[test]
fn vmovdqu32_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU32, operand1: Some(Direct(ZMM30)), operand2: Some(IndirectScaledIndexedDisplaced(RDX, RCX, Two, 1930605276, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 97, 126, 202, 111, 180, 74, 220, 178, 18, 115], OperandSize::Qword)
}

#[test]
fn vmovdqu32_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU32, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 126, 141, 111, 227], OperandSize::Dword)
}

#[test]
fn vmovdqu32_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU32, operand1: Some(IndirectScaledIndexedDisplaced(ESI, EDX, Four, 1623147040, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 126, 8, 127, 172, 150, 32, 66, 191, 96], OperandSize::Dword)
}

#[test]
fn vmovdqu32_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU32, operand1: Some(Direct(XMM21)), operand2: Some(Direct(XMM26)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 129, 126, 137, 111, 234], OperandSize::Qword)
}

#[test]
fn vmovdqu32_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU32, operand1: Some(Indirect(RDX, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM23)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 225, 126, 8, 127, 58], OperandSize::Qword)
}

#[test]
fn vmovdqu32_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU32, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 126, 175, 111, 206], OperandSize::Dword)
}

#[test]
fn vmovdqu32_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU32, operand1: Some(Indirect(EBX, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(YMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 126, 40, 127, 59], OperandSize::Dword)
}

#[test]
fn vmovdqu32_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU32, operand1: Some(Direct(YMM9)), operand2: Some(Direct(YMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 113, 126, 171, 111, 206], OperandSize::Qword)
}

#[test]
fn vmovdqu32_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU32, operand1: Some(Indirect(RCX, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(YMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 126, 40, 127, 57], OperandSize::Qword)
}

#[test]
fn vmovdqu32_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU32, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 126, 204, 111, 246], OperandSize::Dword)
}

#[test]
fn vmovdqu32_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU32, operand1: Some(IndirectScaledDisplaced(ESI, Two, 304776441, Some(OperandSize::Zmmword), None)), operand2: Some(Direct(ZMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 126, 72, 127, 20, 117, 249, 132, 42, 18], OperandSize::Dword)
}

#[test]
fn vmovdqu32_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU32, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 126, 206, 111, 229], OperandSize::Qword)
}

#[test]
fn vmovdqu32_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU32, operand1: Some(IndirectScaledIndexedDisplaced(RDI, RBX, Four, 1635767611, Some(OperandSize::Zmmword), None)), operand2: Some(Direct(ZMM26)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 97, 126, 72, 127, 148, 159, 59, 213, 127, 97], OperandSize::Qword)
}

