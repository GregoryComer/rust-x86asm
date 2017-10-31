use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vmovapd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 40, 201], OperandSize::Dword)
}

#[test]
fn vmovapd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPD, operand1: Some(Direct(XMM3)), operand2: Some(Indirect(EDI, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 40, 31], OperandSize::Dword)
}

#[test]
fn vmovapd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 40, 209], OperandSize::Qword)
}

#[test]
fn vmovapd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPD, operand1: Some(Direct(XMM5)), operand2: Some(IndirectScaledDisplaced(RSI, Four, 395943360, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 40, 44, 181, 192, 157, 153, 23], OperandSize::Qword)
}

#[test]
fn vmovapd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 40, 235], OperandSize::Dword)
}

#[test]
fn vmovapd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPD, operand1: Some(Direct(YMM0)), operand2: Some(IndirectScaledDisplaced(EBX, Four, 1914755367, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 40, 4, 157, 39, 217, 32, 114], OperandSize::Dword)
}

#[test]
fn vmovapd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPD, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 40, 220], OperandSize::Qword)
}

#[test]
fn vmovapd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPD, operand1: Some(Direct(YMM5)), operand2: Some(IndirectDisplaced(RSI, 725673089, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 40, 174, 129, 228, 64, 43], OperandSize::Qword)
}

#[test]
fn vmovapd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 253, 141, 40, 192], OperandSize::Dword)
}

#[test]
fn vmovapd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPD, operand1: Some(Direct(XMM6)), operand2: Some(Indirect(EBX, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 253, 138, 40, 51], OperandSize::Dword)
}

#[test]
fn vmovapd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM23)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 177, 253, 140, 40, 199], OperandSize::Qword)
}

#[test]
fn vmovapd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPD, operand1: Some(Direct(XMM29)), operand2: Some(IndirectScaledDisplaced(RSI, Two, 994823850, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 97, 253, 138, 40, 44, 117, 170, 206, 75, 59], OperandSize::Qword)
}

#[test]
fn vmovapd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPD, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 253, 175, 40, 199], OperandSize::Dword)
}

#[test]
fn vmovapd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPD, operand1: Some(Direct(YMM1)), operand2: Some(IndirectScaledDisplaced(EAX, Four, 1555590131, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 253, 173, 40, 12, 133, 243, 107, 184, 92], OperandSize::Dword)
}

#[test]
fn vmovapd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPD, operand1: Some(Direct(YMM17)), operand2: Some(Direct(YMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 225, 253, 171, 40, 202], OperandSize::Qword)
}

#[test]
fn vmovapd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPD, operand1: Some(Direct(YMM29)), operand2: Some(IndirectDisplaced(RBX, 1184696613, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 97, 253, 169, 40, 171, 37, 9, 157, 70], OperandSize::Qword)
}

#[test]
fn vmovapd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPD, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 253, 204, 40, 250], OperandSize::Dword)
}

#[test]
fn vmovapd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPD, operand1: Some(Direct(ZMM0)), operand2: Some(IndirectScaledIndexed(EAX, EBX, Four, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 253, 202, 40, 4, 152], OperandSize::Dword)
}

#[test]
fn vmovapd_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPD, operand1: Some(Direct(ZMM31)), operand2: Some(Direct(ZMM9)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 65, 253, 201, 40, 249], OperandSize::Qword)
}

#[test]
fn vmovapd_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPD, operand1: Some(Direct(ZMM25)), operand2: Some(IndirectDisplaced(RAX, 102723404, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 97, 253, 202, 40, 136, 76, 111, 31, 6], OperandSize::Qword)
}

#[test]
fn vmovapd_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 40, 242], OperandSize::Dword)
}

#[test]
fn vmovapd_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPD, operand1: Some(Indirect(EAX, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 41, 56], OperandSize::Dword)
}

#[test]
fn vmovapd_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 40, 210], OperandSize::Qword)
}

#[test]
fn vmovapd_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPD, operand1: Some(IndirectScaledDisplaced(RDI, Four, 25312807, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 41, 60, 189, 39, 62, 130, 1], OperandSize::Qword)
}

#[test]
fn vmovapd_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 40, 237], OperandSize::Dword)
}

#[test]
fn vmovapd_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPD, operand1: Some(Indirect(EAX, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(YMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 41, 40], OperandSize::Dword)
}

#[test]
fn vmovapd_27() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPD, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 40, 196], OperandSize::Qword)
}

#[test]
fn vmovapd_28() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPD, operand1: Some(IndirectScaledDisplaced(RCX, Four, 1463792050, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(YMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 41, 28, 141, 178, 177, 63, 87], OperandSize::Qword)
}

#[test]
fn vmovapd_29() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 253, 138, 40, 221], OperandSize::Dword)
}

#[test]
fn vmovapd_30() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPD, operand1: Some(IndirectScaledIndexed(EBX, EAX, Eight, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 41, 20, 195], OperandSize::Dword)
}

#[test]
fn vmovapd_31() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPD, operand1: Some(Direct(XMM16)), operand2: Some(Direct(XMM29)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 129, 253, 140, 40, 197], OperandSize::Qword)
}

#[test]
fn vmovapd_32() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPD, operand1: Some(IndirectDisplaced(RDX, 1587258261, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM21)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 225, 253, 8, 41, 170, 149, 163, 155, 94], OperandSize::Qword)
}

#[test]
fn vmovapd_33() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPD, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 253, 171, 40, 193], OperandSize::Dword)
}

#[test]
fn vmovapd_34() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPD, operand1: Some(IndirectScaledDisplaced(EAX, Eight, 903114571, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(YMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 41, 44, 197, 75, 111, 212, 53], OperandSize::Dword)
}

#[test]
fn vmovapd_35() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 253, 173, 40, 236], OperandSize::Qword)
}

#[test]
fn vmovapd_36() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPD, operand1: Some(Indirect(RDX, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(YMM23)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 225, 253, 40, 41, 58], OperandSize::Qword)
}

#[test]
fn vmovapd_37() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPD, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 253, 207, 40, 228], OperandSize::Dword)
}

#[test]
fn vmovapd_38() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPD, operand1: Some(IndirectDisplaced(ESI, 1498511521, Some(OperandSize::Zmmword), None)), operand2: Some(Direct(ZMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 253, 72, 41, 134, 161, 120, 81, 89], OperandSize::Dword)
}

#[test]
fn vmovapd_39() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPD, operand1: Some(Direct(ZMM18)), operand2: Some(Direct(ZMM15)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 193, 253, 204, 40, 215], OperandSize::Qword)
}

#[test]
fn vmovapd_40() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPD, operand1: Some(IndirectScaledIndexed(RBX, RDI, Two, Some(OperandSize::Zmmword), None)), operand2: Some(Direct(ZMM29)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 97, 253, 72, 41, 44, 123], OperandSize::Qword)
}

