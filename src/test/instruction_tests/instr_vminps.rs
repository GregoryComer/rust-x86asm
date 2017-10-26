use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vminps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VMINPS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 232, 93, 225], OperandSize::Dword)
}

#[test]
fn vminps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VMINPS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectDisplaced(EAX, 1578635162, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 93, 144, 154, 15, 24, 94], OperandSize::Dword)
}

#[test]
fn vminps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VMINPS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 192, 93, 218], OperandSize::Qword)
}

#[test]
fn vminps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VMINPS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectScaledIndexed(RSI, RBX, Two, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 240, 93, 36, 94], OperandSize::Qword)
}

#[test]
fn vminps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VMINPS, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 196, 93, 211], OperandSize::Dword)
}

#[test]
fn vminps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VMINPS, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectScaledIndexed(ESI, ECX, Four, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 212, 93, 44, 142], OperandSize::Dword)
}

#[test]
fn vminps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VMINPS, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 220, 93, 227], OperandSize::Qword)
}

#[test]
fn vminps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VMINPS, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectScaledIndexedDisplaced(RAX, RCX, Two, 1608056321, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 220, 93, 188, 72, 1, 254, 216, 95], OperandSize::Qword)
}

#[test]
fn vminps_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VMINPS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 108, 139, 93, 236], OperandSize::Dword)
}

#[test]
fn vminps_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VMINPS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM4)), operand3: Some(Indirect(EDI, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 92, 141, 93, 7], OperandSize::Dword)
}

#[test]
fn vminps_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VMINPS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectDisplaced(EAX, 1405084752, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 100, 158, 93, 176, 80, 228, 191, 83], OperandSize::Dword)
}

#[test]
fn vminps_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VMINPS, operand1: Some(Direct(XMM19)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 225, 100, 141, 93, 217], OperandSize::Qword)
}

#[test]
fn vminps_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VMINPS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM28)), operand3: Some(IndirectScaledIndexed(RBX, RBX, Eight, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 28, 134, 93, 44, 219], OperandSize::Qword)
}

#[test]
fn vminps_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VMINPS, operand1: Some(Direct(XMM16)), operand2: Some(Direct(XMM23)), operand3: Some(IndirectScaledIndexed(RCX, RBX, Four, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 225, 68, 151, 93, 4, 153], OperandSize::Qword)
}

#[test]
fn vminps_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VMINPS, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(YMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 124, 170, 93, 209], OperandSize::Dword)
}

#[test]
fn vminps_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VMINPS, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectDisplaced(ECX, 1276865772, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 100, 169, 93, 153, 236, 108, 27, 76], OperandSize::Dword)
}

#[test]
fn vminps_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VMINPS, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectScaledIndexedDisplaced(EBX, ECX, Two, 2095127212, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 124, 187, 93, 164, 75, 172, 26, 225, 124], OperandSize::Dword)
}

#[test]
fn vminps_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VMINPS, operand1: Some(Direct(YMM8)), operand2: Some(Direct(YMM16)), operand3: Some(Direct(YMM29)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 17, 124, 163, 93, 197], OperandSize::Qword)
}

#[test]
fn vminps_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VMINPS, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM24)), operand3: Some(IndirectScaledIndexed(RDX, RCX, Four, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 60, 161, 93, 52, 138], OperandSize::Qword)
}

#[test]
fn vminps_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VMINPS, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM16)), operand3: Some(IndirectScaledIndexedDisplaced(RAX, RSI, Four, 304425513, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 124, 177, 93, 140, 176, 41, 42, 37, 18], OperandSize::Qword)
}

#[test]
fn vminps_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VMINPS, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM4)), operand3: Some(Direct(ZMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 92, 154, 93, 214], OperandSize::Dword)
}

#[test]
fn vminps_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VMINPS, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM2)), operand3: Some(IndirectDisplaced(ECX, 1390102696, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 108, 203, 93, 129, 168, 72, 219, 82], OperandSize::Dword)
}

#[test]
fn vminps_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VMINPS, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM2)), operand3: Some(Indirect(EDI, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 241, 108, 217, 93, 63], OperandSize::Dword)
}

#[test]
fn vminps_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VMINPS, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM30)), operand3: Some(Direct(ZMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 12, 148, 93, 200], OperandSize::Qword)
}

#[test]
fn vminps_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VMINPS, operand1: Some(Direct(ZMM23)), operand2: Some(Direct(ZMM26)), operand3: Some(Indirect(RDX, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 225, 44, 193, 93, 58], OperandSize::Qword)
}

#[test]
fn vminps_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VMINPS, operand1: Some(Direct(ZMM19)), operand2: Some(Direct(ZMM19)), operand3: Some(IndirectScaledDisplaced(RDX, Eight, 1060249567, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 225, 100, 209, 93, 28, 213, 223, 31, 50, 63], OperandSize::Qword)
}

