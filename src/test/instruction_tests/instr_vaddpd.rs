use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vaddpd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDPD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 233, 88, 222], OperandSize::Dword)
}

#[test]
fn vaddpd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDPD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectDisplaced(EDX, 1153587631, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 201, 88, 146, 175, 89, 194, 68], OperandSize::Dword)
}

#[test]
fn vaddpd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDPD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 225, 88, 246], OperandSize::Qword)
}

#[test]
fn vaddpd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDPD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectScaledDisplaced(RBX, Two, 1512744516, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 241, 88, 52, 93, 68, 166, 42, 90], OperandSize::Qword)
}

#[test]
fn vaddpd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDPD, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(YMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 88, 192], OperandSize::Dword)
}

#[test]
fn vaddpd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDPD, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM3)), operand3: Some(Indirect(EDX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 229, 88, 2], OperandSize::Dword)
}

#[test]
fn vaddpd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDPD, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(YMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 229, 88, 224], OperandSize::Qword)
}

#[test]
fn vaddpd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDPD, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectScaledIndexedDisplaced(RBX, RAX, Two, 932035641, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 213, 88, 164, 67, 57, 188, 141, 55], OperandSize::Qword)
}

#[test]
fn vaddpd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDPD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 229, 138, 88, 245], OperandSize::Dword)
}

#[test]
fn vaddpd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDPD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledIndexedDisplaced(EDI, ECX, Four, 505406389, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 213, 141, 88, 148, 143, 181, 227, 31, 30], OperandSize::Dword)
}

#[test]
fn vaddpd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDPD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledIndexedDisplaced(EDX, ESI, Four, 985352874, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 241, 229, 159, 88, 156, 178, 170, 74, 187, 58], OperandSize::Dword)
}

#[test]
fn vaddpd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDPD, operand1: Some(Direct(XMM18)), operand2: Some(Direct(XMM29)), operand3: Some(Direct(XMM22)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 161, 149, 130, 88, 214], OperandSize::Qword)
}

#[test]
fn vaddpd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDPD, operand1: Some(Direct(XMM11)), operand2: Some(Direct(XMM28)), operand3: Some(IndirectDisplaced(RDI, 1808501287, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 113, 157, 133, 88, 159, 39, 138, 203, 107], OperandSize::Qword)
}

#[test]
fn vaddpd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDPD, operand1: Some(Direct(XMM21)), operand2: Some(Direct(XMM14)), operand3: Some(IndirectDisplaced(RSI, 401774069, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 225, 141, 155, 88, 174, 245, 149, 242, 23], OperandSize::Qword)
}

#[test]
fn vaddpd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDPD, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 245, 169, 88, 208], OperandSize::Dword)
}

#[test]
fn vaddpd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDPD, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectScaledIndexedDisplaced(EAX, EBX, Two, 2083035409, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 237, 174, 88, 164, 88, 17, 153, 40, 124], OperandSize::Dword)
}

#[test]
fn vaddpd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDPD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectDisplaced(EDI, 1313141907, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 253, 191, 88, 175, 147, 244, 68, 78], OperandSize::Dword)
}

#[test]
fn vaddpd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDPD, operand1: Some(Direct(YMM26)), operand2: Some(Direct(YMM26)), operand3: Some(Direct(YMM10)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 65, 173, 163, 88, 210], OperandSize::Qword)
}

#[test]
fn vaddpd_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDPD, operand1: Some(Direct(YMM12)), operand2: Some(Direct(YMM1)), operand3: Some(Indirect(RAX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 113, 245, 172, 88, 32], OperandSize::Qword)
}

#[test]
fn vaddpd_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDPD, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM8)), operand3: Some(IndirectScaledDisplaced(RCX, Two, 459978429, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 189, 188, 88, 12, 77, 189, 182, 106, 27], OperandSize::Qword)
}

#[test]
fn vaddpd_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDPD, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM2)), operand3: Some(Direct(ZMM6)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Nearest), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 237, 158, 88, 222], OperandSize::Dword)
}

#[test]
fn vaddpd_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDPD, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM3)), operand3: Some(IndirectScaledDisplaced(EBX, Eight, 75360757, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 229, 201, 88, 60, 221, 245, 233, 125, 4], OperandSize::Dword)
}

#[test]
fn vaddpd_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDPD, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM5)), operand3: Some(IndirectScaledIndexed(ESI, ESI, Eight, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 213, 218, 88, 60, 246], OperandSize::Dword)
}

#[test]
fn vaddpd_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDPD, operand1: Some(Direct(ZMM8)), operand2: Some(Direct(ZMM11)), operand3: Some(Direct(ZMM16)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Zero), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 49, 165, 253, 88, 192], OperandSize::Qword)
}

#[test]
fn vaddpd_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDPD, operand1: Some(Direct(ZMM8)), operand2: Some(Direct(ZMM23)), operand3: Some(IndirectScaledIndexed(RAX, RCX, Two, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 113, 197, 196, 88, 4, 72], OperandSize::Qword)
}

#[test]
fn vaddpd_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDPD, operand1: Some(Direct(ZMM23)), operand2: Some(Direct(ZMM28)), operand3: Some(IndirectScaledIndexedDisplaced(RBX, RCX, Four, 1788261499, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 225, 157, 211, 88, 188, 139, 123, 180, 150, 106], OperandSize::Qword)
}

