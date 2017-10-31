use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vptestmd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTMD, operand1: Some(Direct(K5)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 125, 11, 39, 234], OperandSize::Dword)
}

#[test]
fn vptestmd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTMD, operand1: Some(Direct(K5)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledDisplaced(ECX, Four, 87102446, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 93, 12, 39, 44, 141, 238, 19, 49, 5], OperandSize::Dword)
}

#[test]
fn vptestmd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTMD, operand1: Some(Direct(K5)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectScaledIndexedDisplaced(ECX, EDI, Two, 971421172, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 77, 28, 39, 172, 121, 244, 181, 230, 57], OperandSize::Dword)
}

#[test]
fn vptestmd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTMD, operand1: Some(Direct(K3)), operand2: Some(Direct(XMM25)), operand3: Some(Direct(XMM10)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 210, 53, 7, 39, 218], OperandSize::Qword)
}

#[test]
fn vptestmd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTMD, operand1: Some(Direct(K5)), operand2: Some(Direct(XMM12)), operand3: Some(IndirectScaledIndexed(RAX, RCX, Two, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 29, 15, 39, 44, 72], OperandSize::Qword)
}

#[test]
fn vptestmd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTMD, operand1: Some(Direct(K4)), operand2: Some(Direct(XMM16)), operand3: Some(Indirect(RAX, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 125, 17, 39, 32], OperandSize::Qword)
}

#[test]
fn vptestmd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTMD, operand1: Some(Direct(K3)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(YMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 77, 42, 39, 223], OperandSize::Dword)
}

#[test]
fn vptestmd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTMD, operand1: Some(Direct(K5)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectDisplaced(EAX, 1006566999, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 125, 43, 39, 168, 87, 254, 254, 59], OperandSize::Dword)
}

#[test]
fn vptestmd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTMD, operand1: Some(Direct(K5)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectScaledIndexed(ESI, EBX, Four, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 85, 61, 39, 44, 158], OperandSize::Dword)
}

#[test]
fn vptestmd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTMD, operand1: Some(Direct(K7)), operand2: Some(Direct(YMM9)), operand3: Some(Direct(YMM22)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 178, 53, 44, 39, 254], OperandSize::Qword)
}

#[test]
fn vptestmd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTMD, operand1: Some(Direct(K7)), operand2: Some(Direct(YMM8)), operand3: Some(IndirectDisplaced(RCX, 1497090751, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 61, 47, 39, 185, 191, 202, 59, 89], OperandSize::Qword)
}

#[test]
fn vptestmd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTMD, operand1: Some(Direct(K2)), operand2: Some(Direct(YMM19)), operand3: Some(IndirectScaledIndexed(RDX, RDX, Two, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 101, 49, 39, 20, 82], OperandSize::Qword)
}

#[test]
fn vptestmd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTMD, operand1: Some(Direct(K4)), operand2: Some(Direct(ZMM7)), operand3: Some(Direct(ZMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 69, 77, 39, 224], OperandSize::Dword)
}

#[test]
fn vptestmd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTMD, operand1: Some(Direct(K5)), operand2: Some(Direct(ZMM2)), operand3: Some(IndirectDisplaced(EDX, 560125130, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 109, 75, 39, 170, 202, 212, 98, 33], OperandSize::Dword)
}

#[test]
fn vptestmd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTMD, operand1: Some(Direct(K7)), operand2: Some(Direct(ZMM1)), operand3: Some(IndirectDisplaced(ESI, 1158053049, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 242, 117, 95, 39, 190, 185, 124, 6, 69], OperandSize::Dword)
}

#[test]
fn vptestmd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTMD, operand1: Some(Direct(K4)), operand2: Some(Direct(ZMM23)), operand3: Some(Direct(ZMM29)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 146, 69, 71, 39, 229], OperandSize::Qword)
}

#[test]
fn vptestmd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTMD, operand1: Some(Direct(K5)), operand2: Some(Direct(ZMM4)), operand3: Some(IndirectScaledIndexedDisplaced(RBX, RDI, Eight, 1289565432, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 93, 73, 39, 172, 251, 248, 52, 221, 76], OperandSize::Qword)
}

#[test]
fn vptestmd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTMD, operand1: Some(Direct(K6)), operand2: Some(Direct(ZMM21)), operand3: Some(IndirectScaledIndexed(RDI, RCX, Four, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 242, 85, 86, 39, 52, 143], OperandSize::Qword)
}

