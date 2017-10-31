use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vminps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VMINPS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 216, 93, 227], OperandSize::Dword)
}

#[test]
fn vminps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VMINPS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM4)), operand3: Some(Indirect(EDX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 216, 93, 50], OperandSize::Dword)
}

#[test]
fn vminps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VMINPS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 240, 93, 197], OperandSize::Qword)
}

#[test]
fn vminps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VMINPS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledIndexedDisplaced(RCX, RDI, Eight, 817953942, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 224, 93, 172, 249, 150, 252, 192, 48], OperandSize::Qword)
}

#[test]
fn vminps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VMINPS, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 220, 93, 212], OperandSize::Dword)
}

#[test]
fn vminps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VMINPS, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectScaledIndexedDisplaced(EDI, EBX, Two, 520237541, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 220, 93, 148, 95, 229, 49, 2, 31], OperandSize::Dword)
}

#[test]
fn vminps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VMINPS, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 236, 93, 218], OperandSize::Qword)
}

#[test]
fn vminps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VMINPS, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectScaledDisplaced(RDI, Two, 1374381921, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 196, 93, 28, 125, 97, 103, 235, 81], OperandSize::Qword)
}

#[test]
fn vminps_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VMINPS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 92, 139, 93, 228], OperandSize::Dword)
}

#[test]
fn vminps_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VMINPS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledIndexed(EBX, EDX, Eight, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 92, 139, 93, 12, 211], OperandSize::Dword)
}

#[test]
fn vminps_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VMINPS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledIndexedDisplaced(ECX, EDI, Two, 218815103, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 92, 153, 93, 140, 121, 127, 218, 10, 13], OperandSize::Dword)
}

#[test]
fn vminps_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VMINPS, operand1: Some(Direct(XMM16)), operand2: Some(Direct(XMM8)), operand3: Some(Direct(XMM12)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 193, 60, 141, 93, 196], OperandSize::Qword)
}

#[test]
fn vminps_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VMINPS, operand1: Some(Direct(XMM14)), operand2: Some(Direct(XMM16)), operand3: Some(IndirectScaledDisplaced(RAX, Eight, 1033337915, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 113, 124, 133, 93, 52, 197, 59, 124, 151, 61], OperandSize::Qword)
}

#[test]
fn vminps_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VMINPS, operand1: Some(Direct(XMM25)), operand2: Some(Direct(XMM20)), operand3: Some(Indirect(RCX, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 97, 92, 151, 93, 9], OperandSize::Qword)
}

#[test]
fn vminps_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VMINPS, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 92, 169, 93, 230], OperandSize::Dword)
}

#[test]
fn vminps_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VMINPS, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectScaledIndexedDisplaced(EBX, EDX, Four, 365465692, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 68, 169, 93, 140, 147, 92, 144, 200, 21], OperandSize::Dword)
}

#[test]
fn vminps_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VMINPS, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectDisplaced(EAX, 1827141842, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 124, 185, 93, 128, 210, 248, 231, 108], OperandSize::Dword)
}

#[test]
fn vminps_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VMINPS, operand1: Some(Direct(YMM11)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(YMM29)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 17, 100, 169, 93, 221], OperandSize::Qword)
}

#[test]
fn vminps_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VMINPS, operand1: Some(Direct(YMM23)), operand2: Some(Direct(YMM30)), operand3: Some(IndirectScaledDisplaced(RDX, Eight, 921899730, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 225, 12, 161, 93, 60, 213, 210, 18, 243, 54], OperandSize::Qword)
}

#[test]
fn vminps_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VMINPS, operand1: Some(Direct(YMM21)), operand2: Some(Direct(YMM8)), operand3: Some(IndirectScaledIndexedDisplaced(RBX, RCX, Two, 1427090229, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 225, 60, 186, 93, 172, 75, 53, 171, 15, 85], OperandSize::Qword)
}

#[test]
fn vminps_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VMINPS, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM1)), operand3: Some(Direct(ZMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 116, 158, 93, 209], OperandSize::Dword)
}

#[test]
fn vminps_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VMINPS, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM6)), operand3: Some(Indirect(ESI, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 76, 203, 93, 22], OperandSize::Dword)
}

#[test]
fn vminps_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VMINPS, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM3)), operand3: Some(IndirectDisplaced(EAX, 2072062080, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 241, 100, 220, 93, 144, 128, 40, 129, 123], OperandSize::Dword)
}

#[test]
fn vminps_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VMINPS, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM28)), operand3: Some(Direct(ZMM18)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K2), broadcast: None }, &[98, 177, 28, 146, 93, 234], OperandSize::Qword)
}

#[test]
fn vminps_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VMINPS, operand1: Some(Direct(ZMM11)), operand2: Some(Direct(ZMM9)), operand3: Some(IndirectScaledIndexedDisplaced(RCX, RCX, Two, 1951846061, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 113, 52, 202, 93, 156, 73, 173, 206, 86, 116], OperandSize::Qword)
}

#[test]
fn vminps_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VMINPS, operand1: Some(Direct(ZMM23)), operand2: Some(Direct(ZMM5)), operand3: Some(IndirectScaledDisplaced(RDI, Eight, 738494167, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 225, 84, 217, 93, 60, 253, 215, 134, 4, 44], OperandSize::Qword)
}

