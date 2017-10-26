use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vandps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDPS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 84, 233], OperandSize::Dword)
}

#[test]
fn vandps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDPS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledDisplaced(EAX, Eight, 1136328344, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 208, 84, 52, 197, 152, 254, 186, 67], OperandSize::Dword)
}

#[test]
fn vandps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDPS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 216, 84, 219], OperandSize::Qword)
}

#[test]
fn vandps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDPS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledIndexedDisplaced(RAX, RBX, Four, 1865359685, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 84, 132, 152, 69, 33, 47, 111], OperandSize::Qword)
}

#[test]
fn vandps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDPS, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(YMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 228, 84, 207], OperandSize::Dword)
}

#[test]
fn vandps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDPS, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM5)), operand3: Some(Indirect(EDX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 212, 84, 42], OperandSize::Dword)
}

#[test]
fn vandps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDPS, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 236, 84, 192], OperandSize::Qword)
}

#[test]
fn vandps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDPS, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectScaledIndexedDisplaced(RBX, RBX, Two, 1079500806, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 228, 84, 156, 91, 6, 224, 87, 64], OperandSize::Qword)
}

#[test]
fn vandps_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDPS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 100, 141, 84, 238], OperandSize::Dword)
}

#[test]
fn vandps_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDPS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledIndexedDisplaced(ECX, EDX, Two, 858308464, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 84, 142, 84, 172, 81, 112, 191, 40, 51], OperandSize::Dword)
}

#[test]
fn vandps_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDPS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledIndexed(ECX, ESI, Two, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 100, 159, 84, 60, 113], OperandSize::Dword)
}

#[test]
fn vandps_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDPS, operand1: Some(Direct(XMM15)), operand2: Some(Direct(XMM29)), operand3: Some(Direct(XMM10)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 81, 20, 129, 84, 250], OperandSize::Qword)
}

#[test]
fn vandps_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDPS, operand1: Some(Direct(XMM10)), operand2: Some(Direct(XMM19)), operand3: Some(IndirectScaledDisplaced(RCX, Eight, 1881546642, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 113, 100, 134, 84, 20, 205, 146, 31, 38, 112], OperandSize::Qword)
}

#[test]
fn vandps_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDPS, operand1: Some(Direct(XMM25)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectDisplaced(RCX, 237096231, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 97, 124, 156, 84, 137, 39, 205, 33, 14], OperandSize::Qword)
}

#[test]
fn vandps_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDPS, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(YMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 84, 169, 84, 226], OperandSize::Dword)
}

#[test]
fn vandps_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDPS, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectScaledIndexed(ESI, ECX, Four, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 68, 169, 84, 4, 142], OperandSize::Dword)
}

#[test]
fn vandps_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDPS, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectScaledIndexed(ESI, ECX, Eight, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 124, 191, 84, 36, 206], OperandSize::Dword)
}

#[test]
fn vandps_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDPS, operand1: Some(Direct(YMM17)), operand2: Some(Direct(YMM22)), operand3: Some(Direct(YMM27)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 129, 76, 163, 84, 203], OperandSize::Qword)
}

#[test]
fn vandps_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDPS, operand1: Some(Direct(YMM30)), operand2: Some(Direct(YMM14)), operand3: Some(Indirect(RDI, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 97, 12, 173, 84, 55], OperandSize::Qword)
}

#[test]
fn vandps_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDPS, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM27)), operand3: Some(IndirectScaledIndexedDisplaced(RCX, RDX, Two, 349319631, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 36, 177, 84, 180, 81, 207, 49, 210, 20], OperandSize::Qword)
}

#[test]
fn vandps_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDPS, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM2)), operand3: Some(Direct(ZMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 108, 201, 84, 236], OperandSize::Dword)
}

#[test]
fn vandps_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDPS, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM3)), operand3: Some(IndirectScaledDisplaced(EBX, Two, 1225315573, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 100, 202, 84, 36, 93, 245, 212, 8, 73], OperandSize::Dword)
}

#[test]
fn vandps_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDPS, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM6)), operand3: Some(Indirect(EBX, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 241, 76, 220, 84, 51], OperandSize::Dword)
}

#[test]
fn vandps_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDPS, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM25)), operand3: Some(Direct(ZMM25)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 145, 52, 196, 84, 217], OperandSize::Qword)
}

#[test]
fn vandps_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDPS, operand1: Some(Direct(ZMM22)), operand2: Some(Direct(ZMM9)), operand3: Some(Indirect(RCX, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 225, 52, 207, 84, 49], OperandSize::Qword)
}

#[test]
fn vandps_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDPS, operand1: Some(Direct(ZMM20)), operand2: Some(Direct(ZMM22)), operand3: Some(IndirectScaledIndexed(RBX, RDX, Eight, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 225, 76, 211, 84, 36, 211], OperandSize::Qword)
}

