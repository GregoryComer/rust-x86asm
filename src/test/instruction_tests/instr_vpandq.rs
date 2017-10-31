use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpandq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDQ, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 205, 141, 219, 214], OperandSize::Dword)
}

#[test]
fn vpandq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDQ, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectDisplaced(ECX, 1339605723, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 245, 138, 219, 185, 219, 194, 216, 79], OperandSize::Dword)
}

#[test]
fn vpandq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDQ, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledDisplaced(ESI, Two, 908318729, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 241, 229, 153, 219, 44, 117, 9, 216, 35, 54], OperandSize::Dword)
}

#[test]
fn vpandq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDQ, operand1: Some(Direct(XMM25)), operand2: Some(Direct(XMM9)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 97, 181, 139, 219, 205], OperandSize::Qword)
}

#[test]
fn vpandq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDQ, operand1: Some(Direct(XMM11)), operand2: Some(Direct(XMM11)), operand3: Some(IndirectScaledDisplaced(RCX, Two, 2009630961, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 113, 165, 138, 219, 28, 77, 241, 136, 200, 119], OperandSize::Qword)
}

#[test]
fn vpandq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDQ, operand1: Some(Direct(XMM22)), operand2: Some(Direct(XMM18)), operand3: Some(IndirectScaledIndexed(RAX, RCX, Eight, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 225, 237, 148, 219, 52, 200], OperandSize::Qword)
}

#[test]
fn vpandq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDQ, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(YMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 213, 174, 219, 221], OperandSize::Dword)
}

#[test]
fn vpandq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDQ, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectScaledDisplaced(ECX, Two, 1270549506, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 197, 172, 219, 20, 77, 2, 12, 187, 75], OperandSize::Dword)
}

#[test]
fn vpandq_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDQ, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectScaledIndexed(EDI, EDI, Two, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 221, 187, 219, 44, 127], OperandSize::Dword)
}

#[test]
fn vpandq_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDQ, operand1: Some(Direct(YMM25)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(YMM12)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 65, 229, 170, 219, 204], OperandSize::Qword)
}

#[test]
fn vpandq_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDQ, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM25)), operand3: Some(IndirectScaledIndexed(RDX, RDI, Four, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 181, 167, 219, 12, 186], OperandSize::Qword)
}

#[test]
fn vpandq_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDQ, operand1: Some(Direct(YMM15)), operand2: Some(Direct(YMM29)), operand3: Some(Indirect(RSI, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 113, 149, 183, 219, 62], OperandSize::Qword)
}

#[test]
fn vpandq_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDQ, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM3)), operand3: Some(Direct(ZMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 229, 203, 219, 225], OperandSize::Dword)
}

#[test]
fn vpandq_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDQ, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM7)), operand3: Some(Indirect(EAX, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 197, 201, 219, 56], OperandSize::Dword)
}

#[test]
fn vpandq_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDQ, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM1)), operand3: Some(IndirectScaledIndexed(EDX, EDX, Two, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 245, 219, 219, 60, 82], OperandSize::Dword)
}

#[test]
fn vpandq_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDQ, operand1: Some(Direct(ZMM26)), operand2: Some(Direct(ZMM7)), operand3: Some(Direct(ZMM17)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 33, 197, 204, 219, 209], OperandSize::Qword)
}

#[test]
fn vpandq_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDQ, operand1: Some(Direct(ZMM18)), operand2: Some(Direct(ZMM9)), operand3: Some(IndirectScaledDisplaced(RSI, Eight, 806966922, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 225, 181, 202, 219, 20, 245, 138, 86, 25, 48], OperandSize::Qword)
}

#[test]
fn vpandq_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDQ, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM14)), operand3: Some(IndirectScaledIndexedDisplaced(RSI, RBX, Four, 666056355, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 141, 219, 219, 140, 158, 163, 54, 179, 39], OperandSize::Qword)
}

