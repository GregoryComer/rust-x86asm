use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vaddps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDPS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 192, 88, 229], OperandSize::Dword)
}

#[test]
fn vaddps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDPS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledDisplaced(ESI, Four, 1109423964, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 216, 88, 52, 181, 92, 119, 32, 66], OperandSize::Dword)
}

#[test]
fn vaddps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDPS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 224, 88, 200], OperandSize::Qword)
}

#[test]
fn vaddps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDPS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledIndexedDisplaced(RSI, RSI, Eight, 1246218299, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 192, 88, 148, 246, 59, 200, 71, 74], OperandSize::Qword)
}

#[test]
fn vaddps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDPS, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 220, 88, 233], OperandSize::Dword)
}

#[test]
fn vaddps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDPS, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectScaledIndexedDisplaced(EDI, EBX, Two, 136148257, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 196, 88, 140, 95, 33, 117, 29, 8], OperandSize::Dword)
}

#[test]
fn vaddps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDPS, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(YMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 252, 88, 215], OperandSize::Qword)
}

#[test]
fn vaddps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDPS, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectScaledIndexedDisplaced(RSI, RBX, Four, 362928450, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 212, 88, 148, 158, 66, 217, 161, 21], OperandSize::Qword)
}

#[test]
fn vaddps_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDPS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 100, 142, 88, 217], OperandSize::Dword)
}

#[test]
fn vaddps_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDPS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectDisplaced(EBX, 1071035876, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 76, 142, 88, 163, 228, 181, 214, 63], OperandSize::Dword)
}

#[test]
fn vaddps_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDPS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledDisplaced(EDI, Eight, 277953460, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 68, 157, 88, 4, 253, 180, 59, 145, 16], OperandSize::Dword)
}

#[test]
fn vaddps_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDPS, operand1: Some(Direct(XMM25)), operand2: Some(Direct(XMM27)), operand3: Some(Direct(XMM16)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 33, 36, 132, 88, 200], OperandSize::Qword)
}

#[test]
fn vaddps_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDPS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM25)), operand3: Some(IndirectScaledDisplaced(RCX, Eight, 1025080497, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 52, 132, 88, 60, 205, 177, 124, 25, 61], OperandSize::Qword)
}

#[test]
fn vaddps_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDPS, operand1: Some(Direct(XMM9)), operand2: Some(Direct(XMM24)), operand3: Some(IndirectScaledIndexedDisplaced(RBX, RCX, Two, 688138582, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 113, 60, 149, 88, 140, 75, 86, 41, 4, 41], OperandSize::Qword)
}

#[test]
fn vaddps_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDPS, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 92, 169, 88, 240], OperandSize::Dword)
}

#[test]
fn vaddps_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDPS, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectScaledIndexed(EDI, EDX, Two, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 116, 175, 88, 4, 87], OperandSize::Dword)
}

#[test]
fn vaddps_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDPS, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM6)), operand3: Some(Indirect(EAX, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 76, 191, 88, 56], OperandSize::Dword)
}

#[test]
fn vaddps_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDPS, operand1: Some(Direct(YMM13)), operand2: Some(Direct(YMM31)), operand3: Some(Direct(YMM14)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 81, 4, 167, 88, 238], OperandSize::Qword)
}

#[test]
fn vaddps_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDPS, operand1: Some(Direct(YMM10)), operand2: Some(Direct(YMM19)), operand3: Some(IndirectScaledIndexedDisplaced(RAX, RSI, Eight, 2028244923, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 113, 100, 166, 88, 148, 240, 187, 143, 228, 120], OperandSize::Qword)
}

#[test]
fn vaddps_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDPS, operand1: Some(Direct(YMM28)), operand2: Some(Direct(YMM16)), operand3: Some(Indirect(RCX, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 97, 124, 180, 88, 33], OperandSize::Qword)
}

#[test]
fn vaddps_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDPS, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM1)), operand3: Some(Direct(ZMM7)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Up), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 116, 221, 88, 247], OperandSize::Dword)
}

#[test]
fn vaddps_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDPS, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM6)), operand3: Some(IndirectScaledDisplaced(ECX, Two, 601549072, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 76, 205, 88, 20, 77, 16, 233, 218, 35], OperandSize::Dword)
}

#[test]
fn vaddps_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDPS, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM6)), operand3: Some(Indirect(ECX, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 241, 76, 222, 88, 1], OperandSize::Dword)
}

#[test]
fn vaddps_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDPS, operand1: Some(Direct(ZMM27)), operand2: Some(Direct(ZMM12)), operand3: Some(Direct(ZMM20)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Up), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 33, 28, 222, 88, 220], OperandSize::Qword)
}

#[test]
fn vaddps_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDPS, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM17)), operand3: Some(IndirectScaledIndexedDisplaced(RDI, RAX, Two, 430979722, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 116, 195, 88, 164, 71, 138, 58, 176, 25], OperandSize::Qword)
}

#[test]
fn vaddps_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDPS, operand1: Some(Direct(ZMM8)), operand2: Some(Direct(ZMM9)), operand3: Some(IndirectScaledIndexedDisplaced(RDI, RCX, Two, 1451992210, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 113, 52, 218, 88, 132, 79, 146, 164, 139, 86], OperandSize::Qword)
}

