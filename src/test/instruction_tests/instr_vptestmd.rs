use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vptestmd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTMD, operand1: Some(Direct(K7)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 69, 15, 39, 254], OperandSize::Dword)
}

#[test]
fn vptestmd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTMD, operand1: Some(Direct(K6)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectScaledDisplaced(ECX, Two, 224900891, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 117, 9, 39, 52, 77, 27, 183, 103, 13], OperandSize::Dword)
}

#[test]
fn vptestmd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTMD, operand1: Some(Direct(K5)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectDisplaced(EBX, 1066940009, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 85, 29, 39, 171, 105, 54, 152, 63], OperandSize::Dword)
}

#[test]
fn vptestmd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTMD, operand1: Some(Direct(K3)), operand2: Some(Direct(XMM24)), operand3: Some(Direct(XMM18)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 178, 61, 7, 39, 218], OperandSize::Qword)
}

#[test]
fn vptestmd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTMD, operand1: Some(Direct(K1)), operand2: Some(Direct(XMM20)), operand3: Some(Indirect(RDI, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 93, 6, 39, 15], OperandSize::Qword)
}

#[test]
fn vptestmd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTMD, operand1: Some(Direct(K2)), operand2: Some(Direct(XMM3)), operand3: Some(Indirect(RCX, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 101, 29, 39, 17], OperandSize::Qword)
}

#[test]
fn vptestmd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTMD, operand1: Some(Direct(K5)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 69, 45, 39, 238], OperandSize::Dword)
}

#[test]
fn vptestmd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTMD, operand1: Some(Direct(K5)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectScaledIndexed(EDX, EAX, Eight, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 125, 46, 39, 44, 194], OperandSize::Dword)
}

#[test]
fn vptestmd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTMD, operand1: Some(Direct(K2)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectScaledIndexed(EDX, EDX, Eight, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 93, 57, 39, 20, 210], OperandSize::Dword)
}

#[test]
fn vptestmd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTMD, operand1: Some(Direct(K4)), operand2: Some(Direct(YMM15)), operand3: Some(Direct(YMM15)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 210, 5, 44, 39, 231], OperandSize::Qword)
}

#[test]
fn vptestmd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTMD, operand1: Some(Direct(K1)), operand2: Some(Direct(YMM19)), operand3: Some(IndirectDisplaced(RAX, 565799881, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 101, 36, 39, 136, 201, 107, 185, 33], OperandSize::Qword)
}

#[test]
fn vptestmd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTMD, operand1: Some(Direct(K1)), operand2: Some(Direct(YMM24)), operand3: Some(IndirectScaledIndexed(RCX, RCX, Two, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 61, 52, 39, 12, 73], OperandSize::Qword)
}

#[test]
fn vptestmd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTMD, operand1: Some(Direct(K1)), operand2: Some(Direct(ZMM4)), operand3: Some(Direct(ZMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 93, 75, 39, 203], OperandSize::Dword)
}

#[test]
fn vptestmd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTMD, operand1: Some(Direct(K1)), operand2: Some(Direct(ZMM6)), operand3: Some(IndirectScaledIndexedDisplaced(EDI, EAX, Four, 850543909, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 77, 75, 39, 140, 135, 37, 69, 178, 50], OperandSize::Dword)
}

#[test]
fn vptestmd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTMD, operand1: Some(Direct(K4)), operand2: Some(Direct(ZMM7)), operand3: Some(IndirectScaledDisplaced(EDI, Eight, 982907656, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 242, 69, 91, 39, 36, 253, 8, 251, 149, 58], OperandSize::Dword)
}

#[test]
fn vptestmd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTMD, operand1: Some(Direct(K6)), operand2: Some(Direct(ZMM8)), operand3: Some(Direct(ZMM25)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 146, 61, 74, 39, 241], OperandSize::Qword)
}

#[test]
fn vptestmd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTMD, operand1: Some(Direct(K5)), operand2: Some(Direct(ZMM22)), operand3: Some(IndirectScaledDisplaced(RDX, Two, 1233141744, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 77, 70, 39, 44, 85, 240, 63, 128, 73], OperandSize::Qword)
}

#[test]
fn vptestmd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTMD, operand1: Some(Direct(K1)), operand2: Some(Direct(ZMM14)), operand3: Some(IndirectScaledIndexed(RAX, RSI, Two, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 242, 13, 94, 39, 12, 112], OperandSize::Qword)
}

