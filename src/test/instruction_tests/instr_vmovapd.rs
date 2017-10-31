use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vmovapd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 40, 217], OperandSize::Dword)
}

#[test]
fn vmovapd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPD, operand1: Some(Direct(XMM5)), operand2: Some(IndirectDisplaced(ESI, 516943794, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 40, 174, 178, 239, 207, 30], OperandSize::Dword)
}

#[test]
fn vmovapd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 40, 245], OperandSize::Qword)
}

#[test]
fn vmovapd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPD, operand1: Some(Direct(XMM3)), operand2: Some(IndirectScaledDisplaced(RBX, Four, 28691327, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 40, 28, 157, 127, 203, 181, 1], OperandSize::Qword)
}

#[test]
fn vmovapd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPD, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 40, 222], OperandSize::Dword)
}

#[test]
fn vmovapd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPD, operand1: Some(Direct(YMM0)), operand2: Some(IndirectScaledIndexedDisplaced(ECX, EAX, Two, 1278135138, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 40, 132, 65, 98, 203, 46, 76], OperandSize::Dword)
}

#[test]
fn vmovapd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 40, 232], OperandSize::Qword)
}

#[test]
fn vmovapd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPD, operand1: Some(Direct(YMM6)), operand2: Some(Indirect(RCX, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 40, 49], OperandSize::Qword)
}

#[test]
fn vmovapd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 253, 139, 40, 195], OperandSize::Dword)
}

#[test]
fn vmovapd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPD, operand1: Some(Direct(XMM2)), operand2: Some(IndirectScaledDisplaced(EBX, Four, 232155448, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 253, 139, 40, 20, 157, 56, 105, 214, 13], OperandSize::Dword)
}

#[test]
fn vmovapd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPD, operand1: Some(Direct(XMM8)), operand2: Some(Direct(XMM16)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 49, 253, 137, 40, 192], OperandSize::Qword)
}

#[test]
fn vmovapd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPD, operand1: Some(Direct(XMM12)), operand2: Some(IndirectScaledIndexedDisplaced(RBX, RDI, Eight, 65305822, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 113, 253, 138, 40, 164, 251, 222, 124, 228, 3], OperandSize::Qword)
}

#[test]
fn vmovapd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 253, 174, 40, 251], OperandSize::Dword)
}

#[test]
fn vmovapd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPD, operand1: Some(Direct(YMM2)), operand2: Some(Indirect(ESI, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 253, 175, 40, 22], OperandSize::Dword)
}

#[test]
fn vmovapd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPD, operand1: Some(Direct(YMM17)), operand2: Some(Direct(YMM11)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 193, 253, 174, 40, 203], OperandSize::Qword)
}

#[test]
fn vmovapd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPD, operand1: Some(Direct(YMM21)), operand2: Some(IndirectScaledDisplaced(RBX, Four, 816821981, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 225, 253, 173, 40, 44, 157, 221, 182, 175, 48], OperandSize::Qword)
}

#[test]
fn vmovapd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPD, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 253, 205, 40, 223], OperandSize::Dword)
}

#[test]
fn vmovapd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPD, operand1: Some(Direct(ZMM2)), operand2: Some(IndirectScaledIndexedDisplaced(EDX, EBX, Two, 1439334408, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 253, 205, 40, 148, 90, 8, 128, 202, 85], OperandSize::Dword)
}

#[test]
fn vmovapd_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPD, operand1: Some(Direct(ZMM8)), operand2: Some(Direct(ZMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 113, 253, 204, 40, 192], OperandSize::Qword)
}

#[test]
fn vmovapd_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPD, operand1: Some(Direct(ZMM30)), operand2: Some(IndirectScaledIndexedDisplaced(RSI, RAX, Four, 1164040591, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 97, 253, 207, 40, 180, 134, 143, 217, 97, 69], OperandSize::Qword)
}

#[test]
fn vmovapd_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 40, 254], OperandSize::Dword)
}

#[test]
fn vmovapd_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPD, operand1: Some(Indirect(EDI, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 41, 7], OperandSize::Dword)
}

#[test]
fn vmovapd_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 40, 219], OperandSize::Qword)
}

#[test]
fn vmovapd_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPD, operand1: Some(IndirectDisplaced(RSI, 215547544, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 41, 182, 152, 254, 216, 12], OperandSize::Qword)
}

#[test]
fn vmovapd_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 40, 248], OperandSize::Dword)
}

#[test]
fn vmovapd_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPD, operand1: Some(IndirectScaledDisplaced(EBX, Two, 550527485, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(YMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 41, 44, 93, 253, 97, 208, 32], OperandSize::Dword)
}

#[test]
fn vmovapd_27() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPD, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 40, 207], OperandSize::Qword)
}

#[test]
fn vmovapd_28() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPD, operand1: Some(IndirectScaledDisplaced(RAX, Two, 364757822, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(YMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 41, 36, 69, 62, 195, 189, 21], OperandSize::Qword)
}

#[test]
fn vmovapd_29() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 253, 139, 40, 246], OperandSize::Dword)
}

#[test]
fn vmovapd_30() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPD, operand1: Some(IndirectScaledDisplaced(ESI, Four, 1011039593, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 41, 60, 181, 105, 61, 67, 60], OperandSize::Dword)
}

#[test]
fn vmovapd_31() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPD, operand1: Some(Direct(XMM8)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 113, 253, 139, 40, 192], OperandSize::Qword)
}

#[test]
fn vmovapd_32() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPD, operand1: Some(IndirectScaledIndexed(RBX, RBX, Eight, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM8)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 121, 41, 4, 219], OperandSize::Qword)
}

#[test]
fn vmovapd_33() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPD, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 253, 175, 40, 193], OperandSize::Dword)
}

#[test]
fn vmovapd_34() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPD, operand1: Some(IndirectScaledIndexedDisplaced(EAX, EDI, Four, 1194939880, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(YMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 41, 172, 184, 232, 85, 57, 71], OperandSize::Dword)
}

#[test]
fn vmovapd_35() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPD, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM26)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 145, 253, 172, 40, 218], OperandSize::Qword)
}

#[test]
fn vmovapd_36() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPD, operand1: Some(Indirect(RDX, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(YMM26)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 97, 253, 40, 41, 18], OperandSize::Qword)
}

#[test]
fn vmovapd_37() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPD, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 253, 205, 40, 222], OperandSize::Dword)
}

#[test]
fn vmovapd_38() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPD, operand1: Some(IndirectDisplaced(ESI, 657747289, Some(OperandSize::Zmmword), None)), operand2: Some(Direct(ZMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 253, 72, 41, 190, 89, 109, 52, 39], OperandSize::Dword)
}

#[test]
fn vmovapd_39() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPD, operand1: Some(Direct(ZMM18)), operand2: Some(Direct(ZMM19)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 161, 253, 201, 40, 211], OperandSize::Qword)
}

#[test]
fn vmovapd_40() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPD, operand1: Some(IndirectDisplaced(RCX, 1044372203, Some(OperandSize::Zmmword), None)), operand2: Some(Direct(ZMM29)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 97, 253, 72, 41, 169, 235, 218, 63, 62], OperandSize::Qword)
}

