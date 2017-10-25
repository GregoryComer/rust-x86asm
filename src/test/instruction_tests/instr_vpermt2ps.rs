use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpermt2ps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2PS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 69, 141, 127, 241], OperandSize::Dword)
}

#[test]
fn vpermt2ps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2PS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectScaledDisplaced(EDI, Four, 1669870676, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 117, 143, 127, 52, 189, 84, 52, 136, 99], OperandSize::Dword)
}

#[test]
fn vpermt2ps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2PS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectDisplaced(ECX, 409447703, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 101, 156, 127, 145, 23, 173, 103, 24], OperandSize::Dword)
}

#[test]
fn vpermt2ps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2PS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM31)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 146, 109, 139, 127, 255], OperandSize::Qword)
}

#[test]
fn vpermt2ps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2PS, operand1: Some(Direct(XMM22)), operand2: Some(Direct(XMM16)), operand3: Some(IndirectScaledIndexedDisplaced(RDI, RBX, Four, 856689936, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 226, 125, 133, 127, 180, 159, 16, 13, 16, 51], OperandSize::Qword)
}

#[test]
fn vpermt2ps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2PS, operand1: Some(Direct(XMM20)), operand2: Some(Direct(XMM10)), operand3: Some(IndirectDisplaced(RSI, 678594216, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 226, 45, 153, 127, 166, 168, 134, 114, 40], OperandSize::Qword)
}

#[test]
fn vpermt2ps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2PS, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 109, 169, 127, 211], OperandSize::Dword)
}

#[test]
fn vpermt2ps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2PS, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectScaledIndexed(ECX, ESI, Two, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 93, 173, 127, 60, 113], OperandSize::Dword)
}

#[test]
fn vpermt2ps_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2PS, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM3)), operand3: Some(Indirect(ESI, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 101, 188, 127, 38], OperandSize::Dword)
}

#[test]
fn vpermt2ps_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2PS, operand1: Some(Direct(YMM24)), operand2: Some(Direct(YMM10)), operand3: Some(Direct(YMM29)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 2, 45, 172, 127, 197], OperandSize::Qword)
}

#[test]
fn vpermt2ps_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2PS, operand1: Some(Direct(YMM25)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectScaledIndexedDisplaced(RSI, RDI, Four, 692223628, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 98, 109, 173, 127, 140, 190, 140, 126, 66, 41], OperandSize::Qword)
}

#[test]
fn vpermt2ps_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2PS, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectDisplaced(RSI, 38469805, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 117, 191, 127, 166, 173, 0, 75, 2], OperandSize::Qword)
}

#[test]
fn vpermt2ps_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2PS, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM4)), operand3: Some(Direct(ZMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 93, 201, 127, 250], OperandSize::Dword)
}

#[test]
fn vpermt2ps_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2PS, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM1)), operand3: Some(Indirect(ESI, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 117, 202, 127, 62], OperandSize::Dword)
}

#[test]
fn vpermt2ps_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2PS, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM2)), operand3: Some(IndirectScaledIndexed(ECX, ECX, Two, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 242, 109, 219, 127, 28, 73], OperandSize::Dword)
}

#[test]
fn vpermt2ps_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2PS, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM6)), operand3: Some(Direct(ZMM21)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 178, 77, 203, 127, 253], OperandSize::Qword)
}

#[test]
fn vpermt2ps_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2PS, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM19)), operand3: Some(IndirectScaledDisplaced(RDX, Eight, 1254182588, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 101, 199, 127, 28, 213, 188, 78, 193, 74], OperandSize::Qword)
}

#[test]
fn vpermt2ps_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2PS, operand1: Some(Direct(ZMM25)), operand2: Some(Direct(ZMM31)), operand3: Some(IndirectDisplaced(RDX, 148595594, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 98, 5, 209, 127, 138, 138, 99, 219, 8], OperandSize::Qword)
}

