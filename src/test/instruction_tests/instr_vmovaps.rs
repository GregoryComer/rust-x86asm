use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vmovaps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 40, 228], OperandSize::Dword)
}

#[test]
fn vmovaps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPS, operand1: Some(Direct(XMM0)), operand2: Some(IndirectScaledIndexedDisplaced(ESI, ECX, Four, 845944462, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 40, 132, 142, 142, 22, 108, 50], OperandSize::Dword)
}

#[test]
fn vmovaps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 40, 250], OperandSize::Qword)
}

#[test]
fn vmovaps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPS, operand1: Some(Direct(XMM1)), operand2: Some(IndirectScaledDisplaced(RDI, Eight, 1228425412, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 40, 12, 253, 196, 72, 56, 73], OperandSize::Qword)
}

#[test]
fn vmovaps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPS, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 252, 40, 194], OperandSize::Dword)
}

#[test]
fn vmovaps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPS, operand1: Some(Direct(YMM4)), operand2: Some(IndirectScaledIndexed(EAX, EAX, Four, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 252, 40, 36, 128], OperandSize::Dword)
}

#[test]
fn vmovaps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPS, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 252, 40, 245], OperandSize::Qword)
}

#[test]
fn vmovaps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPS, operand1: Some(Direct(YMM7)), operand2: Some(IndirectScaledIndexedDisplaced(RSI, RDX, Two, 704445627, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 252, 40, 188, 86, 187, 252, 252, 41], OperandSize::Qword)
}

#[test]
fn vmovaps_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 124, 141, 40, 227], OperandSize::Dword)
}

#[test]
fn vmovaps_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPS, operand1: Some(Direct(XMM7)), operand2: Some(IndirectScaledIndexedDisplaced(EDI, EDX, Two, 1170079984, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 124, 140, 40, 188, 87, 240, 0, 190, 69], OperandSize::Dword)
}

#[test]
fn vmovaps_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPS, operand1: Some(Direct(XMM28)), operand2: Some(Direct(XMM23)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 33, 124, 143, 40, 231], OperandSize::Qword)
}

#[test]
fn vmovaps_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPS, operand1: Some(Direct(XMM19)), operand2: Some(IndirectScaledIndexedDisplaced(RBX, RBX, Four, 160494243, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 225, 124, 141, 40, 156, 155, 163, 242, 144, 9], OperandSize::Qword)
}

#[test]
fn vmovaps_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPS, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 124, 174, 40, 215], OperandSize::Dword)
}

#[test]
fn vmovaps_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPS, operand1: Some(Direct(YMM1)), operand2: Some(IndirectScaledIndexedDisplaced(EDI, EBX, Two, 1576948463, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 124, 170, 40, 140, 95, 239, 82, 254, 93], OperandSize::Dword)
}

#[test]
fn vmovaps_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPS, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM30)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 145, 124, 169, 40, 198], OperandSize::Qword)
}

#[test]
fn vmovaps_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPS, operand1: Some(Direct(YMM22)), operand2: Some(IndirectScaledIndexed(RAX, RAX, Four, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 225, 124, 173, 40, 52, 128], OperandSize::Qword)
}

#[test]
fn vmovaps_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPS, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 124, 204, 40, 249], OperandSize::Dword)
}

#[test]
fn vmovaps_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPS, operand1: Some(Direct(ZMM1)), operand2: Some(IndirectScaledDisplaced(EBX, Two, 2016267702, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 124, 207, 40, 12, 93, 182, 205, 45, 120], OperandSize::Dword)
}

#[test]
fn vmovaps_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPS, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 124, 204, 40, 239], OperandSize::Qword)
}

#[test]
fn vmovaps_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPS, operand1: Some(Direct(ZMM15)), operand2: Some(IndirectScaledDisplaced(RDI, Two, 71674821, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 113, 124, 201, 40, 60, 125, 197, 171, 69, 4], OperandSize::Qword)
}

#[test]
fn vmovaps_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 40, 229], OperandSize::Dword)
}

#[test]
fn vmovaps_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPS, operand1: Some(IndirectScaledDisplaced(EAX, Four, 1032093871, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 41, 36, 133, 175, 128, 132, 61], OperandSize::Dword)
}

#[test]
fn vmovaps_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 40, 193], OperandSize::Qword)
}

#[test]
fn vmovaps_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPS, operand1: Some(IndirectDisplaced(RCX, 1889043645, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 41, 137, 189, 132, 152, 112], OperandSize::Qword)
}

#[test]
fn vmovaps_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPS, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 252, 40, 248], OperandSize::Dword)
}

#[test]
fn vmovaps_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPS, operand1: Some(IndirectDisplaced(EBX, 122345682, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(YMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 252, 41, 187, 210, 216, 74, 7], OperandSize::Dword)
}

#[test]
fn vmovaps_27() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPS, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 252, 40, 204], OperandSize::Qword)
}

#[test]
fn vmovaps_28() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPS, operand1: Some(IndirectScaledDisplaced(RBX, Two, 592846184, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(YMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 252, 41, 28, 93, 104, 29, 86, 35], OperandSize::Qword)
}

#[test]
fn vmovaps_29() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 124, 138, 40, 196], OperandSize::Dword)
}

#[test]
fn vmovaps_30() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPS, operand1: Some(IndirectScaledIndexed(ECX, EDI, Eight, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 41, 12, 249], OperandSize::Dword)
}

#[test]
fn vmovaps_31() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPS, operand1: Some(Direct(XMM23)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 225, 124, 138, 40, 254], OperandSize::Qword)
}

#[test]
fn vmovaps_32() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPS, operand1: Some(IndirectDisplaced(RDI, 323128582, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM29)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 97, 124, 8, 41, 175, 6, 141, 66, 19], OperandSize::Qword)
}

#[test]
fn vmovaps_33() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPS, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 124, 169, 40, 203], OperandSize::Dword)
}

#[test]
fn vmovaps_34() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPS, operand1: Some(IndirectScaledIndexedDisplaced(ECX, EDI, Four, 41719815, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(YMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 252, 41, 148, 185, 7, 152, 124, 2], OperandSize::Dword)
}

#[test]
fn vmovaps_35() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPS, operand1: Some(Direct(YMM10)), operand2: Some(Direct(YMM18)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 49, 124, 175, 40, 210], OperandSize::Qword)
}

#[test]
fn vmovaps_36() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPS, operand1: Some(Indirect(RDX, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(YMM16)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 225, 124, 40, 41, 2], OperandSize::Qword)
}

#[test]
fn vmovaps_37() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPS, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 124, 201, 40, 214], OperandSize::Dword)
}

#[test]
fn vmovaps_38() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPS, operand1: Some(IndirectDisplaced(EDI, 1121626589, Some(OperandSize::Zmmword), None)), operand2: Some(Direct(ZMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 124, 72, 41, 167, 221, 169, 218, 66], OperandSize::Dword)
}

#[test]
fn vmovaps_39() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPS, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM16)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 177, 124, 207, 40, 224], OperandSize::Qword)
}

#[test]
fn vmovaps_40() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPS, operand1: Some(IndirectScaledIndexedDisplaced(RDI, RCX, Two, 2072707765, Some(OperandSize::Zmmword), None)), operand2: Some(Direct(ZMM12)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 113, 124, 72, 41, 164, 79, 181, 2, 139, 123], OperandSize::Qword)
}

