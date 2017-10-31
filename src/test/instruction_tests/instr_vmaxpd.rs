use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vmaxpd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VMAXPD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 233, 95, 194], OperandSize::Dword)
}

#[test]
fn vmaxpd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VMAXPD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM0)), operand3: Some(Indirect(ESI, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 95, 46], OperandSize::Dword)
}

#[test]
fn vmaxpd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VMAXPD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 233, 95, 222], OperandSize::Qword)
}

#[test]
fn vmaxpd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VMAXPD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectScaledDisplaced(RDX, Eight, 1510673526, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 201, 95, 44, 213, 118, 12, 11, 90], OperandSize::Qword)
}

#[test]
fn vmaxpd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VMAXPD, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 237, 95, 221], OperandSize::Dword)
}

#[test]
fn vmaxpd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VMAXPD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectScaledIndexed(EDX, EBX, Eight, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 197, 95, 44, 218], OperandSize::Dword)
}

#[test]
fn vmaxpd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VMAXPD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 221, 95, 237], OperandSize::Qword)
}

#[test]
fn vmaxpd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VMAXPD, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectDisplaced(RAX, 637585113, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 221, 95, 152, 217, 198, 0, 38], OperandSize::Qword)
}

#[test]
fn vmaxpd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VMAXPD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 245, 140, 95, 235], OperandSize::Dword)
}

#[test]
fn vmaxpd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VMAXPD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledIndexedDisplaced(ECX, EBX, Four, 1847563412, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 229, 139, 95, 140, 153, 148, 148, 31, 110], OperandSize::Dword)
}

#[test]
fn vmaxpd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VMAXPD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledIndexedDisplaced(ESI, EDX, Four, 124114760, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 241, 253, 159, 95, 164, 150, 72, 215, 101, 7], OperandSize::Dword)
}

#[test]
fn vmaxpd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VMAXPD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM16)), operand3: Some(Direct(XMM17)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 177, 253, 135, 95, 193], OperandSize::Qword)
}

#[test]
fn vmaxpd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VMAXPD, operand1: Some(Direct(XMM14)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectDisplaced(RBX, 664504870, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 113, 229, 141, 95, 179, 38, 138, 155, 39], OperandSize::Qword)
}

#[test]
fn vmaxpd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VMAXPD, operand1: Some(Direct(XMM29)), operand2: Some(Direct(XMM9)), operand3: Some(IndirectScaledDisplaced(RDX, Two, 244338767, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 97, 181, 159, 95, 44, 85, 79, 80, 144, 14], OperandSize::Qword)
}

#[test]
fn vmaxpd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VMAXPD, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 221, 170, 95, 206], OperandSize::Dword)
}

#[test]
fn vmaxpd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VMAXPD, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM3)), operand3: Some(Indirect(EAX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 229, 175, 95, 8], OperandSize::Dword)
}

#[test]
fn vmaxpd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VMAXPD, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectScaledIndexed(EDX, ESI, Four, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 253, 191, 95, 12, 178], OperandSize::Dword)
}

#[test]
fn vmaxpd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VMAXPD, operand1: Some(Direct(YMM28)), operand2: Some(Direct(YMM18)), operand3: Some(Direct(YMM15)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 65, 237, 161, 95, 231], OperandSize::Qword)
}

#[test]
fn vmaxpd_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VMAXPD, operand1: Some(Direct(YMM27)), operand2: Some(Direct(YMM13)), operand3: Some(IndirectScaledDisplaced(RBX, Two, 26476358, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 97, 149, 173, 95, 28, 93, 70, 255, 147, 1], OperandSize::Qword)
}

#[test]
fn vmaxpd_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VMAXPD, operand1: Some(Direct(YMM19)), operand2: Some(Direct(YMM11)), operand3: Some(IndirectScaledIndexed(RAX, RDI, Four, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 225, 165, 191, 95, 28, 184], OperandSize::Qword)
}

#[test]
fn vmaxpd_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VMAXPD, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM1)), operand3: Some(Direct(ZMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 245, 157, 95, 245], OperandSize::Dword)
}

#[test]
fn vmaxpd_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VMAXPD, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM6)), operand3: Some(IndirectDisplaced(ECX, 920986256, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 205, 206, 95, 145, 144, 34, 229, 54], OperandSize::Dword)
}

#[test]
fn vmaxpd_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VMAXPD, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM2)), operand3: Some(IndirectScaledDisplaced(ECX, Two, 446366279, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 237, 222, 95, 36, 77, 71, 2, 155, 26], OperandSize::Dword)
}

#[test]
fn vmaxpd_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VMAXPD, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM12)), operand3: Some(Direct(ZMM12)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K4), broadcast: None }, &[98, 209, 157, 156, 95, 252], OperandSize::Qword)
}

#[test]
fn vmaxpd_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VMAXPD, operand1: Some(Direct(ZMM28)), operand2: Some(Direct(ZMM17)), operand3: Some(Indirect(RAX, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 97, 245, 198, 95, 32], OperandSize::Qword)
}

#[test]
fn vmaxpd_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VMAXPD, operand1: Some(Direct(ZMM13)), operand2: Some(Direct(ZMM12)), operand3: Some(Indirect(RCX, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 113, 157, 217, 95, 41], OperandSize::Qword)
}

