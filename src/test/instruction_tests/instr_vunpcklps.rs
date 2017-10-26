use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vunpcklps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKLPS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 20, 254], OperandSize::Dword)
}

#[test]
fn vunpcklps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKLPS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectScaledDisplaced(EBX, Two, 967694450, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 240, 20, 36, 93, 114, 216, 173, 57], OperandSize::Dword)
}

#[test]
fn vunpcklps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKLPS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 208, 20, 255], OperandSize::Qword)
}

#[test]
fn vunpcklps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKLPS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM5)), operand3: Some(Indirect(RDX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 208, 20, 18], OperandSize::Qword)
}

#[test]
fn vunpcklps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKLPS, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 196, 20, 226], OperandSize::Dword)
}

#[test]
fn vunpcklps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKLPS, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectScaledIndexedDisplaced(ESI, ECX, Eight, 1277221757, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 252, 20, 132, 206, 125, 219, 32, 76], OperandSize::Dword)
}

#[test]
fn vunpcklps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKLPS, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(YMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 252, 20, 226], OperandSize::Qword)
}

#[test]
fn vunpcklps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKLPS, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectScaledIndexed(RBX, RCX, Two, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 196, 20, 20, 75], OperandSize::Qword)
}

#[test]
fn vunpcklps_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKLPS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 124, 139, 20, 222], OperandSize::Dword)
}

#[test]
fn vunpcklps_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKLPS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectDisplaced(EAX, 1897937194, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 92, 140, 20, 184, 42, 57, 32, 113], OperandSize::Dword)
}

#[test]
fn vunpcklps_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKLPS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM1)), operand3: Some(Indirect(EAX, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 116, 155, 20, 56], OperandSize::Dword)
}

#[test]
fn vunpcklps_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKLPS, operand1: Some(Direct(XMM18)), operand2: Some(Direct(XMM27)), operand3: Some(Direct(XMM24)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 129, 36, 129, 20, 208], OperandSize::Qword)
}

#[test]
fn vunpcklps_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKLPS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM30)), operand3: Some(IndirectDisplaced(RBX, 2092105401, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 12, 129, 20, 139, 185, 254, 178, 124], OperandSize::Qword)
}

#[test]
fn vunpcklps_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKLPS, operand1: Some(Direct(XMM30)), operand2: Some(Direct(XMM29)), operand3: Some(IndirectScaledIndexedDisplaced(RDI, RAX, Eight, 90558927, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 97, 20, 149, 20, 180, 199, 207, 209, 101, 5], OperandSize::Qword)
}

#[test]
fn vunpcklps_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKLPS, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(YMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 124, 170, 20, 210], OperandSize::Dword)
}

#[test]
fn vunpcklps_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKLPS, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectScaledDisplaced(ECX, Four, 2072928893, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 100, 169, 20, 4, 141, 125, 98, 142, 123], OperandSize::Dword)
}

#[test]
fn vunpcklps_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKLPS, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectScaledIndexed(EAX, EDX, Two, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 84, 191, 20, 36, 80], OperandSize::Dword)
}

#[test]
fn vunpcklps_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKLPS, operand1: Some(Direct(YMM20)), operand2: Some(Direct(YMM22)), operand3: Some(Direct(YMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 225, 76, 167, 20, 227], OperandSize::Qword)
}

#[test]
fn vunpcklps_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKLPS, operand1: Some(Direct(YMM21)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectScaledIndexed(RSI, RSI, Four, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 225, 116, 172, 20, 44, 182], OperandSize::Qword)
}

#[test]
fn vunpcklps_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKLPS, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM27)), operand3: Some(Indirect(RDX, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 36, 182, 20, 58], OperandSize::Qword)
}

#[test]
fn vunpcklps_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKLPS, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM7)), operand3: Some(Direct(ZMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 68, 202, 20, 241], OperandSize::Dword)
}

#[test]
fn vunpcklps_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKLPS, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM4)), operand3: Some(IndirectScaledIndexed(EBX, EBX, Two, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 92, 207, 20, 20, 91], OperandSize::Dword)
}

#[test]
fn vunpcklps_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKLPS, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM4)), operand3: Some(IndirectScaledIndexedDisplaced(ESI, ESI, Four, 748644179, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 241, 92, 221, 20, 164, 182, 83, 103, 159, 44], OperandSize::Dword)
}

#[test]
fn vunpcklps_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKLPS, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM9)), operand3: Some(Direct(ZMM24)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 145, 52, 206, 20, 248], OperandSize::Qword)
}

#[test]
fn vunpcklps_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKLPS, operand1: Some(Direct(ZMM17)), operand2: Some(Direct(ZMM11)), operand3: Some(IndirectScaledIndexed(RDX, RSI, Two, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 225, 36, 203, 20, 12, 114], OperandSize::Qword)
}

#[test]
fn vunpcklps_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKLPS, operand1: Some(Direct(ZMM23)), operand2: Some(Direct(ZMM6)), operand3: Some(IndirectDisplaced(RDX, 13691354, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 225, 76, 219, 20, 186, 218, 233, 208, 0], OperandSize::Qword)
}

