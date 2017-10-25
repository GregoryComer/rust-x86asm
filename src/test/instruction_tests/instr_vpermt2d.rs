use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpermt2d_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2D, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 109, 140, 126, 224], OperandSize::Dword)
}

#[test]
fn vpermt2d_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2D, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledIndexedDisplaced(ESI, EAX, Four, 2067208870, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 69, 139, 126, 180, 134, 166, 26, 55, 123], OperandSize::Dword)
}

#[test]
fn vpermt2d_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2D, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledIndexed(ECX, ESI, Four, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 69, 156, 126, 60, 177], OperandSize::Dword)
}

#[test]
fn vpermt2d_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2D, operand1: Some(Direct(XMM24)), operand2: Some(Direct(XMM25)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 98, 53, 131, 126, 199], OperandSize::Qword)
}

#[test]
fn vpermt2d_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2D, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM14)), operand3: Some(IndirectDisplaced(RDI, 1534332371, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 13, 143, 126, 135, 211, 13, 116, 91], OperandSize::Qword)
}

#[test]
fn vpermt2d_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2D, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM26)), operand3: Some(IndirectDisplaced(RBX, 1103150490, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 45, 147, 126, 163, 154, 189, 192, 65], OperandSize::Qword)
}

#[test]
fn vpermt2d_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2D, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(YMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 85, 170, 126, 226], OperandSize::Dword)
}

#[test]
fn vpermt2d_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2D, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM2)), operand3: Some(Indirect(ESI, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 109, 171, 126, 14], OperandSize::Dword)
}

#[test]
fn vpermt2d_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2D, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectDisplaced(EAX, 979567054, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 77, 186, 126, 128, 206, 1, 99, 58], OperandSize::Dword)
}

#[test]
fn vpermt2d_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2D, operand1: Some(Direct(YMM12)), operand2: Some(Direct(YMM8)), operand3: Some(Direct(YMM17)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 50, 61, 172, 126, 225], OperandSize::Qword)
}

#[test]
fn vpermt2d_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2D, operand1: Some(Direct(YMM28)), operand2: Some(Direct(YMM19)), operand3: Some(IndirectScaledDisplaced(RDI, Eight, 507733573, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 98, 101, 162, 126, 36, 253, 69, 102, 67, 30], OperandSize::Qword)
}

#[test]
fn vpermt2d_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2D, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM18)), operand3: Some(Indirect(RDI, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 109, 180, 126, 63], OperandSize::Qword)
}

#[test]
fn vpermt2d_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2D, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM3)), operand3: Some(Direct(ZMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 101, 202, 126, 219], OperandSize::Dword)
}

#[test]
fn vpermt2d_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2D, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM3)), operand3: Some(IndirectDisplaced(EDI, 405038489, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 101, 202, 126, 183, 153, 101, 36, 24], OperandSize::Dword)
}

#[test]
fn vpermt2d_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2D, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM6)), operand3: Some(IndirectDisplaced(EDX, 180235210, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 242, 77, 220, 126, 178, 202, 43, 190, 10], OperandSize::Dword)
}

#[test]
fn vpermt2d_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2D, operand1: Some(Direct(ZMM31)), operand2: Some(Direct(ZMM17)), operand3: Some(Direct(ZMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 98, 117, 196, 126, 248], OperandSize::Qword)
}

#[test]
fn vpermt2d_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2D, operand1: Some(Direct(ZMM10)), operand2: Some(Direct(ZMM30)), operand3: Some(IndirectScaledIndexedDisplaced(RDX, RBX, Four, 1426778095, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 114, 13, 196, 126, 148, 154, 239, 231, 10, 85], OperandSize::Qword)
}

#[test]
fn vpermt2d_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2D, operand1: Some(Direct(ZMM31)), operand2: Some(Direct(ZMM16)), operand3: Some(IndirectScaledIndexedDisplaced(RDX, RSI, Eight, 1917264461, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 98, 125, 210, 126, 188, 242, 77, 34, 71, 114], OperandSize::Qword)
}

