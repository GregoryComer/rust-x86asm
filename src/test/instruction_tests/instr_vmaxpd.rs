use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vmaxpd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VMAXPD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 95, 230], OperandSize::Dword)
}

#[test]
fn vmaxpd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VMAXPD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledIndexedDisplaced(EBX, EBX, Eight, 1498980265, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 217, 95, 172, 219, 169, 159, 88, 89], OperandSize::Dword)
}

#[test]
fn vmaxpd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VMAXPD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 217, 95, 203], OperandSize::Qword)
}

#[test]
fn vmaxpd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VMAXPD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM1)), operand3: Some(Indirect(RBX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 241, 95, 3], OperandSize::Qword)
}

#[test]
fn vmaxpd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VMAXPD, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(YMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 95, 223], OperandSize::Dword)
}

#[test]
fn vmaxpd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VMAXPD, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM7)), operand3: Some(Indirect(EAX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 197, 95, 8], OperandSize::Dword)
}

#[test]
fn vmaxpd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VMAXPD, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(YMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 205, 95, 226], OperandSize::Qword)
}

#[test]
fn vmaxpd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VMAXPD, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectScaledIndexed(RBX, RBX, Two, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 197, 95, 36, 91], OperandSize::Qword)
}

#[test]
fn vmaxpd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VMAXPD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 205, 140, 95, 227], OperandSize::Dword)
}

#[test]
fn vmaxpd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VMAXPD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledIndexedDisplaced(ECX, ESI, Eight, 1773624460, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 221, 143, 95, 156, 241, 140, 92, 183, 105], OperandSize::Dword)
}

#[test]
fn vmaxpd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VMAXPD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM7)), operand3: Some(Indirect(EDI, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 241, 197, 154, 95, 31], OperandSize::Dword)
}

#[test]
fn vmaxpd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VMAXPD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 197, 142, 95, 225], OperandSize::Qword)
}

#[test]
fn vmaxpd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VMAXPD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM15)), operand3: Some(IndirectScaledDisplaced(RDI, Eight, 1757471764, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 133, 141, 95, 52, 253, 20, 228, 192, 104], OperandSize::Qword)
}

#[test]
fn vmaxpd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VMAXPD, operand1: Some(Direct(XMM11)), operand2: Some(Direct(XMM30)), operand3: Some(IndirectScaledIndexed(RBX, RCX, Eight, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 113, 141, 147, 95, 28, 203], OperandSize::Qword)
}

#[test]
fn vmaxpd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VMAXPD, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 221, 169, 95, 218], OperandSize::Dword)
}

#[test]
fn vmaxpd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VMAXPD, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectScaledDisplaced(EAX, Four, 982532889, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 253, 169, 95, 36, 133, 25, 67, 144, 58], OperandSize::Dword)
}

#[test]
fn vmaxpd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VMAXPD, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectScaledIndexed(EDI, EDX, Eight, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 229, 191, 95, 36, 215], OperandSize::Dword)
}

#[test]
fn vmaxpd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VMAXPD, operand1: Some(Direct(YMM31)), operand2: Some(Direct(YMM9)), operand3: Some(Direct(YMM8)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 65, 181, 172, 95, 248], OperandSize::Qword)
}

#[test]
fn vmaxpd_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VMAXPD, operand1: Some(Direct(YMM19)), operand2: Some(Direct(YMM13)), operand3: Some(IndirectDisplaced(RAX, 592794380, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 225, 149, 170, 95, 152, 12, 83, 85, 35], OperandSize::Qword)
}

#[test]
fn vmaxpd_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VMAXPD, operand1: Some(Direct(YMM23)), operand2: Some(Direct(YMM26)), operand3: Some(IndirectDisplaced(RDX, 1450202618, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 225, 173, 183, 95, 186, 250, 85, 112, 86], OperandSize::Qword)
}

#[test]
fn vmaxpd_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VMAXPD, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM0)), operand3: Some(Direct(ZMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 253, 153, 95, 203], OperandSize::Dword)
}

#[test]
fn vmaxpd_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VMAXPD, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM2)), operand3: Some(IndirectDisplaced(EAX, 849170183, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 237, 204, 95, 152, 7, 79, 157, 50], OperandSize::Dword)
}

#[test]
fn vmaxpd_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VMAXPD, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM2)), operand3: Some(IndirectScaledIndexedDisplaced(EDX, EAX, Four, 447382185, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 237, 219, 95, 180, 130, 169, 130, 170, 26], OperandSize::Dword)
}

#[test]
fn vmaxpd_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VMAXPD, operand1: Some(Direct(ZMM28)), operand2: Some(Direct(ZMM2)), operand3: Some(Direct(ZMM31)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K1), broadcast: None }, &[98, 1, 237, 153, 95, 231], OperandSize::Qword)
}

#[test]
fn vmaxpd_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VMAXPD, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM31)), operand3: Some(IndirectScaledDisplaced(RBX, Eight, 1372597807, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 133, 198, 95, 28, 221, 47, 46, 208, 81], OperandSize::Qword)
}

#[test]
fn vmaxpd_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VMAXPD, operand1: Some(Direct(ZMM19)), operand2: Some(Direct(ZMM31)), operand3: Some(Indirect(RDI, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 225, 133, 211, 95, 31], OperandSize::Qword)
}

