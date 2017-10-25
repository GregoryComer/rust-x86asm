use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn valignq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VALIGNQ, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM0)), operand4: Some(Literal8(2)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 243, 253, 143, 3, 192, 2], OperandSize::Dword)
}

#[test]
fn valignq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VALIGNQ, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectDisplaced(EDX, 1971909047, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(111)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 243, 221, 142, 3, 138, 183, 241, 136, 117, 111], OperandSize::Dword)
}

#[test]
fn valignq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VALIGNQ, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectDisplaced(EDX, 379569552, Some(OperandSize::Qword), None)), operand4: Some(Literal8(20)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 243, 245, 157, 3, 138, 144, 197, 159, 22, 20], OperandSize::Dword)
}

#[test]
fn valignq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VALIGNQ, operand1: Some(Direct(XMM16)), operand2: Some(Direct(XMM15)), operand3: Some(Direct(XMM23)), operand4: Some(Literal8(36)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 163, 133, 142, 3, 199, 36], OperandSize::Qword)
}

#[test]
fn valignq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VALIGNQ, operand1: Some(Direct(XMM14)), operand2: Some(Direct(XMM17)), operand3: Some(IndirectScaledIndexed(RCX, RDI, Four, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(102)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 115, 245, 134, 3, 52, 185, 102], OperandSize::Qword)
}

#[test]
fn valignq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VALIGNQ, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM18)), operand3: Some(IndirectScaledDisplaced(RDI, Two, 43821627, Some(OperandSize::Qword), None)), operand4: Some(Literal8(11)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 243, 237, 145, 3, 60, 125, 59, 170, 156, 2, 11], OperandSize::Qword)
}

#[test]
fn valignq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VALIGNQ, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM3)), operand4: Some(Literal8(65)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 243, 197, 174, 3, 227, 65], OperandSize::Dword)
}

#[test]
fn valignq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VALIGNQ, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM4)), operand3: Some(Indirect(ESI, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(62)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 243, 221, 170, 3, 14, 62], OperandSize::Dword)
}

#[test]
fn valignq_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VALIGNQ, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectDisplaced(EAX, 132601719, Some(OperandSize::Qword), None)), operand4: Some(Literal8(44)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 243, 197, 185, 3, 184, 119, 87, 231, 7, 44], OperandSize::Dword)
}

#[test]
fn valignq_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VALIGNQ, operand1: Some(Direct(YMM20)), operand2: Some(Direct(YMM13)), operand3: Some(Direct(YMM7)), operand4: Some(Literal8(54)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 227, 149, 169, 3, 231, 54], OperandSize::Qword)
}

#[test]
fn valignq_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VALIGNQ, operand1: Some(Direct(YMM17)), operand2: Some(Direct(YMM16)), operand3: Some(IndirectScaledIndexedDisplaced(RBX, RDX, Eight, 1668635830, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(57)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 227, 253, 165, 3, 140, 211, 182, 92, 117, 99, 57], OperandSize::Qword)
}

#[test]
fn valignq_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VALIGNQ, operand1: Some(Direct(YMM24)), operand2: Some(Direct(YMM28)), operand3: Some(Indirect(RDX, Some(OperandSize::Qword), None)), operand4: Some(Literal8(27)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 99, 157, 182, 3, 2, 27], OperandSize::Qword)
}

#[test]
fn valignq_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VALIGNQ, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM7)), operand3: Some(Direct(ZMM5)), operand4: Some(Literal8(23)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 243, 197, 207, 3, 245, 23], OperandSize::Dword)
}

#[test]
fn valignq_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VALIGNQ, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM7)), operand3: Some(IndirectScaledDisplaced(EBX, Eight, 486580086, Some(OperandSize::Zmmword), None)), operand4: Some(Literal8(35)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 243, 197, 202, 3, 52, 221, 118, 159, 0, 29, 35], OperandSize::Dword)
}

#[test]
fn valignq_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VALIGNQ, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM1)), operand3: Some(IndirectScaledIndexedDisplaced(EAX, ECX, Eight, 559780359, Some(OperandSize::Qword), None)), operand4: Some(Literal8(124)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 243, 245, 217, 3, 140, 200, 7, 146, 93, 33, 124], OperandSize::Dword)
}

#[test]
fn valignq_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VALIGNQ, operand1: Some(Direct(ZMM18)), operand2: Some(Direct(ZMM4)), operand3: Some(Direct(ZMM3)), operand4: Some(Literal8(127)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 227, 221, 202, 3, 211, 127], OperandSize::Qword)
}

#[test]
fn valignq_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VALIGNQ, operand1: Some(Direct(ZMM11)), operand2: Some(Direct(ZMM1)), operand3: Some(Indirect(RAX, Some(OperandSize::Zmmword), None)), operand4: Some(Literal8(22)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 115, 245, 202, 3, 24, 22], OperandSize::Qword)
}

#[test]
fn valignq_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VALIGNQ, operand1: Some(Direct(ZMM13)), operand2: Some(Direct(ZMM30)), operand3: Some(IndirectScaledDisplaced(RSI, Two, 1818722850, Some(OperandSize::Qword), None)), operand4: Some(Literal8(71)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 115, 141, 210, 3, 44, 117, 34, 130, 103, 108, 71], OperandSize::Qword)
}

