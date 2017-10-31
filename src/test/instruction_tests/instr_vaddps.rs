use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vaddps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDPS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 232, 88, 216], OperandSize::Dword)
}

#[test]
fn vaddps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDPS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledIndexed(ESI, ECX, Eight, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 192, 88, 52, 206], OperandSize::Dword)
}

#[test]
fn vaddps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDPS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 216, 88, 241], OperandSize::Qword)
}

#[test]
fn vaddps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDPS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledDisplaced(RCX, Eight, 283033762, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 208, 88, 4, 205, 162, 192, 222, 16], OperandSize::Qword)
}

#[test]
fn vaddps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDPS, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(YMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 228, 88, 236], OperandSize::Dword)
}

#[test]
fn vaddps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDPS, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM2)), operand3: Some(Indirect(EDI, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 236, 88, 7], OperandSize::Dword)
}

#[test]
fn vaddps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDPS, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(YMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 204, 88, 234], OperandSize::Qword)
}

#[test]
fn vaddps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDPS, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectScaledIndexedDisplaced(RAX, RSI, Eight, 13867786, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 252, 88, 164, 240, 10, 155, 211, 0], OperandSize::Qword)
}

#[test]
fn vaddps_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDPS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 76, 140, 88, 214], OperandSize::Dword)
}

#[test]
fn vaddps_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDPS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM4)), operand3: Some(Indirect(ESI, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 92, 143, 88, 14], OperandSize::Dword)
}

#[test]
fn vaddps_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDPS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectDisplaced(EAX, 335620217, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 76, 153, 88, 136, 121, 40, 1, 20], OperandSize::Dword)
}

#[test]
fn vaddps_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDPS, operand1: Some(Direct(XMM10)), operand2: Some(Direct(XMM17)), operand3: Some(Direct(XMM13)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 81, 116, 129, 88, 213], OperandSize::Qword)
}

#[test]
fn vaddps_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDPS, operand1: Some(Direct(XMM13)), operand2: Some(Direct(XMM21)), operand3: Some(Indirect(RSI, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 113, 84, 131, 88, 46], OperandSize::Qword)
}

#[test]
fn vaddps_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDPS, operand1: Some(Direct(XMM15)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectDisplaced(RSI, 166611964, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 113, 124, 153, 88, 190, 252, 75, 238, 9], OperandSize::Qword)
}

#[test]
fn vaddps_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDPS, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(YMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 124, 175, 88, 234], OperandSize::Dword)
}

#[test]
fn vaddps_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDPS, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectScaledIndexed(EDX, EAX, Four, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 108, 175, 88, 12, 130], OperandSize::Dword)
}

#[test]
fn vaddps_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDPS, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectDisplaced(EAX, 1242367729, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 100, 187, 88, 168, 241, 6, 13, 74], OperandSize::Dword)
}

#[test]
fn vaddps_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDPS, operand1: Some(Direct(YMM28)), operand2: Some(Direct(YMM23)), operand3: Some(Direct(YMM24)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 1, 68, 165, 88, 224], OperandSize::Qword)
}

#[test]
fn vaddps_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDPS, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM27)), operand3: Some(IndirectScaledIndexedDisplaced(RSI, RDX, Four, 1287417161, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 36, 163, 88, 172, 150, 73, 109, 188, 76], OperandSize::Qword)
}

#[test]
fn vaddps_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDPS, operand1: Some(Direct(YMM11)), operand2: Some(Direct(YMM18)), operand3: Some(IndirectDisplaced(RAX, 192292737, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 113, 108, 181, 88, 152, 129, 39, 118, 11], OperandSize::Qword)
}

#[test]
fn vaddps_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDPS, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM2)), operand3: Some(Direct(ZMM4)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Up), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 108, 223, 88, 196], OperandSize::Dword)
}

#[test]
fn vaddps_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDPS, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM6)), operand3: Some(IndirectScaledIndexedDisplaced(EDX, EBX, Eight, 553151856, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 76, 203, 88, 164, 218, 112, 109, 248, 32], OperandSize::Dword)
}

#[test]
fn vaddps_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDPS, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM3)), operand3: Some(IndirectScaledDisplaced(ESI, Two, 168188154, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 241, 100, 217, 88, 12, 117, 250, 88, 6, 10], OperandSize::Dword)
}

#[test]
fn vaddps_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDPS, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM6)), operand3: Some(Direct(ZMM20)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Nearest), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 177, 76, 154, 88, 220], OperandSize::Qword)
}

#[test]
fn vaddps_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDPS, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM19)), operand3: Some(IndirectDisplaced(RDI, 1732872049, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 100, 194, 88, 151, 113, 135, 73, 103], OperandSize::Qword)
}

#[test]
fn vaddps_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDPS, operand1: Some(Direct(ZMM16)), operand2: Some(Direct(ZMM26)), operand3: Some(IndirectScaledIndexedDisplaced(RSI, RCX, Two, 1066279577, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 225, 44, 212, 88, 132, 78, 153, 34, 142, 63], OperandSize::Qword)
}

