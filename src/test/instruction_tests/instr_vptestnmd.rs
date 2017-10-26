use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vptestnmd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTNMD, operand1: Some(Direct(K6)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 86, 12, 39, 241], OperandSize::Dword)
}

#[test]
fn vptestnmd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTNMD, operand1: Some(Direct(K7)), operand2: Some(Direct(XMM2)), operand3: Some(Indirect(ECX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 110, 13, 39, 57], OperandSize::Dword)
}

#[test]
fn vptestnmd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTNMD, operand1: Some(Direct(K7)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledIndexedDisplaced(ESI, EBX, Four, 244401386, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 70, 25, 39, 188, 158, 234, 68, 145, 14], OperandSize::Dword)
}

#[test]
fn vptestnmd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTNMD, operand1: Some(Direct(K1)), operand2: Some(Direct(XMM31)), operand3: Some(Direct(XMM28)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 146, 6, 3, 39, 204], OperandSize::Qword)
}

#[test]
fn vptestnmd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTNMD, operand1: Some(Direct(K6)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledIndexed(RBX, RAX, Two, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 70, 10, 39, 52, 67], OperandSize::Qword)
}

#[test]
fn vptestnmd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTNMD, operand1: Some(Direct(K7)), operand2: Some(Direct(XMM22)), operand3: Some(Indirect(RAX, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 78, 18, 39, 56], OperandSize::Qword)
}

#[test]
fn vptestnmd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTNMD, operand1: Some(Direct(K6)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 70, 42, 39, 244], OperandSize::Dword)
}

#[test]
fn vptestnmd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTNMD, operand1: Some(Direct(K2)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectScaledIndexedDisplaced(ESI, EAX, Eight, 672325123, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 94, 41, 39, 148, 198, 3, 222, 18, 40], OperandSize::Dword)
}

#[test]
fn vptestnmd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTNMD, operand1: Some(Direct(K5)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectScaledDisplaced(EDI, Four, 1641222241, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 118, 61, 39, 44, 189, 97, 16, 211, 97], OperandSize::Dword)
}

#[test]
fn vptestnmd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTNMD, operand1: Some(Direct(K6)), operand2: Some(Direct(YMM22)), operand3: Some(Direct(YMM22)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 178, 78, 38, 39, 246], OperandSize::Qword)
}

#[test]
fn vptestnmd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTNMD, operand1: Some(Direct(K1)), operand2: Some(Direct(YMM29)), operand3: Some(IndirectScaledIndexedDisplaced(RCX, RAX, Eight, 1574848092, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 22, 33, 39, 140, 193, 92, 70, 222, 93], OperandSize::Qword)
}

#[test]
fn vptestnmd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTNMD, operand1: Some(Direct(K6)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectScaledDisplaced(RDX, Four, 2042197811, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 102, 59, 39, 52, 149, 51, 119, 185, 121], OperandSize::Qword)
}

#[test]
fn vptestnmd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTNMD, operand1: Some(Direct(K3)), operand2: Some(Direct(ZMM6)), operand3: Some(Direct(ZMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 78, 76, 39, 218], OperandSize::Dword)
}

#[test]
fn vptestnmd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTNMD, operand1: Some(Direct(K7)), operand2: Some(Direct(ZMM6)), operand3: Some(IndirectScaledIndexed(EAX, EDX, Eight, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 78, 77, 39, 60, 208], OperandSize::Dword)
}

#[test]
fn vptestnmd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTNMD, operand1: Some(Direct(K4)), operand2: Some(Direct(ZMM5)), operand3: Some(IndirectScaledIndexed(EAX, EBX, Two, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 242, 86, 90, 39, 36, 88], OperandSize::Dword)
}

#[test]
fn vptestnmd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTNMD, operand1: Some(Direct(K2)), operand2: Some(Direct(ZMM23)), operand3: Some(Direct(ZMM13)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 210, 70, 65, 39, 213], OperandSize::Qword)
}

#[test]
fn vptestnmd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTNMD, operand1: Some(Direct(K6)), operand2: Some(Direct(ZMM5)), operand3: Some(IndirectDisplaced(RAX, 1145027369, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 86, 76, 39, 176, 41, 187, 63, 68], OperandSize::Qword)
}

#[test]
fn vptestnmd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTNMD, operand1: Some(Direct(K6)), operand2: Some(Direct(ZMM12)), operand3: Some(IndirectScaledIndexedDisplaced(RDI, RDX, Eight, 1658315046, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 242, 30, 93, 39, 180, 215, 38, 225, 215, 98], OperandSize::Qword)
}

