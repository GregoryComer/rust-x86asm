use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vmovupd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 16, 205], OperandSize::Dword)
}

#[test]
fn vmovupd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPD, operand1: Some(Direct(XMM7)), operand2: Some(IndirectScaledIndexedDisplaced(EBX, EDI, Four, 1552446691, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 16, 188, 187, 227, 116, 136, 92], OperandSize::Dword)
}

#[test]
fn vmovupd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 16, 225], OperandSize::Qword)
}

#[test]
fn vmovupd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPD, operand1: Some(Direct(XMM4)), operand2: Some(IndirectScaledDisplaced(RCX, Eight, 1563497663, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 16, 36, 205, 191, 20, 49, 93], OperandSize::Qword)
}

#[test]
fn vmovupd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPD, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 16, 198], OperandSize::Dword)
}

#[test]
fn vmovupd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPD, operand1: Some(Direct(YMM0)), operand2: Some(IndirectScaledIndexedDisplaced(ESI, EDI, Two, 4854010, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 16, 132, 126, 250, 16, 74, 0], OperandSize::Dword)
}

#[test]
fn vmovupd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPD, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 16, 209], OperandSize::Qword)
}

#[test]
fn vmovupd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPD, operand1: Some(Direct(YMM7)), operand2: Some(IndirectScaledIndexed(RAX, RBX, Eight, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 16, 60, 216], OperandSize::Qword)
}

#[test]
fn vmovupd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 253, 141, 16, 229], OperandSize::Dword)
}

#[test]
fn vmovupd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPD, operand1: Some(Direct(XMM4)), operand2: Some(IndirectScaledIndexed(EDX, EDI, Four, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 253, 138, 16, 36, 186], OperandSize::Dword)
}

#[test]
fn vmovupd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM19)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 177, 253, 138, 16, 235], OperandSize::Qword)
}

#[test]
fn vmovupd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPD, operand1: Some(Direct(XMM14)), operand2: Some(IndirectScaledIndexed(RCX, RBX, Eight, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 113, 253, 137, 16, 52, 217], OperandSize::Qword)
}

#[test]
fn vmovupd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPD, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 253, 169, 16, 213], OperandSize::Dword)
}

#[test]
fn vmovupd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPD, operand1: Some(Direct(YMM5)), operand2: Some(Indirect(EBX, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 253, 174, 16, 43], OperandSize::Dword)
}

#[test]
fn vmovupd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPD, operand1: Some(Direct(YMM18)), operand2: Some(Direct(YMM25)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 129, 253, 171, 16, 209], OperandSize::Qword)
}

#[test]
fn vmovupd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPD, operand1: Some(Direct(YMM7)), operand2: Some(IndirectScaledIndexedDisplaced(RDX, RBX, Eight, 800976767, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 253, 172, 16, 188, 218, 127, 239, 189, 47], OperandSize::Qword)
}

#[test]
fn vmovupd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPD, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 253, 203, 16, 212], OperandSize::Dword)
}

#[test]
fn vmovupd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPD, operand1: Some(Direct(ZMM6)), operand2: Some(IndirectScaledIndexed(EBX, EAX, Eight, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 253, 201, 16, 52, 195], OperandSize::Dword)
}

#[test]
fn vmovupd_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPD, operand1: Some(Direct(ZMM29)), operand2: Some(Direct(ZMM16)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 33, 253, 206, 16, 232], OperandSize::Qword)
}

#[test]
fn vmovupd_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPD, operand1: Some(Direct(ZMM25)), operand2: Some(IndirectScaledIndexed(RDX, RSI, Two, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 97, 253, 206, 16, 12, 114], OperandSize::Qword)
}

#[test]
fn vmovupd_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 16, 197], OperandSize::Dword)
}

#[test]
fn vmovupd_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPD, operand1: Some(IndirectScaledIndexedDisplaced(EDI, ECX, Eight, 395034271, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 17, 140, 207, 159, 190, 139, 23], OperandSize::Dword)
}

#[test]
fn vmovupd_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 16, 203], OperandSize::Qword)
}

#[test]
fn vmovupd_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPD, operand1: Some(IndirectScaledDisplaced(RSI, Eight, 1884106789, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 17, 52, 245, 37, 48, 77, 112], OperandSize::Qword)
}

#[test]
fn vmovupd_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPD, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 16, 199], OperandSize::Dword)
}

#[test]
fn vmovupd_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPD, operand1: Some(Indirect(ESI, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(YMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 17, 46], OperandSize::Dword)
}

#[test]
fn vmovupd_27() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPD, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 16, 231], OperandSize::Qword)
}

#[test]
fn vmovupd_28() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPD, operand1: Some(IndirectScaledIndexedDisplaced(RDX, RAX, Four, 1541825360, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(YMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 17, 148, 130, 80, 99, 230, 91], OperandSize::Qword)
}

#[test]
fn vmovupd_29() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 253, 142, 16, 241], OperandSize::Dword)
}

#[test]
fn vmovupd_30() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPD, operand1: Some(Indirect(EAX, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 17, 40], OperandSize::Dword)
}

#[test]
fn vmovupd_31() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM22)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 177, 253, 138, 16, 254], OperandSize::Qword)
}

#[test]
fn vmovupd_32() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPD, operand1: Some(IndirectScaledIndexedDisplaced(RDX, RSI, Two, 2014841489, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM21)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 225, 253, 8, 17, 172, 114, 145, 10, 24, 120], OperandSize::Qword)
}

#[test]
fn vmovupd_33() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPD, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 253, 173, 16, 213], OperandSize::Dword)
}

#[test]
fn vmovupd_34() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPD, operand1: Some(IndirectDisplaced(EDX, 1220124599, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(YMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 17, 130, 183, 159, 185, 72], OperandSize::Dword)
}

#[test]
fn vmovupd_35() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPD, operand1: Some(Direct(YMM10)), operand2: Some(Direct(YMM9)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 81, 253, 175, 16, 209], OperandSize::Qword)
}

#[test]
fn vmovupd_36() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPD, operand1: Some(IndirectScaledIndexed(RDX, RDX, Two, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(YMM20)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 225, 253, 40, 17, 36, 82], OperandSize::Qword)
}

#[test]
fn vmovupd_37() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPD, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 253, 202, 16, 198], OperandSize::Dword)
}

#[test]
fn vmovupd_38() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPD, operand1: Some(IndirectScaledDisplaced(EDX, Four, 621892450, Some(OperandSize::Zmmword), None)), operand2: Some(Direct(ZMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 253, 72, 17, 44, 149, 98, 83, 17, 37], OperandSize::Dword)
}

#[test]
fn vmovupd_39() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPD, operand1: Some(Direct(ZMM27)), operand2: Some(Direct(ZMM19)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 33, 253, 207, 16, 219], OperandSize::Qword)
}

#[test]
fn vmovupd_40() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPD, operand1: Some(IndirectScaledIndexed(RDX, RDX, Eight, Some(OperandSize::Zmmword), None)), operand2: Some(Direct(ZMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 253, 72, 17, 36, 210], OperandSize::Qword)
}

