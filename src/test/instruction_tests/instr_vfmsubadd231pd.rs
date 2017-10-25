use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vfmsubadd231pd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD231PD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 209, 183, 241], OperandSize::Dword)
}

#[test]
fn vfmsubadd231pd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD231PD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledIndexed(EDX, EDI, Four, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 209, 183, 20, 186], OperandSize::Dword)
}

#[test]
fn vfmsubadd231pd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD231PD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 201, 183, 198], OperandSize::Qword)
}

#[test]
fn vfmsubadd231pd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD231PD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectScaledDisplaced(RSI, Two, 1461586844, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 241, 183, 44, 117, 156, 11, 30, 87], OperandSize::Qword)
}

#[test]
fn vfmsubadd231pd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD231PD, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(YMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 229, 183, 196], OperandSize::Dword)
}

#[test]
fn vfmsubadd231pd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD231PD, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectDisplaced(ESI, 340397426, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 229, 183, 134, 114, 13, 74, 20], OperandSize::Dword)
}

#[test]
fn vfmsubadd231pd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD231PD, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 237, 183, 213], OperandSize::Qword)
}

#[test]
fn vfmsubadd231pd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD231PD, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM1)), operand3: Some(Indirect(RSI, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 245, 183, 54], OperandSize::Qword)
}

#[test]
fn vfmsubadd231pd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD231PD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 205, 143, 183, 248], OperandSize::Dword)
}

#[test]
fn vfmsubadd231pd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD231PD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledIndexedDisplaced(ECX, EBX, Four, 274193830, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 237, 138, 183, 180, 153, 166, 221, 87, 16], OperandSize::Dword)
}

#[test]
fn vfmsubadd231pd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD231PD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectDisplaced(ESI, 550648005, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 242, 213, 154, 183, 134, 197, 56, 210, 32], OperandSize::Dword)
}

#[test]
fn vfmsubadd231pd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD231PD, operand1: Some(Direct(XMM13)), operand2: Some(Direct(XMM8)), operand3: Some(Direct(XMM23)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 50, 189, 141, 183, 239], OperandSize::Qword)
}

#[test]
fn vfmsubadd231pd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD231PD, operand1: Some(Direct(XMM17)), operand2: Some(Direct(XMM20)), operand3: Some(IndirectScaledDisplaced(RCX, Eight, 1031170117, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 226, 221, 131, 183, 12, 205, 69, 104, 118, 61], OperandSize::Qword)
}

#[test]
fn vfmsubadd231pd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD231PD, operand1: Some(Direct(XMM27)), operand2: Some(Direct(XMM18)), operand3: Some(IndirectDisplaced(RBX, 591285899, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 98, 237, 151, 183, 155, 139, 78, 62, 35], OperandSize::Qword)
}

#[test]
fn vfmsubadd231pd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD231PD, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(YMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 205, 172, 183, 225], OperandSize::Dword)
}

#[test]
fn vfmsubadd231pd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD231PD, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectScaledDisplaced(EDX, Two, 1364233719, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 229, 169, 183, 20, 85, 247, 141, 80, 81], OperandSize::Dword)
}

#[test]
fn vfmsubadd231pd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD231PD, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectScaledIndexed(EBX, ESI, Eight, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 229, 185, 183, 52, 243], OperandSize::Dword)
}

#[test]
fn vfmsubadd231pd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD231PD, operand1: Some(Direct(YMM29)), operand2: Some(Direct(YMM27)), operand3: Some(Direct(YMM8)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 66, 165, 165, 183, 232], OperandSize::Qword)
}

#[test]
fn vfmsubadd231pd_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD231PD, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM18)), operand3: Some(IndirectDisplaced(RCX, 1714583658, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 237, 161, 183, 177, 106, 120, 50, 102], OperandSize::Qword)
}

#[test]
fn vfmsubadd231pd_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD231PD, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM29)), operand3: Some(IndirectDisplaced(RDX, 1597791271, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 149, 177, 183, 178, 39, 92, 60, 95], OperandSize::Qword)
}

#[test]
fn vfmsubadd231pd_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD231PD, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM0)), operand3: Some(Direct(ZMM4)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Up), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 253, 221, 183, 236], OperandSize::Dword)
}

#[test]
fn vfmsubadd231pd_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD231PD, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM0)), operand3: Some(Indirect(EDX, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 253, 202, 183, 42], OperandSize::Dword)
}

#[test]
fn vfmsubadd231pd_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD231PD, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM7)), operand3: Some(IndirectScaledDisplaced(ESI, Two, 401089830, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 197, 223, 183, 36, 117, 38, 37, 232, 23], OperandSize::Dword)
}

#[test]
fn vfmsubadd231pd_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD231PD, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM25)), operand3: Some(Direct(ZMM9)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Up), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 210, 181, 214, 183, 201], OperandSize::Qword)
}

#[test]
fn vfmsubadd231pd_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD231PD, operand1: Some(Direct(ZMM12)), operand2: Some(Direct(ZMM9)), operand3: Some(Indirect(RDX, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 114, 181, 205, 183, 34], OperandSize::Qword)
}

#[test]
fn vfmsubadd231pd_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD231PD, operand1: Some(Direct(ZMM17)), operand2: Some(Direct(ZMM10)), operand3: Some(Indirect(RBX, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 226, 173, 221, 183, 11], OperandSize::Qword)
}

