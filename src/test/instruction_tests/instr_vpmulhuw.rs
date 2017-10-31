use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpmulhuw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULHUW, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 233, 228, 202], OperandSize::Dword)
}

#[test]
fn vpmulhuw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULHUW, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectDisplaced(EAX, 1732950149, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 241, 228, 168, 133, 184, 74, 103], OperandSize::Dword)
}

#[test]
fn vpmulhuw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULHUW, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 225, 228, 210], OperandSize::Qword)
}

#[test]
fn vpmulhuw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULHUW, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledIndexedDisplaced(RAX, RSI, Four, 819374641, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 225, 228, 180, 176, 49, 170, 214, 48], OperandSize::Qword)
}

#[test]
fn vpmulhuw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULHUW, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 197, 228, 211], OperandSize::Dword)
}

#[test]
fn vpmulhuw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULHUW, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectScaledIndexedDisplaced(ECX, EBX, Eight, 1645259245, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 221, 228, 148, 217, 237, 169, 16, 98], OperandSize::Dword)
}

#[test]
fn vpmulhuw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULHUW, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(YMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 205, 228, 229], OperandSize::Qword)
}

#[test]
fn vpmulhuw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULHUW, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectScaledIndexed(RCX, RCX, Four, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 229, 228, 44, 137], OperandSize::Qword)
}

#[test]
fn vpmulhuw_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULHUW, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 125, 139, 228, 193], OperandSize::Dword)
}

#[test]
fn vpmulhuw_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULHUW, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledDisplaced(EAX, Two, 1563835845, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 85, 138, 228, 28, 69, 197, 61, 54, 93], OperandSize::Dword)
}

#[test]
fn vpmulhuw_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULHUW, operand1: Some(Direct(XMM27)), operand2: Some(Direct(XMM25)), operand3: Some(Direct(XMM26)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 1, 53, 130, 228, 218], OperandSize::Qword)
}

#[test]
fn vpmulhuw_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULHUW, operand1: Some(Direct(XMM25)), operand2: Some(Direct(XMM13)), operand3: Some(IndirectScaledDisplaced(RDI, Four, 382502070, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 97, 21, 140, 228, 12, 189, 182, 132, 204, 22], OperandSize::Qword)
}

#[test]
fn vpmulhuw_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULHUW, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(YMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 125, 171, 228, 240], OperandSize::Dword)
}

#[test]
fn vpmulhuw_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULHUW, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectDisplaced(EAX, 327899942, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 109, 172, 228, 168, 38, 91, 139, 19], OperandSize::Dword)
}

#[test]
fn vpmulhuw_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULHUW, operand1: Some(Direct(YMM18)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(YMM18)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 161, 101, 173, 228, 210], OperandSize::Qword)
}

#[test]
fn vpmulhuw_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULHUW, operand1: Some(Direct(YMM10)), operand2: Some(Direct(YMM20)), operand3: Some(IndirectDisplaced(RBX, 580310561, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 113, 93, 164, 228, 147, 33, 214, 150, 34], OperandSize::Qword)
}

#[test]
fn vpmulhuw_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULHUW, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM0)), operand3: Some(Direct(ZMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 125, 203, 228, 243], OperandSize::Dword)
}

#[test]
fn vpmulhuw_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULHUW, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM0)), operand3: Some(IndirectScaledIndexedDisplaced(ECX, EDI, Four, 350387922, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 125, 206, 228, 132, 185, 210, 126, 226, 20], OperandSize::Dword)
}

#[test]
fn vpmulhuw_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULHUW, operand1: Some(Direct(ZMM20)), operand2: Some(Direct(ZMM27)), operand3: Some(Direct(ZMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 225, 37, 198, 228, 231], OperandSize::Qword)
}

#[test]
fn vpmulhuw_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULHUW, operand1: Some(Direct(ZMM15)), operand2: Some(Direct(ZMM23)), operand3: Some(IndirectDisplaced(RAX, 956714644, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 113, 69, 193, 228, 184, 148, 78, 6, 57], OperandSize::Qword)
}

