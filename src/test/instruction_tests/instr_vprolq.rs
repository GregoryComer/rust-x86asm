use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vprolq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLQ, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM0)), operand3: Some(Literal8(10)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 253, 137, 114, 200, 10], OperandSize::Dword)
}

#[test]
fn vprolq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLQ, operand1: Some(Direct(XMM0)), operand2: Some(Indirect(EDI, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(13)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 253, 139, 114, 15, 13], OperandSize::Dword)
}

#[test]
fn vprolq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLQ, operand1: Some(Direct(XMM5)), operand2: Some(Indirect(EAX, Some(OperandSize::Qword), None)), operand3: Some(Literal8(117)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 241, 213, 156, 114, 8, 117], OperandSize::Dword)
}

#[test]
fn vprolq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLQ, operand1: Some(Direct(XMM18)), operand2: Some(Direct(XMM18)), operand3: Some(Literal8(122)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 177, 237, 135, 114, 202, 122], OperandSize::Qword)
}

#[test]
fn vprolq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLQ, operand1: Some(Direct(XMM18)), operand2: Some(IndirectScaledIndexed(RDI, RCX, Two, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(118)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 237, 130, 114, 12, 79, 118], OperandSize::Qword)
}

#[test]
fn vprolq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLQ, operand1: Some(Direct(XMM15)), operand2: Some(IndirectScaledDisplaced(RCX, Two, 1946706769, Some(OperandSize::Qword), None)), operand3: Some(Literal8(98)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 241, 133, 153, 114, 12, 77, 81, 99, 8, 116, 98], OperandSize::Qword)
}

#[test]
fn vprolq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLQ, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM1)), operand3: Some(Literal8(31)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 237, 171, 114, 201, 31], OperandSize::Dword)
}

#[test]
fn vprolq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLQ, operand1: Some(Direct(YMM6)), operand2: Some(IndirectScaledIndexed(EDI, ESI, Two, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(111)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 205, 172, 114, 12, 119, 111], OperandSize::Dword)
}

#[test]
fn vprolq_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLQ, operand1: Some(Direct(YMM5)), operand2: Some(IndirectScaledIndexed(ESI, ECX, Eight, Some(OperandSize::Qword), None)), operand3: Some(Literal8(97)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 213, 191, 114, 12, 206, 97], OperandSize::Dword)
}

#[test]
fn vprolq_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLQ, operand1: Some(Direct(YMM12)), operand2: Some(Direct(YMM21)), operand3: Some(Literal8(23)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 177, 157, 170, 114, 205, 23], OperandSize::Qword)
}

#[test]
fn vprolq_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLQ, operand1: Some(Direct(YMM3)), operand2: Some(IndirectDisplaced(RDI, 639661575, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(24)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 229, 172, 114, 143, 7, 118, 32, 38, 24], OperandSize::Qword)
}

#[test]
fn vprolq_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLQ, operand1: Some(Direct(YMM3)), operand2: Some(IndirectDisplaced(RBX, 716774701, Some(OperandSize::Qword), None)), operand3: Some(Literal8(34)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 229, 189, 114, 139, 45, 29, 185, 42, 34], OperandSize::Qword)
}

#[test]
fn vprolq_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLQ, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM5)), operand3: Some(Literal8(17)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 205, 205, 114, 205, 17], OperandSize::Dword)
}

#[test]
fn vprolq_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLQ, operand1: Some(Direct(ZMM5)), operand2: Some(IndirectScaledIndexedDisplaced(EDX, EAX, Four, 327012532, Some(OperandSize::Zmmword), None)), operand3: Some(Literal8(34)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 213, 207, 114, 140, 130, 180, 208, 125, 19, 34], OperandSize::Dword)
}

#[test]
fn vprolq_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLQ, operand1: Some(Direct(ZMM7)), operand2: Some(IndirectScaledDisplaced(EBX, Eight, 1612678962, Some(OperandSize::Qword), None)), operand3: Some(Literal8(112)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 197, 221, 114, 12, 221, 50, 135, 31, 96, 112], OperandSize::Dword)
}

#[test]
fn vprolq_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLQ, operand1: Some(Direct(ZMM23)), operand2: Some(Direct(ZMM23)), operand3: Some(Literal8(60)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 177, 197, 198, 114, 207, 60], OperandSize::Qword)
}

#[test]
fn vprolq_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLQ, operand1: Some(Direct(ZMM28)), operand2: Some(IndirectDisplaced(RAX, 463765250, Some(OperandSize::Zmmword), None)), operand3: Some(Literal8(81)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 157, 196, 114, 136, 2, 127, 164, 27, 81], OperandSize::Qword)
}

#[test]
fn vprolq_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLQ, operand1: Some(Direct(ZMM18)), operand2: Some(IndirectScaledIndexedDisplaced(RSI, RCX, Eight, 620236361, Some(OperandSize::Qword), None)), operand3: Some(Literal8(42)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 237, 215, 114, 140, 206, 73, 14, 248, 36, 42], OperandSize::Qword)
}

