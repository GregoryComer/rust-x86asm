use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vptestnmd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTNMD, operand1: Some(Direct(K3)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 70, 15, 39, 216], OperandSize::Dword)
}

#[test]
fn vptestnmd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTNMD, operand1: Some(Direct(K7)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectDisplaced(ECX, 1053335906, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 102, 12, 39, 185, 98, 161, 200, 62], OperandSize::Dword)
}

#[test]
fn vptestnmd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTNMD, operand1: Some(Direct(K4)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledIndexed(EDI, EBX, Eight, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 102, 25, 39, 36, 223], OperandSize::Dword)
}

#[test]
fn vptestnmd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTNMD, operand1: Some(Direct(K7)), operand2: Some(Direct(XMM13)), operand3: Some(Direct(XMM30)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 146, 22, 13, 39, 254], OperandSize::Qword)
}

#[test]
fn vptestnmd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTNMD, operand1: Some(Direct(K2)), operand2: Some(Direct(XMM16)), operand3: Some(IndirectScaledIndexed(RDX, RCX, Four, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 126, 6, 39, 20, 138], OperandSize::Qword)
}

#[test]
fn vptestnmd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTNMD, operand1: Some(Direct(K4)), operand2: Some(Direct(XMM26)), operand3: Some(IndirectScaledIndexedDisplaced(RDI, RDI, Four, 555335891, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 46, 20, 39, 164, 191, 211, 192, 25, 33], OperandSize::Qword)
}

#[test]
fn vptestnmd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTNMD, operand1: Some(Direct(K6)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 110, 44, 39, 246], OperandSize::Dword)
}

#[test]
fn vptestnmd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTNMD, operand1: Some(Direct(K4)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectScaledIndexed(EDX, ESI, Four, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 78, 47, 39, 36, 178], OperandSize::Dword)
}

#[test]
fn vptestnmd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTNMD, operand1: Some(Direct(K6)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectScaledIndexedDisplaced(EDI, EAX, Eight, 1277956709, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 110, 61, 39, 180, 199, 101, 18, 44, 76], OperandSize::Dword)
}

#[test]
fn vptestnmd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTNMD, operand1: Some(Direct(K2)), operand2: Some(Direct(YMM27)), operand3: Some(Direct(YMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 38, 38, 39, 210], OperandSize::Qword)
}

#[test]
fn vptestnmd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTNMD, operand1: Some(Direct(K6)), operand2: Some(Direct(YMM20)), operand3: Some(IndirectDisplaced(RBX, 2063632333, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 94, 36, 39, 179, 205, 135, 0, 123], OperandSize::Qword)
}

#[test]
fn vptestnmd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTNMD, operand1: Some(Direct(K1)), operand2: Some(Direct(YMM19)), operand3: Some(IndirectScaledIndexed(RDI, RBX, Four, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 102, 54, 39, 12, 159], OperandSize::Qword)
}

#[test]
fn vptestnmd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTNMD, operand1: Some(Direct(K7)), operand2: Some(Direct(ZMM7)), operand3: Some(Direct(ZMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 70, 73, 39, 251], OperandSize::Dword)
}

#[test]
fn vptestnmd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTNMD, operand1: Some(Direct(K6)), operand2: Some(Direct(ZMM6)), operand3: Some(IndirectScaledIndexedDisplaced(EBX, EDI, Two, 991422385, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 78, 73, 39, 180, 123, 177, 231, 23, 59], OperandSize::Dword)
}

#[test]
fn vptestnmd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTNMD, operand1: Some(Direct(K1)), operand2: Some(Direct(ZMM4)), operand3: Some(Indirect(ECX, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 242, 94, 94, 39, 9], OperandSize::Dword)
}

#[test]
fn vptestnmd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTNMD, operand1: Some(Direct(K2)), operand2: Some(Direct(ZMM22)), operand3: Some(Direct(ZMM27)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 146, 78, 68, 39, 211], OperandSize::Qword)
}

#[test]
fn vptestnmd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTNMD, operand1: Some(Direct(K2)), operand2: Some(Direct(ZMM31)), operand3: Some(IndirectScaledIndexed(RAX, RSI, Eight, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 6, 70, 39, 20, 240], OperandSize::Qword)
}

#[test]
fn vptestnmd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTNMD, operand1: Some(Direct(K3)), operand2: Some(Direct(ZMM0)), operand3: Some(IndirectDisplaced(RCX, 939532495, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 242, 126, 89, 39, 153, 207, 32, 0, 56], OperandSize::Qword)
}

