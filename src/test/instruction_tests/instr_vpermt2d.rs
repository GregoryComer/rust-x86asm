use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpermt2d_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2D, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 77, 139, 126, 193], OperandSize::Dword)
}

#[test]
fn vpermt2d_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2D, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledDisplaced(ESI, Two, 2129615628, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 85, 137, 126, 36, 117, 12, 91, 239, 126], OperandSize::Dword)
}

#[test]
fn vpermt2d_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2D, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledDisplaced(EDI, Eight, 861285785, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 125, 157, 126, 60, 253, 153, 45, 86, 51], OperandSize::Dword)
}

#[test]
fn vpermt2d_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2D, operand1: Some(Direct(XMM24)), operand2: Some(Direct(XMM15)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 98, 5, 137, 126, 195], OperandSize::Qword)
}

#[test]
fn vpermt2d_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2D, operand1: Some(Direct(XMM31)), operand2: Some(Direct(XMM31)), operand3: Some(IndirectScaledIndexed(RDI, RAX, Eight, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 98, 5, 133, 126, 60, 199], OperandSize::Qword)
}

#[test]
fn vpermt2d_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2D, operand1: Some(Direct(XMM17)), operand2: Some(Direct(XMM1)), operand3: Some(Indirect(RBX, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 226, 117, 153, 126, 11], OperandSize::Qword)
}

#[test]
fn vpermt2d_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2D, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 117, 169, 126, 227], OperandSize::Dword)
}

#[test]
fn vpermt2d_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2D, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectScaledIndexedDisplaced(ECX, EDX, Eight, 366218286, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 125, 170, 126, 188, 209, 46, 12, 212, 21], OperandSize::Dword)
}

#[test]
fn vpermt2d_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2D, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM3)), operand3: Some(Indirect(ECX, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 101, 186, 126, 41], OperandSize::Dword)
}

#[test]
fn vpermt2d_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2D, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM23)), operand3: Some(Direct(YMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 69, 162, 126, 201], OperandSize::Qword)
}

#[test]
fn vpermt2d_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2D, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM4)), operand3: Some(Indirect(RBX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 93, 175, 126, 11], OperandSize::Qword)
}

#[test]
fn vpermt2d_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2D, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM27)), operand3: Some(Indirect(RDI, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 37, 181, 126, 31], OperandSize::Qword)
}

#[test]
fn vpermt2d_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2D, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM2)), operand3: Some(Direct(ZMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 109, 204, 126, 197], OperandSize::Dword)
}

#[test]
fn vpermt2d_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2D, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM3)), operand3: Some(IndirectScaledIndexedDisplaced(EDI, EAX, Four, 1388166910, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 101, 202, 126, 188, 135, 254, 190, 189, 82], OperandSize::Dword)
}

#[test]
fn vpermt2d_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2D, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM3)), operand3: Some(Indirect(ECX, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 242, 101, 221, 126, 17], OperandSize::Dword)
}

#[test]
fn vpermt2d_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2D, operand1: Some(Direct(ZMM14)), operand2: Some(Direct(ZMM19)), operand3: Some(Direct(ZMM25)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 18, 101, 193, 126, 241], OperandSize::Qword)
}

#[test]
fn vpermt2d_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2D, operand1: Some(Direct(ZMM17)), operand2: Some(Direct(ZMM16)), operand3: Some(Indirect(RSI, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 226, 125, 196, 126, 14], OperandSize::Qword)
}

#[test]
fn vpermt2d_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2D, operand1: Some(Direct(ZMM19)), operand2: Some(Direct(ZMM19)), operand3: Some(IndirectDisplaced(RCX, 1742072234, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 226, 101, 210, 126, 153, 170, 233, 213, 103], OperandSize::Qword)
}

