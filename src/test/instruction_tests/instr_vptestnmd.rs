use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vptestnmd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTNMD, operand1: Some(Direct(K2)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 86, 10, 39, 211], OperandSize::Dword)
}

#[test]
fn vptestnmd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTNMD, operand1: Some(Direct(K3)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectScaledDisplaced(EDX, Four, 48357058, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 118, 9, 39, 28, 149, 194, 222, 225, 2], OperandSize::Dword)
}

#[test]
fn vptestnmd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTNMD, operand1: Some(Direct(K4)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectDisplaced(EBX, 805585348, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 102, 25, 39, 163, 196, 65, 4, 48], OperandSize::Dword)
}

#[test]
fn vptestnmd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTNMD, operand1: Some(Direct(K2)), operand2: Some(Direct(XMM29)), operand3: Some(Direct(XMM9)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 210, 22, 6, 39, 209], OperandSize::Qword)
}

#[test]
fn vptestnmd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTNMD, operand1: Some(Direct(K4)), operand2: Some(Direct(XMM2)), operand3: Some(Indirect(RBX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 110, 11, 39, 35], OperandSize::Qword)
}

#[test]
fn vptestnmd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTNMD, operand1: Some(Direct(K4)), operand2: Some(Direct(XMM9)), operand3: Some(IndirectScaledIndexedDisplaced(RAX, RBX, Two, 600690265, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 54, 29, 39, 164, 88, 89, 206, 205, 35], OperandSize::Qword)
}

#[test]
fn vptestnmd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTNMD, operand1: Some(Direct(K7)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 118, 46, 39, 249], OperandSize::Dword)
}

#[test]
fn vptestnmd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTNMD, operand1: Some(Direct(K5)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectDisplaced(EBX, 2131542194, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 110, 45, 39, 171, 178, 192, 12, 127], OperandSize::Dword)
}

#[test]
fn vptestnmd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTNMD, operand1: Some(Direct(K3)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectDisplaced(EDI, 1730392046, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 110, 57, 39, 159, 238, 175, 35, 103], OperandSize::Dword)
}

#[test]
fn vptestnmd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTNMD, operand1: Some(Direct(K5)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(YMM13)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 210, 102, 44, 39, 237], OperandSize::Qword)
}

#[test]
fn vptestnmd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTNMD, operand1: Some(Direct(K3)), operand2: Some(Direct(YMM11)), operand3: Some(Indirect(RDI, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 38, 42, 39, 31], OperandSize::Qword)
}

#[test]
fn vptestnmd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTNMD, operand1: Some(Direct(K2)), operand2: Some(Direct(YMM12)), operand3: Some(IndirectDisplaced(RAX, 295477381, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 30, 57, 39, 144, 133, 160, 156, 17], OperandSize::Qword)
}

#[test]
fn vptestnmd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTNMD, operand1: Some(Direct(K6)), operand2: Some(Direct(ZMM1)), operand3: Some(Direct(ZMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 118, 74, 39, 244], OperandSize::Dword)
}

#[test]
fn vptestnmd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTNMD, operand1: Some(Direct(K7)), operand2: Some(Direct(ZMM0)), operand3: Some(Indirect(ESI, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 126, 78, 39, 62], OperandSize::Dword)
}

#[test]
fn vptestnmd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTNMD, operand1: Some(Direct(K3)), operand2: Some(Direct(ZMM5)), operand3: Some(IndirectScaledDisplaced(ECX, Eight, 584283170, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 242, 86, 94, 39, 28, 205, 34, 116, 211, 34], OperandSize::Dword)
}

#[test]
fn vptestnmd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTNMD, operand1: Some(Direct(K1)), operand2: Some(Direct(ZMM22)), operand3: Some(Direct(ZMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 78, 67, 39, 202], OperandSize::Qword)
}

#[test]
fn vptestnmd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTNMD, operand1: Some(Direct(K3)), operand2: Some(Direct(ZMM13)), operand3: Some(IndirectScaledDisplaced(RBX, Eight, 3919309, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 22, 78, 39, 28, 221, 205, 205, 59, 0], OperandSize::Qword)
}

#[test]
fn vptestnmd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTNMD, operand1: Some(Direct(K3)), operand2: Some(Direct(ZMM30)), operand3: Some(IndirectScaledIndexedDisplaced(RDI, RAX, Two, 1489018713, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 242, 14, 81, 39, 156, 71, 89, 159, 192, 88], OperandSize::Qword)
}

