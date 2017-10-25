use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vunpcklps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKLPS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 20, 204], OperandSize::Dword)
}

#[test]
fn vunpcklps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKLPS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM7)), operand3: Some(Indirect(EAX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 192, 20, 16], OperandSize::Dword)
}

#[test]
fn vunpcklps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKLPS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 200, 20, 237], OperandSize::Qword)
}

#[test]
fn vunpcklps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKLPS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledDisplaced(RDI, Eight, 321183578, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 20, 52, 253, 90, 223, 36, 19], OperandSize::Qword)
}

#[test]
fn vunpcklps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKLPS, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(YMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 204, 20, 202], OperandSize::Dword)
}

#[test]
fn vunpcklps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKLPS, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectScaledIndexed(ECX, EDI, Four, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 236, 20, 4, 185], OperandSize::Dword)
}

#[test]
fn vunpcklps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKLPS, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 220, 20, 216], OperandSize::Qword)
}

#[test]
fn vunpcklps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKLPS, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectScaledIndexed(RCX, RDX, Eight, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 236, 20, 20, 209], OperandSize::Qword)
}

#[test]
fn vunpcklps_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKLPS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 68, 143, 20, 228], OperandSize::Dword)
}

#[test]
fn vunpcklps_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKLPS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledIndexedDisplaced(EDX, EDX, Eight, 460395711, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 92, 143, 20, 180, 210, 191, 20, 113, 27], OperandSize::Dword)
}

#[test]
fn vunpcklps_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKLPS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM3)), operand3: Some(Indirect(EAX, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 100, 153, 20, 40], OperandSize::Dword)
}

#[test]
fn vunpcklps_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKLPS, operand1: Some(Direct(XMM13)), operand2: Some(Direct(XMM16)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 113, 124, 130, 20, 237], OperandSize::Qword)
}

#[test]
fn vunpcklps_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKLPS, operand1: Some(Direct(XMM10)), operand2: Some(Direct(XMM31)), operand3: Some(IndirectDisplaced(RCX, 907883552, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 113, 4, 131, 20, 145, 32, 52, 29, 54], OperandSize::Qword)
}

#[test]
fn vunpcklps_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKLPS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM30)), operand3: Some(IndirectScaledIndexedDisplaced(RSI, RAX, Eight, 1691628163, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 12, 151, 20, 132, 198, 131, 50, 212, 100], OperandSize::Qword)
}

#[test]
fn vunpcklps_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKLPS, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(YMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 84, 175, 20, 216], OperandSize::Dword)
}

#[test]
fn vunpcklps_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKLPS, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectScaledIndexedDisplaced(EDI, ECX, Eight, 110364018, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 84, 172, 20, 156, 207, 114, 5, 148, 6], OperandSize::Dword)
}

#[test]
fn vunpcklps_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKLPS, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM4)), operand3: Some(Indirect(EDX, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 92, 186, 20, 50], OperandSize::Dword)
}

#[test]
fn vunpcklps_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKLPS, operand1: Some(Direct(YMM24)), operand2: Some(Direct(YMM11)), operand3: Some(Direct(YMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 97, 36, 175, 20, 192], OperandSize::Qword)
}

#[test]
fn vunpcklps_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKLPS, operand1: Some(Direct(YMM8)), operand2: Some(Direct(YMM11)), operand3: Some(Indirect(RSI, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 113, 36, 172, 20, 6], OperandSize::Qword)
}

#[test]
fn vunpcklps_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKLPS, operand1: Some(Direct(YMM30)), operand2: Some(Direct(YMM27)), operand3: Some(IndirectScaledIndexed(RDI, RAX, Eight, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 97, 36, 178, 20, 52, 199], OperandSize::Qword)
}

#[test]
fn vunpcklps_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKLPS, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM7)), operand3: Some(Direct(ZMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 68, 207, 20, 217], OperandSize::Dword)
}

#[test]
fn vunpcklps_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKLPS, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM2)), operand3: Some(IndirectScaledIndexedDisplaced(ESI, ECX, Eight, 1320417097, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 108, 202, 20, 172, 206, 73, 247, 179, 78], OperandSize::Dword)
}

#[test]
fn vunpcklps_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKLPS, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM2)), operand3: Some(IndirectScaledIndexedDisplaced(EAX, ESI, Four, 1913426055, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 241, 108, 218, 20, 164, 176, 135, 144, 12, 114], OperandSize::Dword)
}

#[test]
fn vunpcklps_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKLPS, operand1: Some(Direct(ZMM28)), operand2: Some(Direct(ZMM5)), operand3: Some(Direct(ZMM19)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 33, 84, 205, 20, 227], OperandSize::Qword)
}

#[test]
fn vunpcklps_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKLPS, operand1: Some(Direct(ZMM8)), operand2: Some(Direct(ZMM10)), operand3: Some(IndirectDisplaced(RBX, 985256943, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 113, 44, 204, 20, 131, 239, 211, 185, 58], OperandSize::Qword)
}

#[test]
fn vunpcklps_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKLPS, operand1: Some(Direct(ZMM16)), operand2: Some(Direct(ZMM6)), operand3: Some(IndirectScaledIndexedDisplaced(RDI, RCX, Four, 1981449281, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 225, 76, 219, 20, 132, 143, 65, 132, 26, 118], OperandSize::Qword)
}

