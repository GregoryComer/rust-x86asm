use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vfpclassps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VFPCLASSPS, operand1: Some(Direct(K2)), operand2: Some(Direct(XMM3)), operand3: Some(Literal8(2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 243, 125, 12, 102, 211, 2], OperandSize::Dword)
}

#[test]
fn vfpclassps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VFPCLASSPS, operand1: Some(Direct(K6)), operand2: Some(Indirect(EDX, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(94)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 243, 125, 11, 102, 50, 94], OperandSize::Dword)
}

#[test]
fn vfpclassps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VFPCLASSPS, operand1: Some(Direct(K2)), operand2: Some(IndirectScaledIndexedDisplaced(EDI, ESI, Two, 2066352994, Some(OperandSize::Dword), None)), operand3: Some(Literal8(117)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 243, 125, 30, 102, 148, 119, 98, 11, 42, 123, 117], OperandSize::Dword)
}

#[test]
fn vfpclassps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VFPCLASSPS, operand1: Some(Direct(K7)), operand2: Some(Direct(XMM0)), operand3: Some(Literal8(115)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 243, 125, 13, 102, 248, 115], OperandSize::Qword)
}

#[test]
fn vfpclassps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VFPCLASSPS, operand1: Some(Direct(K5)), operand2: Some(IndirectScaledIndexedDisplaced(RDI, RDI, Eight, 552281664, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(10)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 243, 125, 11, 102, 172, 255, 64, 38, 235, 32, 10], OperandSize::Qword)
}

#[test]
fn vfpclassps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VFPCLASSPS, operand1: Some(Direct(K6)), operand2: Some(Indirect(RDX, Some(OperandSize::Dword), None)), operand3: Some(Literal8(84)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 243, 125, 29, 102, 50, 84], OperandSize::Qword)
}

#[test]
fn vfpclassps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VFPCLASSPS, operand1: Some(Direct(K5)), operand2: Some(Direct(YMM4)), operand3: Some(Literal8(94)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 243, 125, 41, 102, 236, 94], OperandSize::Dword)
}

#[test]
fn vfpclassps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VFPCLASSPS, operand1: Some(Direct(K3)), operand2: Some(IndirectScaledIndexed(ECX, ECX, Four, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(14)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 243, 125, 44, 102, 28, 137, 14], OperandSize::Dword)
}

#[test]
fn vfpclassps_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VFPCLASSPS, operand1: Some(Direct(K7)), operand2: Some(IndirectScaledIndexed(ECX, EDI, Two, Some(OperandSize::Dword), None)), operand3: Some(Literal8(51)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 243, 125, 61, 102, 60, 121, 51], OperandSize::Dword)
}

#[test]
fn vfpclassps_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VFPCLASSPS, operand1: Some(Direct(K4)), operand2: Some(Direct(YMM4)), operand3: Some(Literal8(117)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 243, 125, 47, 102, 228, 117], OperandSize::Qword)
}

#[test]
fn vfpclassps_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VFPCLASSPS, operand1: Some(Direct(K2)), operand2: Some(IndirectScaledIndexed(RBX, RCX, Two, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(86)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 243, 125, 41, 102, 20, 75, 86], OperandSize::Qword)
}

#[test]
fn vfpclassps_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VFPCLASSPS, operand1: Some(Direct(K3)), operand2: Some(IndirectDisplaced(RBX, 1559698870, Some(OperandSize::Dword), None)), operand3: Some(Literal8(121)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 243, 125, 63, 102, 155, 182, 29, 247, 92, 121], OperandSize::Qword)
}

#[test]
fn vfpclassps_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VFPCLASSPS, operand1: Some(Direct(K4)), operand2: Some(Direct(ZMM7)), operand3: Some(Literal8(109)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 243, 125, 78, 102, 231, 109], OperandSize::Dword)
}

#[test]
fn vfpclassps_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VFPCLASSPS, operand1: Some(Direct(K5)), operand2: Some(IndirectScaledIndexed(EDX, EBX, Four, Some(OperandSize::Zmmword), None)), operand3: Some(Literal8(127)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 243, 125, 79, 102, 44, 154, 127], OperandSize::Dword)
}

#[test]
fn vfpclassps_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VFPCLASSPS, operand1: Some(Direct(K4)), operand2: Some(IndirectScaledIndexedDisplaced(ESI, ESI, Four, 914707462, Some(OperandSize::Dword), None)), operand3: Some(Literal8(104)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 243, 125, 94, 102, 164, 182, 6, 84, 133, 54, 104], OperandSize::Dword)
}

#[test]
fn vfpclassps_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VFPCLASSPS, operand1: Some(Direct(K1)), operand2: Some(Direct(ZMM14)), operand3: Some(Literal8(43)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 211, 125, 75, 102, 206, 43], OperandSize::Qword)
}

#[test]
fn vfpclassps_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VFPCLASSPS, operand1: Some(Direct(K7)), operand2: Some(IndirectScaledIndexed(RAX, RDI, Two, Some(OperandSize::Zmmword), None)), operand3: Some(Literal8(35)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 243, 125, 73, 102, 60, 120, 35], OperandSize::Qword)
}

#[test]
fn vfpclassps_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VFPCLASSPS, operand1: Some(Direct(K7)), operand2: Some(IndirectDisplaced(RBX, 2017801700, Some(OperandSize::Dword), None)), operand3: Some(Literal8(57)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 243, 125, 95, 102, 187, 228, 53, 69, 120, 57], OperandSize::Qword)
}

