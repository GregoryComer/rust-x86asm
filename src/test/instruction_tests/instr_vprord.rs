use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vprord_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM4)), operand3: Some(Literal8(82)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 109, 137, 114, 196, 82], OperandSize::Dword)
}

#[test]
fn vprord_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORD, operand1: Some(Direct(XMM1)), operand2: Some(IndirectScaledIndexed(EDI, ECX, Four, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(10)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 117, 137, 114, 4, 143, 10], OperandSize::Dword)
}

#[test]
fn vprord_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORD, operand1: Some(Direct(XMM2)), operand2: Some(IndirectDisplaced(EDI, 299732578, Some(OperandSize::Dword), None)), operand3: Some(Literal8(72)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 109, 157, 114, 135, 98, 142, 221, 17, 72], OperandSize::Dword)
}

#[test]
fn vprord_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORD, operand1: Some(Direct(XMM13)), operand2: Some(Direct(XMM18)), operand3: Some(Literal8(80)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 177, 21, 139, 114, 194, 80], OperandSize::Qword)
}

#[test]
fn vprord_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORD, operand1: Some(Direct(XMM6)), operand2: Some(IndirectScaledDisplaced(RDX, Two, 1166078507, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(103)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 77, 139, 114, 4, 85, 43, 242, 128, 69, 103], OperandSize::Qword)
}

#[test]
fn vprord_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORD, operand1: Some(Direct(XMM21)), operand2: Some(IndirectScaledDisplaced(RBX, Four, 2035904241, Some(OperandSize::Dword), None)), operand3: Some(Literal8(51)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 85, 148, 114, 4, 157, 241, 110, 89, 121, 51], OperandSize::Qword)
}

#[test]
fn vprord_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORD, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM2)), operand3: Some(Literal8(122)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 93, 171, 114, 194, 122], OperandSize::Dword)
}

#[test]
fn vprord_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORD, operand1: Some(Direct(YMM2)), operand2: Some(IndirectScaledDisplaced(EAX, Four, 1468846723, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(41)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 109, 170, 114, 4, 133, 131, 210, 140, 87, 41], OperandSize::Dword)
}

#[test]
fn vprord_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORD, operand1: Some(Direct(YMM1)), operand2: Some(IndirectScaledDisplaced(EDX, Four, 1091217345, Some(OperandSize::Dword), None)), operand3: Some(Literal8(1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 117, 188, 114, 4, 149, 193, 167, 10, 65, 1], OperandSize::Dword)
}

#[test]
fn vprord_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORD, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM22)), operand3: Some(Literal8(123)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 177, 117, 173, 114, 198, 123], OperandSize::Qword)
}

#[test]
fn vprord_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORD, operand1: Some(Direct(YMM2)), operand2: Some(IndirectScaledIndexed(RDI, RBX, Eight, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(78)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 109, 171, 114, 4, 223, 78], OperandSize::Qword)
}

#[test]
fn vprord_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORD, operand1: Some(Direct(YMM9)), operand2: Some(IndirectScaledDisplaced(RDX, Four, 1668090149, Some(OperandSize::Dword), None)), operand3: Some(Literal8(73)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 53, 188, 114, 4, 149, 37, 9, 109, 99, 73], OperandSize::Qword)
}

#[test]
fn vprord_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORD, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM4)), operand3: Some(Literal8(56)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 85, 202, 114, 196, 56], OperandSize::Dword)
}

#[test]
fn vprord_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORD, operand1: Some(Direct(ZMM4)), operand2: Some(IndirectDisplaced(EBX, 699666728, Some(OperandSize::Zmmword), None)), operand3: Some(Literal8(8)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 93, 204, 114, 131, 40, 17, 180, 41, 8], OperandSize::Dword)
}

#[test]
fn vprord_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORD, operand1: Some(Direct(ZMM0)), operand2: Some(IndirectDisplaced(EDI, 2102612717, Some(OperandSize::Dword), None)), operand3: Some(Literal8(63)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 241, 125, 221, 114, 135, 237, 82, 83, 125, 63], OperandSize::Dword)
}

#[test]
fn vprord_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORD, operand1: Some(Direct(ZMM16)), operand2: Some(Direct(ZMM25)), operand3: Some(Literal8(110)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 145, 125, 195, 114, 193, 110], OperandSize::Qword)
}

#[test]
fn vprord_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORD, operand1: Some(Direct(ZMM13)), operand2: Some(IndirectScaledIndexedDisplaced(RSI, RAX, Eight, 2011742005, Some(OperandSize::Zmmword), None)), operand3: Some(Literal8(93)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 21, 202, 114, 132, 198, 53, 191, 232, 119, 93], OperandSize::Qword)
}

#[test]
fn vprord_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORD, operand1: Some(Direct(ZMM25)), operand2: Some(IndirectScaledIndexed(RDI, RCX, Four, Some(OperandSize::Dword), None)), operand3: Some(Literal8(93)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 241, 53, 212, 114, 4, 143, 93], OperandSize::Qword)
}

