use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vmovshdup_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSHDUP, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 250, 22, 225], OperandSize::Dword)
}

#[test]
fn vmovshdup_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSHDUP, operand1: Some(Direct(XMM0)), operand2: Some(Indirect(EDX, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 250, 22, 2], OperandSize::Dword)
}

#[test]
fn vmovshdup_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSHDUP, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 250, 22, 236], OperandSize::Qword)
}

#[test]
fn vmovshdup_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSHDUP, operand1: Some(Direct(XMM3)), operand2: Some(IndirectScaledIndexedDisplaced(RDI, RDX, Eight, 1514787312, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 250, 22, 156, 215, 240, 209, 73, 90], OperandSize::Qword)
}

#[test]
fn vmovshdup_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSHDUP, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 254, 22, 221], OperandSize::Dword)
}

#[test]
fn vmovshdup_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSHDUP, operand1: Some(Direct(YMM3)), operand2: Some(IndirectDisplaced(EAX, 767075352, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 254, 22, 152, 24, 164, 184, 45], OperandSize::Dword)
}

#[test]
fn vmovshdup_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSHDUP, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 254, 22, 211], OperandSize::Qword)
}

#[test]
fn vmovshdup_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSHDUP, operand1: Some(Direct(YMM3)), operand2: Some(IndirectScaledDisplaced(RCX, Four, 598398357, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 254, 22, 28, 141, 149, 213, 170, 35], OperandSize::Qword)
}

#[test]
fn vmovshdup_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSHDUP, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 126, 143, 22, 247], OperandSize::Dword)
}

#[test]
fn vmovshdup_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSHDUP, operand1: Some(Direct(XMM6)), operand2: Some(Indirect(EAX, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 126, 142, 22, 48], OperandSize::Dword)
}

#[test]
fn vmovshdup_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSHDUP, operand1: Some(Direct(XMM8)), operand2: Some(Direct(XMM20)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 49, 126, 138, 22, 196], OperandSize::Qword)
}

#[test]
fn vmovshdup_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSHDUP, operand1: Some(Direct(XMM18)), operand2: Some(Indirect(RAX, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 225, 126, 138, 22, 16], OperandSize::Qword)
}

#[test]
fn vmovshdup_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSHDUP, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 126, 174, 22, 202], OperandSize::Dword)
}

#[test]
fn vmovshdup_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSHDUP, operand1: Some(Direct(YMM1)), operand2: Some(IndirectScaledDisplaced(EDX, Four, 1554088353, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 126, 169, 22, 12, 149, 161, 129, 161, 92], OperandSize::Dword)
}

#[test]
fn vmovshdup_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSHDUP, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 126, 169, 22, 220], OperandSize::Qword)
}

#[test]
fn vmovshdup_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSHDUP, operand1: Some(Direct(YMM0)), operand2: Some(IndirectScaledIndexed(RBX, RDI, Eight, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 126, 171, 22, 4, 251], OperandSize::Qword)
}

#[test]
fn vmovshdup_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSHDUP, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 126, 202, 22, 222], OperandSize::Dword)
}

#[test]
fn vmovshdup_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSHDUP, operand1: Some(Direct(ZMM4)), operand2: Some(IndirectScaledDisplaced(ESI, Two, 837551866, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 126, 202, 22, 36, 117, 250, 6, 236, 49], OperandSize::Dword)
}

#[test]
fn vmovshdup_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSHDUP, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 126, 206, 22, 220], OperandSize::Qword)
}

#[test]
fn vmovshdup_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSHDUP, operand1: Some(Direct(ZMM29)), operand2: Some(IndirectScaledIndexed(RDI, RBX, Two, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 97, 126, 203, 22, 44, 95], OperandSize::Qword)
}

