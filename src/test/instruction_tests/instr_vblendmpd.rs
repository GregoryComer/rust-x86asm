use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vblendmpd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VBLENDMPD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 229, 139, 101, 242], OperandSize::Dword)
}

#[test]
fn vblendmpd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VBLENDMPD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledIndexed(ECX, ECX, Two, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 229, 142, 101, 28, 73], OperandSize::Dword)
}

#[test]
fn vblendmpd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VBLENDMPD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectDisplaced(EDI, 690215298, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 242, 213, 154, 101, 175, 130, 217, 35, 41], OperandSize::Dword)
}

#[test]
fn vblendmpd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VBLENDMPD, operand1: Some(Direct(XMM12)), operand2: Some(Direct(XMM29)), operand3: Some(Direct(XMM24)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 18, 149, 131, 101, 224], OperandSize::Qword)
}

#[test]
fn vblendmpd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VBLENDMPD, operand1: Some(Direct(XMM20)), operand2: Some(Direct(XMM11)), operand3: Some(IndirectDisplaced(RSI, 766800461, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 226, 165, 137, 101, 166, 77, 114, 180, 45], OperandSize::Qword)
}

#[test]
fn vblendmpd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VBLENDMPD, operand1: Some(Direct(XMM8)), operand2: Some(Direct(XMM24)), operand3: Some(IndirectScaledIndexedDisplaced(RCX, RDX, Four, 1928240796, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 114, 189, 147, 101, 132, 145, 156, 158, 238, 114], OperandSize::Qword)
}

#[test]
fn vblendmpd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VBLENDMPD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(YMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 253, 172, 101, 234], OperandSize::Dword)
}

#[test]
fn vblendmpd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VBLENDMPD, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectDisplaced(EDI, 645259514, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 237, 172, 101, 183, 250, 224, 117, 38], OperandSize::Dword)
}

#[test]
fn vblendmpd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VBLENDMPD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectScaledIndexedDisplaced(EAX, EAX, Eight, 73154883, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 213, 189, 101, 188, 192, 67, 65, 92, 4], OperandSize::Dword)
}

#[test]
fn vblendmpd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VBLENDMPD, operand1: Some(Direct(YMM9)), operand2: Some(Direct(YMM26)), operand3: Some(Direct(YMM14)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 82, 173, 164, 101, 206], OperandSize::Qword)
}

#[test]
fn vblendmpd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VBLENDMPD, operand1: Some(Direct(YMM26)), operand2: Some(Direct(YMM14)), operand3: Some(IndirectScaledIndexedDisplaced(RDI, RSI, Four, 1613146571, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 98, 141, 172, 101, 148, 183, 203, 169, 38, 96], OperandSize::Qword)
}

#[test]
fn vblendmpd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VBLENDMPD, operand1: Some(Direct(YMM9)), operand2: Some(Direct(YMM9)), operand3: Some(IndirectScaledIndexedDisplaced(RDX, RCX, Four, 2003387948, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 114, 181, 187, 101, 140, 138, 44, 70, 105, 119], OperandSize::Qword)
}

#[test]
fn vblendmpd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VBLENDMPD, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM3)), operand3: Some(Direct(ZMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 229, 202, 101, 250], OperandSize::Dword)
}

#[test]
fn vblendmpd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VBLENDMPD, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM1)), operand3: Some(IndirectScaledDisplaced(EBX, Eight, 464566168, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 245, 205, 101, 12, 221, 152, 183, 176, 27], OperandSize::Dword)
}

#[test]
fn vblendmpd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VBLENDMPD, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM0)), operand3: Some(IndirectScaledIndexed(EBX, EDX, Two, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 253, 219, 101, 36, 83], OperandSize::Dword)
}

#[test]
fn vblendmpd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VBLENDMPD, operand1: Some(Direct(ZMM27)), operand2: Some(Direct(ZMM25)), operand3: Some(Direct(ZMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 98, 181, 197, 101, 218], OperandSize::Qword)
}

#[test]
fn vblendmpd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VBLENDMPD, operand1: Some(Direct(ZMM20)), operand2: Some(Direct(ZMM8)), operand3: Some(IndirectScaledDisplaced(RDX, Two, 941358088, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 226, 189, 207, 101, 36, 85, 8, 252, 27, 56], OperandSize::Qword)
}

#[test]
fn vblendmpd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VBLENDMPD, operand1: Some(Direct(ZMM18)), operand2: Some(Direct(ZMM1)), operand3: Some(IndirectScaledIndexedDisplaced(RCX, RSI, Eight, 364214786, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 226, 245, 220, 101, 148, 241, 2, 122, 181, 21], OperandSize::Qword)
}

