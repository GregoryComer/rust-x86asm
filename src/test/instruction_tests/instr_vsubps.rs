use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vsubps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VSUBPS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 208, 92, 213], OperandSize::Dword)
}

#[test]
fn vsubps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VSUBPS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledIndexed(ESI, EDX, Eight, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 232, 92, 60, 214], OperandSize::Dword)
}

#[test]
fn vsubps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VSUBPS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 92, 231], OperandSize::Qword)
}

#[test]
fn vsubps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VSUBPS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledIndexed(RAX, RDI, Two, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 232, 92, 36, 120], OperandSize::Qword)
}

#[test]
fn vsubps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VSUBPS, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(YMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 228, 92, 226], OperandSize::Dword)
}

#[test]
fn vsubps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VSUBPS, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectScaledDisplaced(EDI, Four, 610921570, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 204, 92, 20, 189, 98, 236, 105, 36], OperandSize::Dword)
}

#[test]
fn vsubps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VSUBPS, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 244, 92, 225], OperandSize::Qword)
}

#[test]
fn vsubps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VSUBPS, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectDisplaced(RDX, 584289318, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 252, 92, 162, 38, 140, 211, 34], OperandSize::Qword)
}

#[test]
fn vsubps_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VSUBPS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 108, 142, 92, 247], OperandSize::Dword)
}

#[test]
fn vsubps_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VSUBPS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectDisplaced(EDI, 6715842, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 92, 142, 92, 191, 194, 121, 102, 0], OperandSize::Dword)
}

#[test]
fn vsubps_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VSUBPS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectScaledIndexedDisplaced(EDX, ESI, Four, 46082966, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 76, 156, 92, 148, 178, 150, 43, 191, 2], OperandSize::Dword)
}

#[test]
fn vsubps_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VSUBPS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM24)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 145, 100, 143, 92, 208], OperandSize::Qword)
}

#[test]
fn vsubps_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VSUBPS, operand1: Some(Direct(XMM16)), operand2: Some(Direct(XMM29)), operand3: Some(IndirectScaledDisplaced(RAX, Two, 996406317, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 225, 20, 131, 92, 4, 69, 45, 244, 99, 59], OperandSize::Qword)
}

#[test]
fn vsubps_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VSUBPS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM17)), operand3: Some(IndirectDisplaced(RSI, 115818267, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 116, 151, 92, 134, 27, 63, 231, 6], OperandSize::Qword)
}

#[test]
fn vsubps_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VSUBPS, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(YMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 100, 174, 92, 251], OperandSize::Dword)
}

#[test]
fn vsubps_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VSUBPS, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectScaledIndexedDisplaced(EDI, EDX, Two, 1133905060, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 108, 169, 92, 172, 87, 164, 4, 150, 67], OperandSize::Dword)
}

#[test]
fn vsubps_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VSUBPS, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM3)), operand3: Some(Indirect(EDI, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 100, 186, 92, 15], OperandSize::Dword)
}

#[test]
fn vsubps_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VSUBPS, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM17)), operand3: Some(Direct(YMM14)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 209, 116, 165, 92, 238], OperandSize::Qword)
}

#[test]
fn vsubps_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VSUBPS, operand1: Some(Direct(YMM28)), operand2: Some(Direct(YMM1)), operand3: Some(Indirect(RAX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 97, 116, 175, 92, 32], OperandSize::Qword)
}

#[test]
fn vsubps_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VSUBPS, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectScaledIndexedDisplaced(RDI, RDX, Four, 1657590029, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 116, 185, 92, 180, 151, 13, 209, 204, 98], OperandSize::Qword)
}

#[test]
fn vsubps_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VSUBPS, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM7)), operand3: Some(Direct(ZMM3)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Nearest), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 68, 153, 92, 227], OperandSize::Dword)
}

#[test]
fn vsubps_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VSUBPS, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM4)), operand3: Some(IndirectDisplaced(EDX, 1552413305, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 92, 202, 92, 154, 121, 242, 135, 92], OperandSize::Dword)
}

#[test]
fn vsubps_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VSUBPS, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM1)), operand3: Some(IndirectScaledIndexedDisplaced(EDI, ECX, Four, 919125494, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 241, 116, 218, 92, 180, 143, 246, 189, 200, 54], OperandSize::Dword)
}

#[test]
fn vsubps_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VSUBPS, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM1)), operand3: Some(Direct(ZMM18)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Down), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 177, 116, 188, 92, 194], OperandSize::Qword)
}

#[test]
fn vsubps_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VSUBPS, operand1: Some(Direct(ZMM25)), operand2: Some(Direct(ZMM26)), operand3: Some(IndirectScaledDisplaced(RCX, Eight, 1350323956, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 97, 44, 193, 92, 12, 205, 244, 78, 124, 80], OperandSize::Qword)
}

#[test]
fn vsubps_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VSUBPS, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM19)), operand3: Some(IndirectScaledIndexedDisplaced(RSI, RDX, Two, 1063859010, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 241, 100, 214, 92, 132, 86, 66, 51, 105, 63], OperandSize::Qword)
}

