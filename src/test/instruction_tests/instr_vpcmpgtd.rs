use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpcmpgtd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 225, 102, 227], OperandSize::Dword)
}

#[test]
fn vpcmpgtd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM5)), operand3: Some(Indirect(EAX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 209, 102, 40], OperandSize::Dword)
}

#[test]
fn vpcmpgtd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 241, 102, 242], OperandSize::Qword)
}

#[test]
fn vpcmpgtd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledIndexedDisplaced(RAX, RSI, Two, 2017144095, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 102, 148, 112, 31, 45, 59, 120], OperandSize::Qword)
}

#[test]
fn vpcmpgtd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTD, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 197, 102, 228], OperandSize::Dword)
}

#[test]
fn vpcmpgtd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTD, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectScaledDisplaced(ECX, Two, 1045566831, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 213, 102, 4, 77, 111, 21, 82, 62], OperandSize::Dword)
}

#[test]
fn vpcmpgtd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTD, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 237, 102, 225], OperandSize::Qword)
}

#[test]
fn vpcmpgtd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectScaledDisplaced(RSI, Eight, 1479133364, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 221, 102, 60, 245, 180, 200, 41, 88], OperandSize::Qword)
}

#[test]
fn vpcmpgtd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTD, operand1: Some(Direct(K7)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 117, 12, 102, 248], OperandSize::Dword)
}

#[test]
fn vpcmpgtd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTD, operand1: Some(Direct(K1)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectScaledDisplaced(EDI, Two, 1624415075, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 117, 15, 102, 12, 125, 99, 155, 210, 96], OperandSize::Dword)
}

#[test]
fn vpcmpgtd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTD, operand1: Some(Direct(K1)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectDisplaced(EBX, 1299023920, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 77, 30, 102, 139, 48, 136, 109, 77], OperandSize::Dword)
}

#[test]
fn vpcmpgtd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTD, operand1: Some(Direct(K7)), operand2: Some(Direct(XMM24)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 61, 3, 102, 255], OperandSize::Qword)
}

#[test]
fn vpcmpgtd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTD, operand1: Some(Direct(K5)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledIndexedDisplaced(RBX, RSI, Four, 1510729265, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 125, 13, 102, 172, 179, 49, 230, 11, 90], OperandSize::Qword)
}

#[test]
fn vpcmpgtd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTD, operand1: Some(Direct(K1)), operand2: Some(Direct(XMM26)), operand3: Some(IndirectScaledDisplaced(RBX, Eight, 2001361797, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 45, 20, 102, 12, 221, 133, 91, 74, 119], OperandSize::Qword)
}

#[test]
fn vpcmpgtd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTD, operand1: Some(Direct(K7)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 109, 42, 102, 248], OperandSize::Dword)
}

#[test]
fn vpcmpgtd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTD, operand1: Some(Direct(K1)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectScaledIndexedDisplaced(ESI, EDX, Eight, 992731531, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 69, 47, 102, 140, 214, 139, 225, 43, 59], OperandSize::Dword)
}

#[test]
fn vpcmpgtd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTD, operand1: Some(Direct(K2)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectScaledIndexedDisplaced(EBX, EDX, Four, 1440091270, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 93, 59, 102, 148, 147, 134, 12, 214, 85], OperandSize::Dword)
}

#[test]
fn vpcmpgtd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTD, operand1: Some(Direct(K7)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(YMM23)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 177, 77, 44, 102, 255], OperandSize::Qword)
}

#[test]
fn vpcmpgtd_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTD, operand1: Some(Direct(K5)), operand2: Some(Direct(YMM9)), operand3: Some(IndirectScaledDisplaced(RCX, Four, 367902708, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 53, 42, 102, 44, 141, 244, 191, 237, 21], OperandSize::Qword)
}

#[test]
fn vpcmpgtd_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTD, operand1: Some(Direct(K2)), operand2: Some(Direct(YMM19)), operand3: Some(Indirect(RAX, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 101, 51, 102, 16], OperandSize::Qword)
}

#[test]
fn vpcmpgtd_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTD, operand1: Some(Direct(K3)), operand2: Some(Direct(ZMM7)), operand3: Some(Direct(ZMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 69, 75, 102, 220], OperandSize::Dword)
}

#[test]
fn vpcmpgtd_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTD, operand1: Some(Direct(K2)), operand2: Some(Direct(ZMM1)), operand3: Some(Indirect(ECX, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 117, 74, 102, 17], OperandSize::Dword)
}

#[test]
fn vpcmpgtd_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTD, operand1: Some(Direct(K4)), operand2: Some(Direct(ZMM6)), operand3: Some(IndirectScaledDisplaced(EDI, Two, 1896945761, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 241, 77, 91, 102, 36, 125, 97, 24, 17, 113], OperandSize::Dword)
}

#[test]
fn vpcmpgtd_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTD, operand1: Some(Direct(K4)), operand2: Some(Direct(ZMM28)), operand3: Some(Direct(ZMM9)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 209, 29, 70, 102, 225], OperandSize::Qword)
}

#[test]
fn vpcmpgtd_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTD, operand1: Some(Direct(K2)), operand2: Some(Direct(ZMM19)), operand3: Some(IndirectScaledIndexedDisplaced(RDI, RDI, Four, 317605505, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 101, 69, 102, 148, 191, 129, 70, 238, 18], OperandSize::Qword)
}

#[test]
fn vpcmpgtd_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTD, operand1: Some(Direct(K1)), operand2: Some(Direct(ZMM24)), operand3: Some(IndirectScaledDisplaced(RAX, Eight, 680489755, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 241, 61, 81, 102, 12, 197, 27, 115, 143, 40], OperandSize::Qword)
}

