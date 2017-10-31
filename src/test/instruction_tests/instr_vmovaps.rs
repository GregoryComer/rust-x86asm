use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vmovaps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 40, 231], OperandSize::Dword)
}

#[test]
fn vmovaps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPS, operand1: Some(Direct(XMM5)), operand2: Some(IndirectScaledIndexedDisplaced(ECX, ECX, Four, 1269212838, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 40, 172, 137, 166, 166, 166, 75], OperandSize::Dword)
}

#[test]
fn vmovaps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 40, 202], OperandSize::Qword)
}

#[test]
fn vmovaps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPS, operand1: Some(Direct(XMM7)), operand2: Some(IndirectScaledIndexed(RDI, RCX, Four, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 40, 60, 143], OperandSize::Qword)
}

#[test]
fn vmovaps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPS, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 252, 40, 239], OperandSize::Dword)
}

#[test]
fn vmovaps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPS, operand1: Some(Direct(YMM3)), operand2: Some(IndirectScaledDisplaced(EAX, Four, 1843532353, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 252, 40, 28, 133, 65, 18, 226, 109], OperandSize::Dword)
}

#[test]
fn vmovaps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPS, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 252, 40, 232], OperandSize::Qword)
}

#[test]
fn vmovaps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPS, operand1: Some(Direct(YMM3)), operand2: Some(IndirectScaledDisplaced(RCX, Four, 1895485957, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 252, 40, 28, 141, 5, 210, 250, 112], OperandSize::Qword)
}

#[test]
fn vmovaps_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 124, 143, 40, 201], OperandSize::Dword)
}

#[test]
fn vmovaps_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPS, operand1: Some(Direct(XMM5)), operand2: Some(IndirectScaledIndexedDisplaced(ECX, ECX, Two, 716283707, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 124, 139, 40, 172, 73, 59, 159, 177, 42], OperandSize::Dword)
}

#[test]
fn vmovaps_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPS, operand1: Some(Direct(XMM20)), operand2: Some(Direct(XMM21)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 161, 124, 139, 40, 229], OperandSize::Qword)
}

#[test]
fn vmovaps_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPS, operand1: Some(Direct(XMM5)), operand2: Some(IndirectScaledIndexed(RAX, RCX, Eight, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 124, 138, 40, 44, 200], OperandSize::Qword)
}

#[test]
fn vmovaps_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPS, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 124, 172, 40, 195], OperandSize::Dword)
}

#[test]
fn vmovaps_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPS, operand1: Some(Direct(YMM5)), operand2: Some(IndirectScaledIndexedDisplaced(ECX, EDX, Eight, 1760304406, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 124, 172, 40, 172, 209, 22, 29, 236, 104], OperandSize::Dword)
}

#[test]
fn vmovaps_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPS, operand1: Some(Direct(YMM30)), operand2: Some(Direct(YMM11)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 65, 124, 173, 40, 243], OperandSize::Qword)
}

#[test]
fn vmovaps_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPS, operand1: Some(Direct(YMM11)), operand2: Some(IndirectDisplaced(RBX, 1544871093, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 113, 124, 171, 40, 155, 181, 220, 20, 92], OperandSize::Qword)
}

#[test]
fn vmovaps_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPS, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 124, 207, 40, 249], OperandSize::Dword)
}

#[test]
fn vmovaps_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPS, operand1: Some(Direct(ZMM7)), operand2: Some(IndirectScaledIndexedDisplaced(ESI, EDI, Two, 1735544650, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 124, 206, 40, 188, 126, 74, 79, 114, 103], OperandSize::Dword)
}

#[test]
fn vmovaps_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPS, operand1: Some(Direct(ZMM12)), operand2: Some(Direct(ZMM28)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 17, 124, 203, 40, 228], OperandSize::Qword)
}

#[test]
fn vmovaps_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPS, operand1: Some(Direct(ZMM17)), operand2: Some(IndirectDisplaced(RCX, 123080958, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 225, 124, 201, 40, 137, 254, 16, 86, 7], OperandSize::Qword)
}

#[test]
fn vmovaps_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 40, 212], OperandSize::Dword)
}

#[test]
fn vmovaps_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPS, operand1: Some(IndirectDisplaced(EAX, 327406364, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 41, 168, 28, 211, 131, 19], OperandSize::Dword)
}

#[test]
fn vmovaps_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 40, 249], OperandSize::Qword)
}

#[test]
fn vmovaps_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPS, operand1: Some(IndirectScaledIndexedDisplaced(RSI, RSI, Two, 1631214406, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 41, 140, 118, 70, 91, 58, 97], OperandSize::Qword)
}

#[test]
fn vmovaps_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPS, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 252, 40, 225], OperandSize::Dword)
}

#[test]
fn vmovaps_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPS, operand1: Some(IndirectScaledIndexedDisplaced(ESI, ESI, Four, 1409863557, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(YMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 252, 41, 156, 182, 133, 207, 8, 84], OperandSize::Dword)
}

#[test]
fn vmovaps_27() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPS, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 252, 40, 240], OperandSize::Qword)
}

#[test]
fn vmovaps_28() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPS, operand1: Some(IndirectDisplaced(RSI, 828268557, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(YMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 252, 41, 166, 13, 96, 94, 49], OperandSize::Qword)
}

#[test]
fn vmovaps_29() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 124, 139, 40, 197], OperandSize::Dword)
}

#[test]
fn vmovaps_30() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPS, operand1: Some(IndirectDisplaced(EDX, 752043146, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 41, 170, 138, 68, 211, 44], OperandSize::Dword)
}

#[test]
fn vmovaps_31() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPS, operand1: Some(Direct(XMM19)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 225, 124, 138, 40, 218], OperandSize::Qword)
}

#[test]
fn vmovaps_32() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPS, operand1: Some(Indirect(RCX, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM14)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 120, 41, 49], OperandSize::Qword)
}

#[test]
fn vmovaps_33() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPS, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 124, 172, 40, 208], OperandSize::Dword)
}

#[test]
fn vmovaps_34() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPS, operand1: Some(IndirectScaledIndexed(ECX, EBX, Four, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(YMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 252, 41, 36, 153], OperandSize::Dword)
}

#[test]
fn vmovaps_35() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPS, operand1: Some(Direct(YMM8)), operand2: Some(Direct(YMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 113, 124, 174, 40, 192], OperandSize::Qword)
}

#[test]
fn vmovaps_36() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPS, operand1: Some(IndirectScaledDisplaced(RCX, Eight, 775200694, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(YMM29)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 97, 124, 40, 41, 44, 205, 182, 159, 52, 46], OperandSize::Qword)
}

#[test]
fn vmovaps_37() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPS, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 124, 201, 40, 255], OperandSize::Dword)
}

#[test]
fn vmovaps_38() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPS, operand1: Some(IndirectScaledDisplaced(EBX, Four, 782656979, Some(OperandSize::Zmmword), None)), operand2: Some(Direct(ZMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 124, 72, 41, 44, 157, 211, 101, 166, 46], OperandSize::Dword)
}

#[test]
fn vmovaps_39() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPS, operand1: Some(Direct(ZMM22)), operand2: Some(Direct(ZMM29)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 129, 124, 207, 40, 245], OperandSize::Qword)
}

#[test]
fn vmovaps_40() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPS, operand1: Some(IndirectScaledIndexedDisplaced(RDI, RBX, Four, 1115180787, Some(OperandSize::Zmmword), None)), operand2: Some(Direct(ZMM24)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 97, 124, 72, 41, 132, 159, 243, 78, 120, 66], OperandSize::Qword)
}

