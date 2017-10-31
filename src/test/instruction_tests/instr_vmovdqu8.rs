use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vmovdqu8_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU8, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 127, 143, 111, 251], OperandSize::Dword)
}

#[test]
fn vmovdqu8_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU8, operand1: Some(Direct(XMM3)), operand2: Some(Indirect(EBX, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 127, 141, 111, 27], OperandSize::Dword)
}

#[test]
fn vmovdqu8_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU8, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM15)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 209, 127, 138, 111, 199], OperandSize::Qword)
}

#[test]
fn vmovdqu8_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU8, operand1: Some(Direct(XMM2)), operand2: Some(IndirectScaledDisplaced(RSI, Four, 1555639245, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 127, 140, 111, 20, 181, 205, 43, 185, 92], OperandSize::Qword)
}

#[test]
fn vmovdqu8_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU8, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 127, 170, 111, 241], OperandSize::Dword)
}

#[test]
fn vmovdqu8_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU8, operand1: Some(Direct(YMM5)), operand2: Some(IndirectScaledDisplaced(EDX, Two, 702767076, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 127, 171, 111, 44, 85, 228, 95, 227, 41], OperandSize::Dword)
}

#[test]
fn vmovdqu8_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU8, operand1: Some(Direct(YMM9)), operand2: Some(Direct(YMM14)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 81, 127, 174, 111, 206], OperandSize::Qword)
}

#[test]
fn vmovdqu8_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU8, operand1: Some(Direct(YMM10)), operand2: Some(IndirectScaledIndexed(RBX, RBX, Eight, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 113, 127, 169, 111, 20, 219], OperandSize::Qword)
}

#[test]
fn vmovdqu8_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU8, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 127, 203, 111, 246], OperandSize::Dword)
}

#[test]
fn vmovdqu8_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU8, operand1: Some(Direct(ZMM1)), operand2: Some(IndirectScaledIndexedDisplaced(EAX, ECX, Eight, 469266177, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 127, 203, 111, 140, 200, 1, 111, 248, 27], OperandSize::Dword)
}

#[test]
fn vmovdqu8_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU8, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM30)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 145, 127, 207, 111, 246], OperandSize::Qword)
}

#[test]
fn vmovdqu8_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU8, operand1: Some(Direct(ZMM24)), operand2: Some(IndirectScaledIndexed(RAX, RDI, Four, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 97, 127, 204, 111, 4, 184], OperandSize::Qword)
}

#[test]
fn vmovdqu8_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU8, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 127, 141, 111, 215], OperandSize::Dword)
}

#[test]
fn vmovdqu8_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU8, operand1: Some(IndirectDisplaced(EAX, 974577127, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 127, 8, 127, 152, 231, 221, 22, 58], OperandSize::Dword)
}

#[test]
fn vmovdqu8_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU8, operand1: Some(Direct(XMM16)), operand2: Some(Direct(XMM19)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 161, 127, 142, 111, 195], OperandSize::Qword)
}

#[test]
fn vmovdqu8_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU8, operand1: Some(IndirectScaledIndexed(RAX, RDI, Four, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 127, 8, 127, 28, 184], OperandSize::Qword)
}

#[test]
fn vmovdqu8_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU8, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 127, 172, 111, 199], OperandSize::Dword)
}

#[test]
fn vmovdqu8_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU8, operand1: Some(IndirectScaledDisplaced(EDX, Eight, 1609707198, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(YMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 127, 40, 127, 52, 213, 190, 46, 242, 95], OperandSize::Dword)
}

#[test]
fn vmovdqu8_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU8, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM14)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 209, 127, 169, 111, 222], OperandSize::Qword)
}

#[test]
fn vmovdqu8_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU8, operand1: Some(IndirectScaledIndexed(RDI, RDX, Two, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(YMM24)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 97, 127, 40, 127, 4, 87], OperandSize::Qword)
}

#[test]
fn vmovdqu8_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU8, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 127, 201, 111, 248], OperandSize::Dword)
}

#[test]
fn vmovdqu8_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU8, operand1: Some(IndirectScaledIndexedDisplaced(EDI, ESI, Two, 2092527770, Some(OperandSize::Zmmword), None)), operand2: Some(Direct(ZMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 127, 72, 127, 140, 119, 154, 112, 185, 124], OperandSize::Dword)
}

#[test]
fn vmovdqu8_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU8, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 127, 206, 111, 244], OperandSize::Qword)
}

#[test]
fn vmovdqu8_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU8, operand1: Some(Indirect(RDX, Some(OperandSize::Zmmword), None)), operand2: Some(Direct(ZMM16)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 225, 127, 72, 127, 2], OperandSize::Qword)
}

