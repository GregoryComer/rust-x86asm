use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vfmaddsub231pd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB231PD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 233, 182, 251], OperandSize::Dword)
}

#[test]
fn vfmaddsub231pd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB231PD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectDisplaced(ESI, 1676074904, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 241, 182, 166, 152, 223, 230, 99], OperandSize::Dword)
}

#[test]
fn vfmaddsub231pd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB231PD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 193, 182, 229], OperandSize::Qword)
}

#[test]
fn vfmaddsub231pd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB231PD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledDisplaced(RBX, Two, 934362488, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 225, 182, 28, 93, 120, 61, 177, 55], OperandSize::Qword)
}

#[test]
fn vfmaddsub231pd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB231PD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(YMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 213, 182, 233], OperandSize::Dword)
}

#[test]
fn vfmaddsub231pd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB231PD, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectScaledDisplaced(EDX, Two, 2080328207, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 221, 182, 20, 85, 15, 74, 255, 123], OperandSize::Dword)
}

#[test]
fn vfmaddsub231pd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB231PD, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 245, 182, 210], OperandSize::Qword)
}

#[test]
fn vfmaddsub231pd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB231PD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectDisplaced(RSI, 327632307, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 213, 182, 174, 179, 69, 135, 19], OperandSize::Qword)
}

#[test]
fn vfmaddsub231pd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB231PD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 229, 138, 182, 239], OperandSize::Dword)
}

#[test]
fn vfmaddsub231pd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB231PD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledDisplaced(ECX, Two, 216258388, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 237, 141, 182, 44, 77, 84, 215, 227, 12], OperandSize::Dword)
}

#[test]
fn vfmaddsub231pd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB231PD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledIndexedDisplaced(EDX, ECX, Four, 989737074, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 242, 221, 153, 182, 164, 138, 114, 48, 254, 58], OperandSize::Dword)
}

#[test]
fn vfmaddsub231pd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB231PD, operand1: Some(Direct(XMM13)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM28)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 18, 253, 137, 182, 236], OperandSize::Qword)
}

#[test]
fn vfmaddsub231pd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB231PD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledDisplaced(RSI, Eight, 1597253831, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 197, 137, 182, 44, 245, 199, 40, 52, 95], OperandSize::Qword)
}

#[test]
fn vfmaddsub231pd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB231PD, operand1: Some(Direct(XMM13)), operand2: Some(Direct(XMM21)), operand3: Some(Indirect(RSI, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 114, 213, 149, 182, 46], OperandSize::Qword)
}

#[test]
fn vfmaddsub231pd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB231PD, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(YMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 205, 169, 182, 208], OperandSize::Dword)
}

#[test]
fn vfmaddsub231pd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB231PD, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM3)), operand3: Some(Indirect(EDX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 229, 169, 182, 2], OperandSize::Dword)
}

#[test]
fn vfmaddsub231pd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB231PD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectScaledDisplaced(ESI, Eight, 801954172, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 213, 190, 182, 60, 245, 124, 217, 204, 47], OperandSize::Dword)
}

#[test]
fn vfmaddsub231pd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB231PD, operand1: Some(Direct(YMM18)), operand2: Some(Direct(YMM15)), operand3: Some(Direct(YMM25)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 130, 133, 169, 182, 209], OperandSize::Qword)
}

#[test]
fn vfmaddsub231pd_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB231PD, operand1: Some(Direct(YMM8)), operand2: Some(Direct(YMM28)), operand3: Some(IndirectScaledIndexedDisplaced(RDX, RDI, Four, 584597786, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 114, 157, 161, 182, 132, 186, 26, 65, 216, 34], OperandSize::Qword)
}

#[test]
fn vfmaddsub231pd_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB231PD, operand1: Some(Direct(YMM31)), operand2: Some(Direct(YMM17)), operand3: Some(IndirectScaledIndexedDisplaced(RCX, RCX, Two, 1962788492, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 98, 245, 183, 182, 188, 73, 140, 198, 253, 116], OperandSize::Qword)
}

#[test]
fn vfmaddsub231pd_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB231PD, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM1)), operand3: Some(Direct(ZMM3)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Up), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 245, 223, 182, 219], OperandSize::Dword)
}

#[test]
fn vfmaddsub231pd_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB231PD, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM5)), operand3: Some(IndirectScaledIndexed(ESI, EDI, Four, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 213, 201, 182, 60, 190], OperandSize::Dword)
}

#[test]
fn vfmaddsub231pd_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB231PD, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM0)), operand3: Some(Indirect(EDI, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 253, 218, 182, 23], OperandSize::Dword)
}

#[test]
fn vfmaddsub231pd_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB231PD, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM0)), operand3: Some(Direct(ZMM28)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Up), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 146, 253, 220, 182, 204], OperandSize::Qword)
}

#[test]
fn vfmaddsub231pd_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB231PD, operand1: Some(Direct(ZMM10)), operand2: Some(Direct(ZMM28)), operand3: Some(IndirectDisplaced(RAX, 105145960, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 114, 157, 195, 182, 144, 104, 102, 68, 6], OperandSize::Qword)
}

#[test]
fn vfmaddsub231pd_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB231PD, operand1: Some(Direct(ZMM11)), operand2: Some(Direct(ZMM31)), operand3: Some(IndirectScaledDisplaced(RCX, Eight, 1705990946, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 114, 133, 210, 182, 28, 205, 34, 91, 175, 101], OperandSize::Qword)
}

