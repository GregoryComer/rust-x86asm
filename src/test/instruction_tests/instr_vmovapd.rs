use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vmovapd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 40, 194], OperandSize::Dword)
}

#[test]
fn vmovapd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPD, operand1: Some(Direct(XMM4)), operand2: Some(Indirect(EAX, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 40, 32], OperandSize::Dword)
}

#[test]
fn vmovapd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 40, 209], OperandSize::Qword)
}

#[test]
fn vmovapd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPD, operand1: Some(Direct(XMM6)), operand2: Some(IndirectScaledIndexed(RCX, RBX, Two, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 40, 52, 89], OperandSize::Qword)
}

#[test]
fn vmovapd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPD, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 40, 241], OperandSize::Dword)
}

#[test]
fn vmovapd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPD, operand1: Some(Direct(YMM0)), operand2: Some(IndirectDisplaced(EAX, 1989723424, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 40, 128, 32, 197, 152, 118], OperandSize::Dword)
}

#[test]
fn vmovapd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPD, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 40, 231], OperandSize::Qword)
}

#[test]
fn vmovapd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPD, operand1: Some(Direct(YMM5)), operand2: Some(IndirectDisplaced(RDX, 1782069363, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 40, 170, 115, 56, 56, 106], OperandSize::Qword)
}

#[test]
fn vmovapd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 253, 137, 40, 232], OperandSize::Dword)
}

#[test]
fn vmovapd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPD, operand1: Some(Direct(XMM3)), operand2: Some(IndirectScaledIndexed(ECX, EBX, Eight, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 253, 140, 40, 28, 217], OperandSize::Dword)
}

#[test]
fn vmovapd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 253, 143, 40, 219], OperandSize::Qword)
}

#[test]
fn vmovapd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPD, operand1: Some(Direct(XMM21)), operand2: Some(Indirect(RDI, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 225, 253, 138, 40, 47], OperandSize::Qword)
}

#[test]
fn vmovapd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 253, 169, 40, 232], OperandSize::Dword)
}

#[test]
fn vmovapd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPD, operand1: Some(Direct(YMM1)), operand2: Some(IndirectScaledDisplaced(EDX, Four, 1273285772, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 253, 172, 40, 12, 149, 140, 204, 228, 75], OperandSize::Dword)
}

#[test]
fn vmovapd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPD, operand1: Some(Direct(YMM22)), operand2: Some(Direct(YMM15)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 193, 253, 175, 40, 247], OperandSize::Qword)
}

#[test]
fn vmovapd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPD, operand1: Some(Direct(YMM7)), operand2: Some(Indirect(RDI, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 253, 173, 40, 63], OperandSize::Qword)
}

#[test]
fn vmovapd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPD, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 253, 205, 40, 252], OperandSize::Dword)
}

#[test]
fn vmovapd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPD, operand1: Some(Direct(ZMM5)), operand2: Some(IndirectScaledDisplaced(ECX, Two, 1597829976, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 253, 203, 40, 44, 77, 88, 243, 60, 95], OperandSize::Dword)
}

#[test]
fn vmovapd_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPD, operand1: Some(Direct(ZMM20)), operand2: Some(Direct(ZMM8)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 193, 253, 204, 40, 224], OperandSize::Qword)
}

#[test]
fn vmovapd_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPD, operand1: Some(Direct(ZMM22)), operand2: Some(IndirectScaledDisplaced(RDI, Two, 1249731476, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 225, 253, 202, 40, 52, 125, 148, 99, 125, 74], OperandSize::Qword)
}

#[test]
fn vmovapd_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 40, 245], OperandSize::Dword)
}

#[test]
fn vmovapd_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPD, operand1: Some(IndirectScaledIndexedDisplaced(EDX, ESI, Eight, 401978658, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 41, 164, 242, 34, 181, 245, 23], OperandSize::Dword)
}

#[test]
fn vmovapd_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 40, 195], OperandSize::Qword)
}

#[test]
fn vmovapd_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPD, operand1: Some(IndirectScaledDisplaced(RSI, Two, 414697860, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 41, 44, 117, 132, 201, 183, 24], OperandSize::Qword)
}

#[test]
fn vmovapd_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPD, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 40, 195], OperandSize::Dword)
}

#[test]
fn vmovapd_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPD, operand1: Some(IndirectDisplaced(ESI, 65318145, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(YMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 41, 166, 1, 173, 228, 3], OperandSize::Dword)
}

#[test]
fn vmovapd_27() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPD, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 40, 225], OperandSize::Qword)
}

#[test]
fn vmovapd_28() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPD, operand1: Some(IndirectScaledDisplaced(RBX, Four, 1261575364, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(YMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 41, 60, 157, 196, 28, 50, 75], OperandSize::Qword)
}

#[test]
fn vmovapd_29() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 253, 137, 40, 246], OperandSize::Dword)
}

#[test]
fn vmovapd_30() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPD, operand1: Some(Indirect(EDX, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 41, 2], OperandSize::Dword)
}

#[test]
fn vmovapd_31() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPD, operand1: Some(Direct(XMM14)), operand2: Some(Direct(XMM11)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 81, 253, 142, 40, 243], OperandSize::Qword)
}

#[test]
fn vmovapd_32() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPD, operand1: Some(Indirect(RSI, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM27)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 97, 253, 8, 41, 30], OperandSize::Qword)
}

#[test]
fn vmovapd_33() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 253, 170, 40, 248], OperandSize::Dword)
}

#[test]
fn vmovapd_34() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPD, operand1: Some(Indirect(EBX, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(YMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 41, 3], OperandSize::Dword)
}

#[test]
fn vmovapd_35() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPD, operand1: Some(Direct(YMM13)), operand2: Some(Direct(YMM18)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 49, 253, 169, 40, 234], OperandSize::Qword)
}

#[test]
fn vmovapd_36() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPD, operand1: Some(Indirect(RAX, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(YMM8)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 125, 41, 0], OperandSize::Qword)
}

#[test]
fn vmovapd_37() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPD, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 253, 202, 40, 253], OperandSize::Dword)
}

#[test]
fn vmovapd_38() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPD, operand1: Some(IndirectDisplaced(ESI, 1190488486, Some(OperandSize::Zmmword), None)), operand2: Some(Direct(ZMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 253, 72, 41, 166, 166, 105, 245, 70], OperandSize::Dword)
}

#[test]
fn vmovapd_39() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPD, operand1: Some(Direct(ZMM13)), operand2: Some(Direct(ZMM20)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 49, 253, 206, 40, 236], OperandSize::Qword)
}

#[test]
fn vmovapd_40() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPD, operand1: Some(IndirectDisplaced(RDX, 378995260, Some(OperandSize::Zmmword), None)), operand2: Some(Direct(ZMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 253, 72, 41, 170, 60, 2, 151, 22], OperandSize::Qword)
}

