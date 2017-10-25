use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vptestmd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTMD, operand1: Some(Direct(K2)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 101, 9, 39, 210], OperandSize::Dword)
}

#[test]
fn vptestmd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTMD, operand1: Some(Direct(K2)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledDisplaced(EBX, Eight, 74831441, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 101, 9, 39, 20, 221, 81, 214, 117, 4], OperandSize::Dword)
}

#[test]
fn vptestmd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTMD, operand1: Some(Direct(K1)), operand2: Some(Direct(XMM5)), operand3: Some(Indirect(ESI, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 85, 26, 39, 14], OperandSize::Dword)
}

#[test]
fn vptestmd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTMD, operand1: Some(Direct(K6)), operand2: Some(Direct(XMM14)), operand3: Some(Direct(XMM15)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 210, 13, 15, 39, 247], OperandSize::Qword)
}

#[test]
fn vptestmd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTMD, operand1: Some(Direct(K5)), operand2: Some(Direct(XMM27)), operand3: Some(IndirectDisplaced(RDI, 157669945, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 37, 7, 39, 175, 57, 218, 101, 9], OperandSize::Qword)
}

#[test]
fn vptestmd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTMD, operand1: Some(Direct(K7)), operand2: Some(Direct(XMM26)), operand3: Some(IndirectDisplaced(RSI, 833364985, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 45, 22, 39, 190, 249, 35, 172, 49], OperandSize::Qword)
}

#[test]
fn vptestmd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTMD, operand1: Some(Direct(K1)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 69, 47, 39, 202], OperandSize::Dword)
}

#[test]
fn vptestmd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTMD, operand1: Some(Direct(K3)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectDisplaced(ESI, 351666789, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 93, 44, 39, 158, 101, 2, 246, 20], OperandSize::Dword)
}

#[test]
fn vptestmd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTMD, operand1: Some(Direct(K7)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectScaledIndexed(ESI, EDX, Two, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 77, 57, 39, 60, 86], OperandSize::Dword)
}

#[test]
fn vptestmd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTMD, operand1: Some(Direct(K3)), operand2: Some(Direct(YMM17)), operand3: Some(Direct(YMM8)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 210, 117, 36, 39, 216], OperandSize::Qword)
}

#[test]
fn vptestmd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTMD, operand1: Some(Direct(K6)), operand2: Some(Direct(YMM30)), operand3: Some(Indirect(RDX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 13, 36, 39, 50], OperandSize::Qword)
}

#[test]
fn vptestmd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTMD, operand1: Some(Direct(K3)), operand2: Some(Direct(YMM15)), operand3: Some(IndirectScaledDisplaced(RDX, Four, 924366498, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 5, 62, 39, 28, 149, 162, 182, 24, 55], OperandSize::Qword)
}

#[test]
fn vptestmd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTMD, operand1: Some(Direct(K3)), operand2: Some(Direct(ZMM3)), operand3: Some(Direct(ZMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 101, 76, 39, 223], OperandSize::Dword)
}

#[test]
fn vptestmd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTMD, operand1: Some(Direct(K2)), operand2: Some(Direct(ZMM3)), operand3: Some(Indirect(EDX, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 101, 79, 39, 18], OperandSize::Dword)
}

#[test]
fn vptestmd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTMD, operand1: Some(Direct(K5)), operand2: Some(Direct(ZMM1)), operand3: Some(Indirect(ECX, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 242, 117, 93, 39, 41], OperandSize::Dword)
}

#[test]
fn vptestmd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTMD, operand1: Some(Direct(K6)), operand2: Some(Direct(ZMM0)), operand3: Some(Direct(ZMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 125, 76, 39, 247], OperandSize::Qword)
}

#[test]
fn vptestmd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTMD, operand1: Some(Direct(K4)), operand2: Some(Direct(ZMM9)), operand3: Some(IndirectScaledIndexed(RBX, RDX, Four, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 53, 79, 39, 36, 147], OperandSize::Qword)
}

#[test]
fn vptestmd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTMD, operand1: Some(Direct(K1)), operand2: Some(Direct(ZMM15)), operand3: Some(IndirectScaledIndexed(RAX, RSI, Eight, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 242, 5, 93, 39, 12, 240], OperandSize::Qword)
}

