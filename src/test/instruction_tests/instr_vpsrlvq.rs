use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpsrlvq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLVQ, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 233, 69, 233], OperandSize::Dword)
}

#[test]
fn vpsrlvq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLVQ, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectScaledIndexed(EBX, ECX, Four, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 201, 69, 60, 139], OperandSize::Dword)
}

#[test]
fn vpsrlvq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLVQ, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 193, 69, 245], OperandSize::Qword)
}

#[test]
fn vpsrlvq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLVQ, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectScaledIndexed(RDX, RBX, Four, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 241, 69, 52, 154], OperandSize::Qword)
}

#[test]
fn vpsrlvq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLVQ, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 221, 69, 230], OperandSize::Dword)
}

#[test]
fn vpsrlvq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLVQ, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM0)), operand3: Some(Indirect(ECX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 253, 69, 57], OperandSize::Dword)
}

#[test]
fn vpsrlvq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLVQ, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 221, 69, 225], OperandSize::Qword)
}

#[test]
fn vpsrlvq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLVQ, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectDisplaced(RAX, 951179166, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 197, 69, 168, 158, 215, 177, 56], OperandSize::Qword)
}

#[test]
fn vpsrlvq_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLVQ, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 237, 142, 69, 213], OperandSize::Dword)
}

#[test]
fn vpsrlvq_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLVQ, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectDisplaced(EDI, 889497927, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 237, 138, 69, 183, 71, 169, 4, 53], OperandSize::Dword)
}

#[test]
fn vpsrlvq_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLVQ, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectScaledDisplaced(EBX, Two, 972046369, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 242, 245, 155, 69, 12, 93, 33, 64, 240, 57], OperandSize::Dword)
}

#[test]
fn vpsrlvq_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLVQ, operand1: Some(Direct(XMM19)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM26)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 130, 253, 138, 69, 218], OperandSize::Qword)
}

#[test]
fn vpsrlvq_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLVQ, operand1: Some(Direct(XMM15)), operand2: Some(Direct(XMM30)), operand3: Some(Indirect(RCX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 114, 141, 135, 69, 57], OperandSize::Qword)
}

#[test]
fn vpsrlvq_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLVQ, operand1: Some(Direct(XMM14)), operand2: Some(Direct(XMM22)), operand3: Some(IndirectScaledIndexed(RDX, RCX, Eight, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 114, 205, 148, 69, 52, 202], OperandSize::Qword)
}

#[test]
fn vpsrlvq_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLVQ, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(YMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 253, 173, 69, 239], OperandSize::Dword)
}

#[test]
fn vpsrlvq_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLVQ, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM1)), operand3: Some(Indirect(ESI, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 245, 174, 69, 38], OperandSize::Dword)
}

#[test]
fn vpsrlvq_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLVQ, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM7)), operand3: Some(Indirect(EAX, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 197, 186, 69, 56], OperandSize::Dword)
}

#[test]
fn vpsrlvq_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLVQ, operand1: Some(Direct(YMM14)), operand2: Some(Direct(YMM19)), operand3: Some(Direct(YMM21)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 50, 229, 164, 69, 245], OperandSize::Qword)
}

#[test]
fn vpsrlvq_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLVQ, operand1: Some(Direct(YMM19)), operand2: Some(Direct(YMM15)), operand3: Some(IndirectScaledIndexedDisplaced(RCX, RAX, Eight, 1159137893, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 226, 133, 169, 69, 156, 193, 101, 10, 23, 69], OperandSize::Qword)
}

#[test]
fn vpsrlvq_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLVQ, operand1: Some(Direct(YMM12)), operand2: Some(Direct(YMM22)), operand3: Some(Indirect(RAX, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 114, 205, 183, 69, 32], OperandSize::Qword)
}

#[test]
fn vpsrlvq_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLVQ, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM3)), operand3: Some(Direct(ZMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 229, 206, 69, 254], OperandSize::Dword)
}

#[test]
fn vpsrlvq_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLVQ, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM7)), operand3: Some(Indirect(EBX, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 197, 207, 69, 35], OperandSize::Dword)
}

#[test]
fn vpsrlvq_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLVQ, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM4)), operand3: Some(IndirectScaledIndexed(EBX, EAX, Two, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 221, 221, 69, 60, 67], OperandSize::Dword)
}

#[test]
fn vpsrlvq_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLVQ, operand1: Some(Direct(ZMM21)), operand2: Some(Direct(ZMM1)), operand3: Some(Direct(ZMM18)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 162, 245, 205, 69, 234], OperandSize::Qword)
}

#[test]
fn vpsrlvq_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLVQ, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM6)), operand3: Some(IndirectScaledIndexedDisplaced(RAX, RBX, Four, 981601089, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 205, 206, 69, 132, 152, 65, 11, 130, 58], OperandSize::Qword)
}

#[test]
fn vpsrlvq_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLVQ, operand1: Some(Direct(ZMM21)), operand2: Some(Direct(ZMM1)), operand3: Some(IndirectScaledDisplaced(RDI, Eight, 842472632, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 226, 245, 218, 69, 44, 253, 184, 28, 55, 50], OperandSize::Qword)
}

