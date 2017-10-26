use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vmovupd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 16, 218], OperandSize::Dword)
}

#[test]
fn vmovupd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPD, operand1: Some(Direct(XMM4)), operand2: Some(IndirectScaledIndexed(ESI, EAX, Four, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 16, 36, 134], OperandSize::Dword)
}

#[test]
fn vmovupd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 16, 209], OperandSize::Qword)
}

#[test]
fn vmovupd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPD, operand1: Some(Direct(XMM4)), operand2: Some(IndirectDisplaced(RAX, 4142118, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 16, 160, 38, 52, 63, 0], OperandSize::Qword)
}

#[test]
fn vmovupd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 16, 236], OperandSize::Dword)
}

#[test]
fn vmovupd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPD, operand1: Some(Direct(YMM7)), operand2: Some(IndirectScaledDisplaced(ESI, Four, 1815516541, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 16, 60, 181, 125, 149, 54, 108], OperandSize::Dword)
}

#[test]
fn vmovupd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPD, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 16, 246], OperandSize::Qword)
}

#[test]
fn vmovupd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPD, operand1: Some(Direct(YMM5)), operand2: Some(Indirect(RAX, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 16, 40], OperandSize::Qword)
}

#[test]
fn vmovupd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 253, 138, 16, 218], OperandSize::Dword)
}

#[test]
fn vmovupd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPD, operand1: Some(Direct(XMM5)), operand2: Some(IndirectScaledIndexedDisplaced(EDI, ESI, Eight, 1243268114, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 253, 143, 16, 172, 247, 18, 196, 26, 74], OperandSize::Dword)
}

#[test]
fn vmovupd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPD, operand1: Some(Direct(XMM17)), operand2: Some(Direct(XMM20)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 161, 253, 143, 16, 204], OperandSize::Qword)
}

#[test]
fn vmovupd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPD, operand1: Some(Direct(XMM24)), operand2: Some(IndirectScaledIndexedDisplaced(RAX, RSI, Four, 336414386, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 97, 253, 137, 16, 132, 176, 178, 70, 13, 20], OperandSize::Qword)
}

#[test]
fn vmovupd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 253, 171, 16, 236], OperandSize::Dword)
}

#[test]
fn vmovupd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPD, operand1: Some(Direct(YMM0)), operand2: Some(IndirectScaledIndexed(EAX, EBX, Eight, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 253, 174, 16, 4, 216], OperandSize::Dword)
}

#[test]
fn vmovupd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPD, operand1: Some(Direct(YMM21)), operand2: Some(Direct(YMM9)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 193, 253, 172, 16, 233], OperandSize::Qword)
}

#[test]
fn vmovupd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPD, operand1: Some(Direct(YMM30)), operand2: Some(IndirectScaledIndexedDisplaced(RCX, RAX, Eight, 2052605181, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 97, 253, 169, 16, 180, 193, 253, 68, 88, 122], OperandSize::Qword)
}

#[test]
fn vmovupd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPD, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 253, 201, 16, 232], OperandSize::Dword)
}

#[test]
fn vmovupd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPD, operand1: Some(Direct(ZMM3)), operand2: Some(IndirectDisplaced(ECX, 82907189, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 253, 203, 16, 153, 53, 16, 241, 4], OperandSize::Dword)
}

#[test]
fn vmovupd_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPD, operand1: Some(Direct(ZMM20)), operand2: Some(Direct(ZMM12)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 193, 253, 205, 16, 228], OperandSize::Qword)
}

#[test]
fn vmovupd_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPD, operand1: Some(Direct(ZMM5)), operand2: Some(IndirectScaledIndexed(RAX, RAX, Four, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 253, 206, 16, 44, 128], OperandSize::Qword)
}

#[test]
fn vmovupd_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 16, 245], OperandSize::Dword)
}

#[test]
fn vmovupd_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPD, operand1: Some(IndirectScaledIndexedDisplaced(ECX, ECX, Eight, 528231, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 17, 140, 201, 103, 15, 8, 0], OperandSize::Dword)
}

#[test]
fn vmovupd_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 16, 201], OperandSize::Qword)
}

#[test]
fn vmovupd_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPD, operand1: Some(IndirectDisplaced(RCX, 816635101, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 17, 177, 221, 220, 172, 48], OperandSize::Qword)
}

#[test]
fn vmovupd_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 16, 253], OperandSize::Dword)
}

#[test]
fn vmovupd_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPD, operand1: Some(IndirectScaledIndexedDisplaced(EDI, ECX, Eight, 918368094, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(YMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 17, 180, 207, 94, 47, 189, 54], OperandSize::Dword)
}

#[test]
fn vmovupd_27() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPD, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 16, 213], OperandSize::Qword)
}

#[test]
fn vmovupd_28() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPD, operand1: Some(IndirectScaledIndexed(RDI, RDI, Four, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(YMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 17, 12, 191], OperandSize::Qword)
}

#[test]
fn vmovupd_29() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 253, 142, 16, 202], OperandSize::Dword)
}

#[test]
fn vmovupd_30() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPD, operand1: Some(IndirectScaledIndexed(ECX, EDX, Two, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 17, 44, 81], OperandSize::Dword)
}

#[test]
fn vmovupd_31() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPD, operand1: Some(Direct(XMM10)), operand2: Some(Direct(XMM15)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 81, 253, 139, 16, 215], OperandSize::Qword)
}

#[test]
fn vmovupd_32() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPD, operand1: Some(IndirectScaledIndexedDisplaced(RSI, RCX, Eight, 1075646355, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM29)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 97, 253, 8, 17, 172, 206, 147, 15, 29, 64], OperandSize::Qword)
}

#[test]
fn vmovupd_33() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPD, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 253, 174, 16, 195], OperandSize::Dword)
}

#[test]
fn vmovupd_34() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPD, operand1: Some(IndirectDisplaced(ESI, 145388535, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(YMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 17, 190, 247, 115, 170, 8], OperandSize::Dword)
}

#[test]
fn vmovupd_35() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPD, operand1: Some(Direct(YMM16)), operand2: Some(Direct(YMM18)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 161, 253, 173, 16, 194], OperandSize::Qword)
}

#[test]
fn vmovupd_36() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPD, operand1: Some(IndirectScaledIndexedDisplaced(RSI, RDI, Two, 496013539, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(YMM27)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 97, 253, 40, 17, 156, 126, 227, 144, 144, 29], OperandSize::Qword)
}

#[test]
fn vmovupd_37() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPD, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 253, 207, 16, 193], OperandSize::Dword)
}

#[test]
fn vmovupd_38() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPD, operand1: Some(IndirectDisplaced(EDX, 1032981540, Some(OperandSize::Zmmword), None)), operand2: Some(Direct(ZMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 253, 72, 17, 170, 36, 12, 146, 61], OperandSize::Dword)
}

#[test]
fn vmovupd_39() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPD, operand1: Some(Direct(ZMM11)), operand2: Some(Direct(ZMM24)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 17, 253, 207, 16, 216], OperandSize::Qword)
}

#[test]
fn vmovupd_40() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVUPD, operand1: Some(Indirect(RDX, Some(OperandSize::Zmmword), None)), operand2: Some(Direct(ZMM16)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 225, 253, 72, 17, 2], OperandSize::Qword)
}

