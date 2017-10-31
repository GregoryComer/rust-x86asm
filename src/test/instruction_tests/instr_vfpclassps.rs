use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vfpclassps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VFPCLASSPS, operand1: Some(Direct(K6)), operand2: Some(Direct(XMM7)), operand3: Some(Literal8(115)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 243, 125, 13, 102, 247, 115], OperandSize::Dword)
}

#[test]
fn vfpclassps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VFPCLASSPS, operand1: Some(Direct(K4)), operand2: Some(Indirect(EDX, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(78)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 243, 125, 13, 102, 34, 78], OperandSize::Dword)
}

#[test]
fn vfpclassps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VFPCLASSPS, operand1: Some(Direct(K1)), operand2: Some(Indirect(ECX, Some(OperandSize::Dword), None)), operand3: Some(Literal8(27)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 243, 125, 27, 102, 9, 27], OperandSize::Dword)
}

#[test]
fn vfpclassps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VFPCLASSPS, operand1: Some(Direct(K6)), operand2: Some(Direct(XMM19)), operand3: Some(Literal8(80)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 179, 125, 9, 102, 243, 80], OperandSize::Qword)
}

#[test]
fn vfpclassps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VFPCLASSPS, operand1: Some(Direct(K3)), operand2: Some(IndirectDisplaced(RAX, 1685955049, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(102)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 243, 125, 11, 102, 152, 233, 161, 125, 100, 102], OperandSize::Qword)
}

#[test]
fn vfpclassps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VFPCLASSPS, operand1: Some(Direct(K2)), operand2: Some(IndirectScaledDisplaced(RDI, Eight, 476144664, Some(OperandSize::Dword), None)), operand3: Some(Literal8(112)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 243, 125, 29, 102, 20, 253, 24, 100, 97, 28, 112], OperandSize::Qword)
}

#[test]
fn vfpclassps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VFPCLASSPS, operand1: Some(Direct(K2)), operand2: Some(Direct(YMM3)), operand3: Some(Literal8(1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 243, 125, 44, 102, 211, 1], OperandSize::Dword)
}

#[test]
fn vfpclassps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VFPCLASSPS, operand1: Some(Direct(K3)), operand2: Some(Indirect(ESI, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(13)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 243, 125, 42, 102, 30, 13], OperandSize::Dword)
}

#[test]
fn vfpclassps_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VFPCLASSPS, operand1: Some(Direct(K5)), operand2: Some(IndirectScaledIndexed(ECX, ECX, Eight, Some(OperandSize::Dword), None)), operand3: Some(Literal8(43)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 243, 125, 62, 102, 44, 201, 43], OperandSize::Dword)
}

#[test]
fn vfpclassps_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VFPCLASSPS, operand1: Some(Direct(K7)), operand2: Some(Direct(YMM24)), operand3: Some(Literal8(33)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 147, 125, 47, 102, 248, 33], OperandSize::Qword)
}

#[test]
fn vfpclassps_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VFPCLASSPS, operand1: Some(Direct(K2)), operand2: Some(IndirectDisplaced(RCX, 1155713260, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(61)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 243, 125, 43, 102, 145, 236, 200, 226, 68, 61], OperandSize::Qword)
}

#[test]
fn vfpclassps_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VFPCLASSPS, operand1: Some(Direct(K4)), operand2: Some(Indirect(RDX, Some(OperandSize::Dword), None)), operand3: Some(Literal8(0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 243, 125, 59, 102, 34, 0], OperandSize::Qword)
}

#[test]
fn vfpclassps_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VFPCLASSPS, operand1: Some(Direct(K7)), operand2: Some(Direct(ZMM7)), operand3: Some(Literal8(34)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 243, 125, 73, 102, 255, 34], OperandSize::Dword)
}

#[test]
fn vfpclassps_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VFPCLASSPS, operand1: Some(Direct(K2)), operand2: Some(Indirect(EBX, Some(OperandSize::Zmmword), None)), operand3: Some(Literal8(91)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 243, 125, 77, 102, 19, 91], OperandSize::Dword)
}

#[test]
fn vfpclassps_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VFPCLASSPS, operand1: Some(Direct(K4)), operand2: Some(Indirect(EBX, Some(OperandSize::Dword), None)), operand3: Some(Literal8(125)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 243, 125, 95, 102, 35, 125], OperandSize::Dword)
}

#[test]
fn vfpclassps_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VFPCLASSPS, operand1: Some(Direct(K2)), operand2: Some(Direct(ZMM15)), operand3: Some(Literal8(123)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 211, 125, 75, 102, 215, 123], OperandSize::Qword)
}

#[test]
fn vfpclassps_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VFPCLASSPS, operand1: Some(Direct(K6)), operand2: Some(IndirectDisplaced(RDX, 215348264, Some(OperandSize::Zmmword), None)), operand3: Some(Literal8(69)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 243, 125, 75, 102, 178, 40, 244, 213, 12, 69], OperandSize::Qword)
}

#[test]
fn vfpclassps_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VFPCLASSPS, operand1: Some(Direct(K4)), operand2: Some(IndirectScaledIndexedDisplaced(RSI, RAX, Four, 2042149782, Some(OperandSize::Dword), None)), operand3: Some(Literal8(53)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 243, 125, 93, 102, 164, 134, 150, 187, 184, 121, 53], OperandSize::Qword)
}

