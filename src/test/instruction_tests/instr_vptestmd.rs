use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vptestmd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTMD, operand1: Some(Direct(K1)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 117, 9, 39, 202], OperandSize::Dword)
}

#[test]
fn vptestmd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTMD, operand1: Some(Direct(K1)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectDisplaced(ESI, 64088176, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 125, 13, 39, 142, 112, 232, 209, 3], OperandSize::Dword)
}

#[test]
fn vptestmd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTMD, operand1: Some(Direct(K4)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledIndexedDisplaced(EAX, EDI, Two, 230569804, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 125, 31, 39, 164, 120, 76, 55, 190, 13], OperandSize::Dword)
}

#[test]
fn vptestmd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTMD, operand1: Some(Direct(K7)), operand2: Some(Direct(XMM13)), operand3: Some(Direct(XMM21)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 178, 21, 10, 39, 253], OperandSize::Qword)
}

#[test]
fn vptestmd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTMD, operand1: Some(Direct(K7)), operand2: Some(Direct(XMM13)), operand3: Some(IndirectScaledIndexed(RDX, RCX, Eight, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 21, 10, 39, 60, 202], OperandSize::Qword)
}

#[test]
fn vptestmd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTMD, operand1: Some(Direct(K2)), operand2: Some(Direct(XMM12)), operand3: Some(IndirectDisplaced(RCX, 1805694641, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 29, 28, 39, 145, 177, 182, 160, 107], OperandSize::Qword)
}

#[test]
fn vptestmd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTMD, operand1: Some(Direct(K4)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(YMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 77, 47, 39, 231], OperandSize::Dword)
}

#[test]
fn vptestmd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTMD, operand1: Some(Direct(K2)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectScaledDisplaced(EBX, Two, 654173302, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 93, 41, 39, 20, 93, 118, 228, 253, 38], OperandSize::Dword)
}

#[test]
fn vptestmd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTMD, operand1: Some(Direct(K1)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectDisplaced(EBX, 645627552, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 93, 62, 39, 139, 160, 126, 123, 38], OperandSize::Dword)
}

#[test]
fn vptestmd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTMD, operand1: Some(Direct(K2)), operand2: Some(Direct(YMM25)), operand3: Some(Direct(YMM21)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 178, 53, 39, 39, 213], OperandSize::Qword)
}

#[test]
fn vptestmd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTMD, operand1: Some(Direct(K7)), operand2: Some(Direct(YMM20)), operand3: Some(IndirectScaledIndexed(RSI, RCX, Two, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 93, 37, 39, 60, 78], OperandSize::Qword)
}

#[test]
fn vptestmd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTMD, operand1: Some(Direct(K6)), operand2: Some(Direct(YMM19)), operand3: Some(IndirectScaledDisplaced(RSI, Eight, 1729335872, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 101, 50, 39, 52, 245, 64, 146, 19, 103], OperandSize::Qword)
}

#[test]
fn vptestmd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTMD, operand1: Some(Direct(K7)), operand2: Some(Direct(ZMM6)), operand3: Some(Direct(ZMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 77, 75, 39, 253], OperandSize::Dword)
}

#[test]
fn vptestmd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTMD, operand1: Some(Direct(K5)), operand2: Some(Direct(ZMM7)), operand3: Some(Indirect(ESI, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 69, 74, 39, 46], OperandSize::Dword)
}

#[test]
fn vptestmd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTMD, operand1: Some(Direct(K2)), operand2: Some(Direct(ZMM6)), operand3: Some(IndirectScaledIndexedDisplaced(EDI, ECX, Two, 938075067, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 242, 77, 89, 39, 148, 79, 187, 227, 233, 55], OperandSize::Dword)
}

#[test]
fn vptestmd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTMD, operand1: Some(Direct(K5)), operand2: Some(Direct(ZMM28)), operand3: Some(Direct(ZMM18)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 178, 29, 67, 39, 234], OperandSize::Qword)
}

#[test]
fn vptestmd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTMD, operand1: Some(Direct(K7)), operand2: Some(Direct(ZMM27)), operand3: Some(Indirect(RCX, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 37, 65, 39, 57], OperandSize::Qword)
}

#[test]
fn vptestmd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTMD, operand1: Some(Direct(K6)), operand2: Some(Direct(ZMM4)), operand3: Some(IndirectScaledIndexed(RAX, RAX, Eight, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 242, 93, 94, 39, 52, 192], OperandSize::Qword)
}

