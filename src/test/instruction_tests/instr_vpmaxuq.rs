use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpmaxuq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUQ, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 229, 138, 63, 250], OperandSize::Dword)
}

#[test]
fn vpmaxuq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUQ, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectScaledDisplaced(EDX, Eight, 1686085684, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 205, 140, 63, 12, 213, 52, 160, 127, 100], OperandSize::Dword)
}

#[test]
fn vpmaxuq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUQ, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectScaledDisplaced(EAX, Eight, 953815858, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 242, 245, 154, 63, 28, 197, 50, 19, 218, 56], OperandSize::Dword)
}

#[test]
fn vpmaxuq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUQ, operand1: Some(Direct(XMM10)), operand2: Some(Direct(XMM19)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 114, 229, 129, 63, 215], OperandSize::Qword)
}

#[test]
fn vpmaxuq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUQ, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM31)), operand3: Some(IndirectDisplaced(RDX, 461420095, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 133, 129, 63, 186, 63, 182, 128, 27], OperandSize::Qword)
}

#[test]
fn vpmaxuq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUQ, operand1: Some(Direct(XMM11)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledIndexedDisplaced(RDI, RDI, Two, 824510953, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 114, 253, 159, 63, 156, 127, 233, 9, 37, 49], OperandSize::Qword)
}

#[test]
fn vpmaxuq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUQ, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 237, 169, 63, 192], OperandSize::Dword)
}

#[test]
fn vpmaxuq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUQ, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM2)), operand3: Some(Indirect(ECX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 237, 174, 63, 17], OperandSize::Dword)
}

#[test]
fn vpmaxuq_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUQ, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectScaledIndexed(EBX, EAX, Two, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 253, 185, 63, 20, 67], OperandSize::Dword)
}

#[test]
fn vpmaxuq_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUQ, operand1: Some(Direct(YMM17)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(YMM21)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 162, 229, 172, 63, 205], OperandSize::Qword)
}

#[test]
fn vpmaxuq_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUQ, operand1: Some(Direct(YMM13)), operand2: Some(Direct(YMM19)), operand3: Some(IndirectScaledDisplaced(RCX, Two, 1385014916, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 114, 229, 167, 63, 44, 77, 132, 166, 141, 82], OperandSize::Qword)
}

#[test]
fn vpmaxuq_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUQ, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM13)), operand3: Some(IndirectScaledIndexed(RCX, RSI, Eight, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 149, 186, 63, 4, 241], OperandSize::Qword)
}

#[test]
fn vpmaxuq_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUQ, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM2)), operand3: Some(Direct(ZMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 237, 203, 63, 223], OperandSize::Dword)
}

#[test]
fn vpmaxuq_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUQ, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM7)), operand3: Some(IndirectDisplaced(ECX, 420387180, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 197, 204, 63, 129, 108, 153, 14, 25], OperandSize::Dword)
}

#[test]
fn vpmaxuq_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUQ, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM5)), operand3: Some(Indirect(EAX, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 213, 217, 63, 40], OperandSize::Dword)
}

#[test]
fn vpmaxuq_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUQ, operand1: Some(Direct(ZMM11)), operand2: Some(Direct(ZMM10)), operand3: Some(Direct(ZMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 114, 173, 205, 63, 220], OperandSize::Qword)
}

#[test]
fn vpmaxuq_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUQ, operand1: Some(Direct(ZMM28)), operand2: Some(Direct(ZMM5)), operand3: Some(IndirectScaledIndexedDisplaced(RSI, RSI, Two, 833186402, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 98, 213, 202, 63, 164, 118, 98, 106, 169, 49], OperandSize::Qword)
}

#[test]
fn vpmaxuq_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXUQ, operand1: Some(Direct(ZMM24)), operand2: Some(Direct(ZMM15)), operand3: Some(IndirectDisplaced(RDI, 158141215, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 98, 133, 220, 63, 135, 31, 11, 109, 9], OperandSize::Qword)
}

