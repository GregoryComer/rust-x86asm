use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vmovapd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 40, 225], OperandSize::Dword)
}

#[test]
fn vmovapd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPD, operand1: Some(Direct(XMM1)), operand2: Some(IndirectScaledIndexed(EAX, ESI, Eight, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 40, 12, 240], OperandSize::Dword)
}

#[test]
fn vmovapd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 40, 204], OperandSize::Qword)
}

#[test]
fn vmovapd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPD, operand1: Some(Direct(XMM6)), operand2: Some(IndirectScaledDisplaced(RCX, Four, 1162004254, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 40, 52, 141, 30, 199, 66, 69], OperandSize::Qword)
}

#[test]
fn vmovapd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPD, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 40, 223], OperandSize::Dword)
}

#[test]
fn vmovapd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPD, operand1: Some(Direct(YMM6)), operand2: Some(IndirectScaledDisplaced(EAX, Eight, 1005976921, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 40, 52, 197, 89, 253, 245, 59], OperandSize::Dword)
}

#[test]
fn vmovapd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPD, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 40, 243], OperandSize::Qword)
}

#[test]
fn vmovapd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPD, operand1: Some(Direct(YMM2)), operand2: Some(IndirectScaledIndexed(RSI, RDI, Two, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 40, 20, 126], OperandSize::Qword)
}

#[test]
fn vmovapd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 253, 138, 40, 217], OperandSize::Dword)
}

#[test]
fn vmovapd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPD, operand1: Some(Direct(XMM2)), operand2: Some(IndirectDisplaced(ECX, 385934837, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 253, 137, 40, 145, 245, 229, 0, 23], OperandSize::Dword)
}

#[test]
fn vmovapd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPD, operand1: Some(Direct(XMM12)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 113, 253, 140, 40, 230], OperandSize::Qword)
}

#[test]
fn vmovapd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPD, operand1: Some(Direct(XMM23)), operand2: Some(IndirectScaledDisplaced(RSI, Two, 795924592, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 225, 253, 137, 40, 60, 117, 112, 216, 112, 47], OperandSize::Qword)
}

#[test]
fn vmovapd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 253, 174, 40, 251], OperandSize::Dword)
}

#[test]
fn vmovapd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPD, operand1: Some(Direct(YMM0)), operand2: Some(IndirectScaledIndexedDisplaced(EDI, EBX, Four, 1247571383, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 253, 174, 40, 132, 159, 183, 109, 92, 74], OperandSize::Dword)
}

#[test]
fn vmovapd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPD, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM18)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 177, 253, 175, 40, 226], OperandSize::Qword)
}

#[test]
fn vmovapd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPD, operand1: Some(Direct(YMM8)), operand2: Some(IndirectScaledDisplaced(RSI, Eight, 884608383, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 113, 253, 172, 40, 4, 245, 127, 13, 186, 52], OperandSize::Qword)
}

#[test]
fn vmovapd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPD, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 253, 201, 40, 228], OperandSize::Dword)
}

#[test]
fn vmovapd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPD, operand1: Some(Direct(ZMM5)), operand2: Some(IndirectScaledDisplaced(EAX, Four, 366578017, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 253, 202, 40, 44, 133, 97, 137, 217, 21], OperandSize::Dword)
}

#[test]
fn vmovapd_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPD, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM9)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 209, 253, 201, 40, 217], OperandSize::Qword)
}

#[test]
fn vmovapd_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPD, operand1: Some(Direct(ZMM14)), operand2: Some(IndirectScaledDisplaced(RBX, Four, 1313761069, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 113, 253, 206, 40, 52, 157, 45, 103, 78, 78], OperandSize::Qword)
}

#[test]
fn vmovapd_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 40, 255], OperandSize::Dword)
}

#[test]
fn vmovapd_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPD, operand1: Some(IndirectScaledDisplaced(EDI, Eight, 1573463331, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 41, 4, 253, 35, 37, 201, 93], OperandSize::Dword)
}

#[test]
fn vmovapd_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 40, 200], OperandSize::Qword)
}

#[test]
fn vmovapd_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPD, operand1: Some(IndirectDisplaced(RBX, 1362794166, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 41, 147, 182, 150, 58, 81], OperandSize::Qword)
}

#[test]
fn vmovapd_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPD, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 40, 246], OperandSize::Dword)
}

#[test]
fn vmovapd_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPD, operand1: Some(Indirect(ESI, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(YMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 41, 38], OperandSize::Dword)
}

#[test]
fn vmovapd_27() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPD, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 40, 247], OperandSize::Qword)
}

#[test]
fn vmovapd_28() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPD, operand1: Some(Indirect(RDX, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(YMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 41, 18], OperandSize::Qword)
}

#[test]
fn vmovapd_29() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 253, 139, 40, 212], OperandSize::Dword)
}

#[test]
fn vmovapd_30() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPD, operand1: Some(IndirectScaledIndexed(EDX, EAX, Eight, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 41, 36, 194], OperandSize::Dword)
}

#[test]
fn vmovapd_31() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPD, operand1: Some(Direct(XMM14)), operand2: Some(Direct(XMM22)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 49, 253, 142, 40, 246], OperandSize::Qword)
}

#[test]
fn vmovapd_32() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPD, operand1: Some(IndirectDisplaced(RDX, 222259981, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM31)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 97, 253, 8, 41, 186, 13, 107, 63, 13], OperandSize::Qword)
}

#[test]
fn vmovapd_33() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPD, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 253, 174, 40, 200], OperandSize::Dword)
}

#[test]
fn vmovapd_34() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPD, operand1: Some(IndirectScaledDisplaced(EDX, Eight, 925040968, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(YMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 41, 36, 213, 72, 1, 35, 55], OperandSize::Dword)
}

#[test]
fn vmovapd_35() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPD, operand1: Some(Direct(YMM18)), operand2: Some(Direct(YMM18)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 161, 253, 172, 40, 210], OperandSize::Qword)
}

#[test]
fn vmovapd_36() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPD, operand1: Some(IndirectScaledDisplaced(RCX, Four, 692432975, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(YMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 41, 52, 141, 79, 176, 69, 41], OperandSize::Qword)
}

#[test]
fn vmovapd_37() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPD, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 253, 207, 40, 232], OperandSize::Dword)
}

#[test]
fn vmovapd_38() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPD, operand1: Some(Indirect(EAX, Some(OperandSize::Zmmword), None)), operand2: Some(Direct(ZMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 253, 72, 41, 0], OperandSize::Dword)
}

#[test]
fn vmovapd_39() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPD, operand1: Some(Direct(ZMM11)), operand2: Some(Direct(ZMM27)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 17, 253, 201, 40, 219], OperandSize::Qword)
}

#[test]
fn vmovapd_40() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPD, operand1: Some(IndirectScaledDisplaced(RBX, Eight, 1960611431, Some(OperandSize::Zmmword), None)), operand2: Some(Direct(ZMM11)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 113, 253, 72, 41, 28, 221, 103, 142, 220, 116], OperandSize::Qword)
}

