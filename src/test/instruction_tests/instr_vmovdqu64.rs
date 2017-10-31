use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vmovdqu64_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU64, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 254, 138, 111, 231], OperandSize::Dword)
}

#[test]
fn vmovdqu64_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU64, operand1: Some(Direct(XMM0)), operand2: Some(IndirectScaledIndexedDisplaced(ESI, EBX, Eight, 591832571, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 254, 142, 111, 132, 222, 251, 165, 70, 35], OperandSize::Dword)
}

#[test]
fn vmovdqu64_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU64, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM29)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 145, 254, 138, 111, 205], OperandSize::Qword)
}

#[test]
fn vmovdqu64_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU64, operand1: Some(Direct(XMM20)), operand2: Some(IndirectScaledDisplaced(RBX, Eight, 1001245345, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 225, 254, 141, 111, 36, 221, 161, 202, 173, 59], OperandSize::Qword)
}

#[test]
fn vmovdqu64_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU64, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 254, 174, 111, 235], OperandSize::Dword)
}

#[test]
fn vmovdqu64_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU64, operand1: Some(Direct(YMM3)), operand2: Some(IndirectScaledIndexed(EDI, EAX, Four, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 254, 173, 111, 28, 135], OperandSize::Dword)
}

#[test]
fn vmovdqu64_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU64, operand1: Some(Direct(YMM13)), operand2: Some(Direct(YMM14)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 81, 254, 169, 111, 238], OperandSize::Qword)
}

#[test]
fn vmovdqu64_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU64, operand1: Some(Direct(YMM9)), operand2: Some(IndirectScaledIndexed(RSI, RDI, Eight, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 113, 254, 175, 111, 12, 254], OperandSize::Qword)
}

#[test]
fn vmovdqu64_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU64, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 254, 206, 111, 209], OperandSize::Dword)
}

#[test]
fn vmovdqu64_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU64, operand1: Some(Direct(ZMM1)), operand2: Some(IndirectDisplaced(EAX, 1685421984, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 254, 202, 111, 136, 160, 127, 117, 100], OperandSize::Dword)
}

#[test]
fn vmovdqu64_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU64, operand1: Some(Direct(ZMM28)), operand2: Some(Direct(ZMM12)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 65, 254, 204, 111, 228], OperandSize::Qword)
}

#[test]
fn vmovdqu64_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU64, operand1: Some(Direct(ZMM12)), operand2: Some(IndirectScaledDisplaced(RCX, Eight, 1840191231, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 113, 254, 202, 111, 36, 205, 255, 22, 175, 109], OperandSize::Qword)
}

#[test]
fn vmovdqu64_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU64, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 254, 142, 111, 211], OperandSize::Dword)
}

#[test]
fn vmovdqu64_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU64, operand1: Some(IndirectScaledIndexedDisplaced(ECX, ESI, Four, 1025149989, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 254, 8, 127, 148, 177, 37, 140, 26, 61], OperandSize::Dword)
}

#[test]
fn vmovdqu64_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU64, operand1: Some(Direct(XMM26)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 97, 254, 137, 111, 210], OperandSize::Qword)
}

#[test]
fn vmovdqu64_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU64, operand1: Some(IndirectScaledDisplaced(RDX, Eight, 1064686277, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM19)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 225, 254, 8, 127, 28, 213, 197, 210, 117, 63], OperandSize::Qword)
}

#[test]
fn vmovdqu64_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU64, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 254, 171, 111, 216], OperandSize::Dword)
}

#[test]
fn vmovdqu64_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU64, operand1: Some(IndirectScaledIndexedDisplaced(EDI, ECX, Eight, 696688991, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(YMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 254, 40, 127, 156, 207, 95, 161, 134, 41], OperandSize::Dword)
}

#[test]
fn vmovdqu64_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU64, operand1: Some(Direct(YMM11)), operand2: Some(Direct(YMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 113, 254, 170, 111, 219], OperandSize::Qword)
}

#[test]
fn vmovdqu64_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU64, operand1: Some(IndirectScaledIndexed(RDX, RDX, Four, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(YMM9)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 113, 254, 40, 127, 12, 146], OperandSize::Qword)
}

#[test]
fn vmovdqu64_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU64, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 254, 206, 111, 221], OperandSize::Dword)
}

#[test]
fn vmovdqu64_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU64, operand1: Some(Indirect(EAX, Some(OperandSize::Zmmword), None)), operand2: Some(Direct(ZMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 254, 72, 127, 40], OperandSize::Dword)
}

#[test]
fn vmovdqu64_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU64, operand1: Some(Direct(ZMM12)), operand2: Some(Direct(ZMM29)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 17, 254, 201, 111, 229], OperandSize::Qword)
}

#[test]
fn vmovdqu64_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU64, operand1: Some(Indirect(RCX, Some(OperandSize::Zmmword), None)), operand2: Some(Direct(ZMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 254, 72, 127, 49], OperandSize::Qword)
}

