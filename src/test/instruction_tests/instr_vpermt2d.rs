use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpermt2d_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2D, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 125, 138, 126, 233], OperandSize::Dword)
}

#[test]
fn vpermt2d_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2D, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectDisplaced(EAX, 2077061868, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 85, 142, 126, 168, 236, 114, 205, 123], OperandSize::Dword)
}

#[test]
fn vpermt2d_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2D, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledIndexed(ESI, EDI, Eight, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 125, 159, 126, 28, 254], OperandSize::Dword)
}

#[test]
fn vpermt2d_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2D, operand1: Some(Direct(XMM27)), operand2: Some(Direct(XMM10)), operand3: Some(Direct(XMM17)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 34, 45, 137, 126, 217], OperandSize::Qword)
}

#[test]
fn vpermt2d_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2D, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledIndexedDisplaced(RCX, RCX, Two, 874076556, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 101, 139, 126, 156, 73, 140, 89, 25, 52], OperandSize::Qword)
}

#[test]
fn vpermt2d_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2D, operand1: Some(Direct(XMM14)), operand2: Some(Direct(XMM23)), operand3: Some(Indirect(RSI, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 114, 69, 149, 126, 54], OperandSize::Qword)
}

#[test]
fn vpermt2d_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2D, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 93, 169, 126, 218], OperandSize::Dword)
}

#[test]
fn vpermt2d_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2D, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM0)), operand3: Some(Indirect(EBX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 125, 172, 126, 27], OperandSize::Dword)
}

#[test]
fn vpermt2d_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2D, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectScaledDisplaced(EDI, Eight, 1480196732, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 101, 187, 126, 12, 253, 124, 2, 58, 88], OperandSize::Dword)
}

#[test]
fn vpermt2d_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2D, operand1: Some(Direct(YMM18)), operand2: Some(Direct(YMM20)), operand3: Some(Direct(YMM12)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 194, 93, 164, 126, 212], OperandSize::Qword)
}

#[test]
fn vpermt2d_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2D, operand1: Some(Direct(YMM20)), operand2: Some(Direct(YMM16)), operand3: Some(Indirect(RAX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 226, 125, 162, 126, 32], OperandSize::Qword)
}

#[test]
fn vpermt2d_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2D, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectScaledDisplaced(RSI, Four, 121121644, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 69, 189, 126, 4, 181, 108, 43, 56, 7], OperandSize::Qword)
}

#[test]
fn vpermt2d_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2D, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM1)), operand3: Some(Direct(ZMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 117, 206, 126, 202], OperandSize::Dword)
}

#[test]
fn vpermt2d_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2D, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM1)), operand3: Some(Indirect(EAX, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 117, 202, 126, 48], OperandSize::Dword)
}

#[test]
fn vpermt2d_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2D, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM7)), operand3: Some(IndirectScaledIndexedDisplaced(EAX, EAX, Two, 1345193833, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 242, 69, 221, 126, 188, 64, 105, 7, 46, 80], OperandSize::Dword)
}

#[test]
fn vpermt2d_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2D, operand1: Some(Direct(ZMM9)), operand2: Some(Direct(ZMM5)), operand3: Some(Direct(ZMM15)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 82, 85, 202, 126, 207], OperandSize::Qword)
}

#[test]
fn vpermt2d_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2D, operand1: Some(Direct(ZMM27)), operand2: Some(Direct(ZMM7)), operand3: Some(IndirectScaledIndexed(RBX, RDI, Four, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 98, 69, 203, 126, 28, 187], OperandSize::Qword)
}

#[test]
fn vpermt2d_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2D, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM25)), operand3: Some(IndirectScaledIndexedDisplaced(RAX, RAX, Eight, 759713340, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 242, 53, 209, 126, 180, 192, 60, 78, 72, 45], OperandSize::Qword)
}

