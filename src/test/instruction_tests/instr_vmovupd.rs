use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vmovupd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 16, 215], OperandSize::Dword)
}

#[test]
fn vmovupd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPD, operand1: Some(Direct(XMM6)), operand2: Some(IndirectDisplaced(EDI, 1847064716, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 16, 183, 140, 248, 23, 110], OperandSize::Dword)
}

#[test]
fn vmovupd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 16, 247], OperandSize::Qword)
}

#[test]
fn vmovupd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPD, operand1: Some(Direct(XMM5)), operand2: Some(Indirect(RSI, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 16, 46], OperandSize::Qword)
}

#[test]
fn vmovupd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 16, 251], OperandSize::Dword)
}

#[test]
fn vmovupd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPD, operand1: Some(Direct(YMM2)), operand2: Some(IndirectDisplaced(EBX, 1812553966, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 16, 147, 238, 96, 9, 108], OperandSize::Dword)
}

#[test]
fn vmovupd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 16, 250], OperandSize::Qword)
}

#[test]
fn vmovupd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPD, operand1: Some(Direct(YMM0)), operand2: Some(Indirect(RDX, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 16, 2], OperandSize::Qword)
}

#[test]
fn vmovupd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 253, 140, 16, 214], OperandSize::Dword)
}

#[test]
fn vmovupd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPD, operand1: Some(Direct(XMM6)), operand2: Some(IndirectDisplaced(ESI, 1718964687, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 253, 139, 16, 182, 207, 81, 117, 102], OperandSize::Dword)
}

#[test]
fn vmovupd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPD, operand1: Some(Direct(XMM22)), operand2: Some(Direct(XMM23)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 161, 253, 138, 16, 247], OperandSize::Qword)
}

#[test]
fn vmovupd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPD, operand1: Some(Direct(XMM22)), operand2: Some(IndirectDisplaced(RAX, 868831487, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 225, 253, 140, 16, 176, 255, 80, 201, 51], OperandSize::Qword)
}

#[test]
fn vmovupd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 253, 171, 16, 250], OperandSize::Dword)
}

#[test]
fn vmovupd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPD, operand1: Some(Direct(YMM6)), operand2: Some(IndirectScaledIndexedDisplaced(ESI, EAX, Four, 682402586, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 253, 173, 16, 180, 134, 26, 163, 172, 40], OperandSize::Dword)
}

#[test]
fn vmovupd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPD, operand1: Some(Direct(YMM31)), operand2: Some(Direct(YMM10)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 65, 253, 170, 16, 250], OperandSize::Qword)
}

#[test]
fn vmovupd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPD, operand1: Some(Direct(YMM16)), operand2: Some(Indirect(RAX, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 225, 253, 173, 16, 0], OperandSize::Qword)
}

#[test]
fn vmovupd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPD, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 253, 206, 16, 209], OperandSize::Dword)
}

#[test]
fn vmovupd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPD, operand1: Some(Direct(ZMM6)), operand2: Some(IndirectDisplaced(EDI, 1517262120, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 253, 203, 16, 183, 40, 149, 111, 90], OperandSize::Dword)
}

#[test]
fn vmovupd_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPD, operand1: Some(Direct(ZMM10)), operand2: Some(Direct(ZMM22)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 49, 253, 206, 16, 214], OperandSize::Qword)
}

#[test]
fn vmovupd_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPD, operand1: Some(Direct(ZMM9)), operand2: Some(IndirectScaledIndexed(RSI, RDI, Two, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 113, 253, 207, 16, 12, 126], OperandSize::Qword)
}

#[test]
fn vmovupd_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 16, 233], OperandSize::Dword)
}

#[test]
fn vmovupd_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPD, operand1: Some(IndirectDisplaced(ECX, 820161265, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 17, 177, 241, 170, 226, 48], OperandSize::Dword)
}

#[test]
fn vmovupd_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 16, 215], OperandSize::Qword)
}

#[test]
fn vmovupd_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPD, operand1: Some(IndirectDisplaced(RSI, 719110923, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 17, 190, 11, 195, 220, 42], OperandSize::Qword)
}

#[test]
fn vmovupd_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPD, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 16, 226], OperandSize::Dword)
}

#[test]
fn vmovupd_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPD, operand1: Some(IndirectScaledIndexedDisplaced(ESI, ECX, Eight, 1185950313, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(YMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 17, 180, 206, 105, 42, 176, 70], OperandSize::Dword)
}

#[test]
fn vmovupd_27() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPD, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 16, 195], OperandSize::Qword)
}

#[test]
fn vmovupd_28() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPD, operand1: Some(IndirectScaledDisplaced(RSI, Eight, 1874977385, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(YMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 17, 12, 245, 105, 226, 193, 111], OperandSize::Qword)
}

#[test]
fn vmovupd_29() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 253, 141, 16, 229], OperandSize::Dword)
}

#[test]
fn vmovupd_30() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPD, operand1: Some(IndirectScaledIndexedDisplaced(EBX, EDI, Four, 2082169184, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 17, 172, 187, 96, 97, 27, 124], OperandSize::Dword)
}

#[test]
fn vmovupd_31() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPD, operand1: Some(Direct(XMM21)), operand2: Some(Direct(XMM15)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 193, 253, 137, 16, 239], OperandSize::Qword)
}

#[test]
fn vmovupd_32() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPD, operand1: Some(IndirectScaledIndexedDisplaced(RAX, RAX, Four, 866847392, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM18)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 225, 253, 8, 17, 148, 128, 160, 10, 171, 51], OperandSize::Qword)
}

#[test]
fn vmovupd_33() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPD, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 253, 173, 16, 246], OperandSize::Dword)
}

#[test]
fn vmovupd_34() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPD, operand1: Some(IndirectScaledIndexedDisplaced(EAX, EAX, Two, 187152988, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(YMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 17, 132, 64, 92, 186, 39, 11], OperandSize::Dword)
}

#[test]
fn vmovupd_35() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPD, operand1: Some(Direct(YMM26)), operand2: Some(Direct(YMM20)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 33, 253, 171, 16, 212], OperandSize::Qword)
}

#[test]
fn vmovupd_36() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPD, operand1: Some(IndirectScaledIndexedDisplaced(RDI, RCX, Eight, 813913146, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(YMM9)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 125, 17, 140, 207, 58, 84, 131, 48], OperandSize::Qword)
}

#[test]
fn vmovupd_37() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPD, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 253, 207, 16, 194], OperandSize::Dword)
}

#[test]
fn vmovupd_38() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPD, operand1: Some(IndirectDisplaced(ECX, 1603336666, Some(OperandSize::Zmmword), None)), operand2: Some(Direct(ZMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 253, 72, 17, 161, 218, 249, 144, 95], OperandSize::Dword)
}

#[test]
fn vmovupd_39() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPD, operand1: Some(Direct(ZMM28)), operand2: Some(Direct(ZMM8)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 65, 253, 204, 16, 224], OperandSize::Qword)
}

#[test]
fn vmovupd_40() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPD, operand1: Some(Indirect(RBX, Some(OperandSize::Zmmword), None)), operand2: Some(Direct(ZMM16)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 225, 253, 72, 17, 3], OperandSize::Qword)
}

