use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vmovupd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 16, 241], OperandSize::Dword)
}

#[test]
fn vmovupd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPD, operand1: Some(Direct(XMM0)), operand2: Some(IndirectScaledIndexed(ECX, EDX, Four, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 16, 4, 145], OperandSize::Dword)
}

#[test]
fn vmovupd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 16, 221], OperandSize::Qword)
}

#[test]
fn vmovupd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPD, operand1: Some(Direct(XMM3)), operand2: Some(Indirect(RAX, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 16, 24], OperandSize::Qword)
}

#[test]
fn vmovupd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPD, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 16, 198], OperandSize::Dword)
}

#[test]
fn vmovupd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPD, operand1: Some(Direct(YMM5)), operand2: Some(Indirect(ESI, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 16, 46], OperandSize::Dword)
}

#[test]
fn vmovupd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 16, 234], OperandSize::Qword)
}

#[test]
fn vmovupd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPD, operand1: Some(Direct(YMM2)), operand2: Some(Indirect(RSI, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 16, 22], OperandSize::Qword)
}

#[test]
fn vmovupd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 253, 140, 16, 247], OperandSize::Dword)
}

#[test]
fn vmovupd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPD, operand1: Some(Direct(XMM1)), operand2: Some(IndirectScaledIndexed(EDX, ECX, Four, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 253, 137, 16, 12, 138], OperandSize::Dword)
}

#[test]
fn vmovupd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPD, operand1: Some(Direct(XMM20)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 225, 253, 142, 16, 228], OperandSize::Qword)
}

#[test]
fn vmovupd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPD, operand1: Some(Direct(XMM17)), operand2: Some(IndirectDisplaced(RCX, 1305176396, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 225, 253, 137, 16, 137, 76, 105, 203, 77], OperandSize::Qword)
}

#[test]
fn vmovupd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 253, 175, 16, 252], OperandSize::Dword)
}

#[test]
fn vmovupd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPD, operand1: Some(Direct(YMM5)), operand2: Some(Indirect(EAX, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 253, 170, 16, 40], OperandSize::Dword)
}

#[test]
fn vmovupd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPD, operand1: Some(Direct(YMM28)), operand2: Some(Direct(YMM30)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 1, 253, 173, 16, 230], OperandSize::Qword)
}

#[test]
fn vmovupd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPD, operand1: Some(Direct(YMM30)), operand2: Some(IndirectDisplaced(RDX, 408359552, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 97, 253, 172, 16, 178, 128, 18, 87, 24], OperandSize::Qword)
}

#[test]
fn vmovupd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPD, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 253, 207, 16, 193], OperandSize::Dword)
}

#[test]
fn vmovupd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPD, operand1: Some(Direct(ZMM0)), operand2: Some(IndirectScaledIndexed(ESI, ECX, Eight, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 253, 204, 16, 4, 206], OperandSize::Dword)
}

#[test]
fn vmovupd_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPD, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 253, 206, 16, 204], OperandSize::Qword)
}

#[test]
fn vmovupd_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPD, operand1: Some(Direct(ZMM11)), operand2: Some(IndirectScaledDisplaced(RBX, Eight, 1416962940, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 113, 253, 201, 16, 28, 221, 124, 35, 117, 84], OperandSize::Qword)
}

#[test]
fn vmovupd_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 16, 205], OperandSize::Dword)
}

#[test]
fn vmovupd_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPD, operand1: Some(IndirectScaledIndexedDisplaced(EBX, ESI, Four, 270910934, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 17, 132, 179, 214, 197, 37, 16], OperandSize::Dword)
}

#[test]
fn vmovupd_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 16, 252], OperandSize::Qword)
}

#[test]
fn vmovupd_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPD, operand1: Some(IndirectScaledIndexedDisplaced(RDI, RSI, Four, 1129257858, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 17, 164, 183, 130, 27, 79, 67], OperandSize::Qword)
}

#[test]
fn vmovupd_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPD, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 16, 223], OperandSize::Dword)
}

#[test]
fn vmovupd_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPD, operand1: Some(Indirect(EDI, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(YMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 17, 55], OperandSize::Dword)
}

#[test]
fn vmovupd_27() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 16, 232], OperandSize::Qword)
}

#[test]
fn vmovupd_28() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPD, operand1: Some(IndirectDisplaced(RSI, 1989076454, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(YMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 17, 174, 230, 229, 142, 118], OperandSize::Qword)
}

#[test]
fn vmovupd_29() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 253, 138, 16, 244], OperandSize::Dword)
}

#[test]
fn vmovupd_30() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPD, operand1: Some(IndirectScaledIndexedDisplaced(EBX, ESI, Eight, 1711242735, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 17, 148, 243, 239, 125, 255, 101], OperandSize::Dword)
}

#[test]
fn vmovupd_31() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPD, operand1: Some(Direct(XMM16)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 225, 253, 139, 16, 194], OperandSize::Qword)
}

#[test]
fn vmovupd_32() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPD, operand1: Some(IndirectDisplaced(RBX, 2001347007, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM24)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 97, 253, 8, 17, 131, 191, 33, 74, 119], OperandSize::Qword)
}

#[test]
fn vmovupd_33() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPD, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 253, 173, 16, 210], OperandSize::Dword)
}

#[test]
fn vmovupd_34() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPD, operand1: Some(Indirect(EDX, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(YMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 17, 10], OperandSize::Dword)
}

#[test]
fn vmovupd_35() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPD, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 253, 171, 16, 206], OperandSize::Qword)
}

#[test]
fn vmovupd_36() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPD, operand1: Some(IndirectDisplaced(RDI, 23682063, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(YMM30)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 97, 253, 40, 17, 183, 15, 92, 105, 1], OperandSize::Qword)
}

#[test]
fn vmovupd_37() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPD, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 253, 205, 16, 199], OperandSize::Dword)
}

#[test]
fn vmovupd_38() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPD, operand1: Some(IndirectScaledDisplaced(EAX, Eight, 2015011794, Some(OperandSize::Zmmword), None)), operand2: Some(Direct(ZMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 253, 72, 17, 28, 197, 210, 163, 26, 120], OperandSize::Dword)
}

#[test]
fn vmovupd_39() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPD, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM21)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 177, 253, 205, 16, 221], OperandSize::Qword)
}

#[test]
fn vmovupd_40() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPD, operand1: Some(IndirectDisplaced(RDX, 1033804406, Some(OperandSize::Zmmword), None)), operand2: Some(Direct(ZMM10)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 113, 253, 72, 17, 146, 118, 154, 158, 61], OperandSize::Qword)
}

