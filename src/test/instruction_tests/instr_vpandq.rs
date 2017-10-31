use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpandq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDQ, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 205, 139, 219, 207], OperandSize::Dword)
}

#[test]
fn vpandq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDQ, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledIndexedDisplaced(EDX, EDX, Four, 824332011, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 197, 141, 219, 164, 146, 235, 78, 34, 49], OperandSize::Dword)
}

#[test]
fn vpandq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDQ, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledIndexed(EDX, ECX, Four, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 241, 229, 154, 219, 36, 138], OperandSize::Dword)
}

#[test]
fn vpandq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDQ, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM28)), operand3: Some(Direct(XMM24)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 145, 157, 132, 219, 224], OperandSize::Qword)
}

#[test]
fn vpandq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDQ, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM25)), operand3: Some(Indirect(RDX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 181, 130, 219, 34], OperandSize::Qword)
}

#[test]
fn vpandq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDQ, operand1: Some(Direct(XMM10)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledDisplaced(RDI, Eight, 1569560625, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 113, 197, 154, 219, 20, 253, 49, 152, 141, 93], OperandSize::Qword)
}

#[test]
fn vpandq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDQ, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 221, 174, 219, 194], OperandSize::Dword)
}

#[test]
fn vpandq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDQ, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectScaledIndexed(ECX, EDI, Two, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 197, 175, 219, 12, 121], OperandSize::Dword)
}

#[test]
fn vpandq_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDQ, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectDisplaced(EAX, 481471957, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 213, 185, 219, 128, 213, 173, 178, 28], OperandSize::Dword)
}

#[test]
fn vpandq_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDQ, operand1: Some(Direct(YMM30)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(YMM25)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 1, 253, 175, 219, 241], OperandSize::Qword)
}

#[test]
fn vpandq_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDQ, operand1: Some(Direct(YMM22)), operand2: Some(Direct(YMM27)), operand3: Some(IndirectScaledIndexed(RSI, RDX, Eight, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 225, 165, 162, 219, 52, 214], OperandSize::Qword)
}

#[test]
fn vpandq_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDQ, operand1: Some(Direct(YMM11)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectScaledIndexed(RSI, RDX, Four, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 113, 197, 187, 219, 28, 150], OperandSize::Qword)
}

#[test]
fn vpandq_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDQ, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM6)), operand3: Some(Direct(ZMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 205, 201, 219, 218], OperandSize::Dword)
}

#[test]
fn vpandq_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDQ, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM2)), operand3: Some(IndirectScaledIndexed(ESI, ECX, Two, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 237, 204, 219, 44, 78], OperandSize::Dword)
}

#[test]
fn vpandq_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDQ, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM3)), operand3: Some(IndirectDisplaced(EDX, 674296130, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 229, 223, 219, 146, 66, 241, 48, 40], OperandSize::Dword)
}

#[test]
fn vpandq_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDQ, operand1: Some(Direct(ZMM13)), operand2: Some(Direct(ZMM14)), operand3: Some(Direct(ZMM15)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 81, 141, 207, 219, 239], OperandSize::Qword)
}

#[test]
fn vpandq_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDQ, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM21)), operand3: Some(IndirectScaledIndexed(RBX, RCX, Two, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 213, 194, 219, 20, 75], OperandSize::Qword)
}

#[test]
fn vpandq_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDQ, operand1: Some(Direct(ZMM18)), operand2: Some(Direct(ZMM18)), operand3: Some(Indirect(RAX, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 225, 237, 209, 219, 16], OperandSize::Qword)
}

