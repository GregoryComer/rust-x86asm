use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vplzcntq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPLZCNTQ, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 253, 142, 68, 233], OperandSize::Dword)
}

#[test]
fn vplzcntq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPLZCNTQ, operand1: Some(Direct(XMM5)), operand2: Some(IndirectScaledDisplaced(EAX, Four, 163020100, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 253, 141, 68, 44, 133, 68, 125, 183, 9], OperandSize::Dword)
}

#[test]
fn vplzcntq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPLZCNTQ, operand1: Some(Direct(XMM4)), operand2: Some(IndirectDisplaced(EDI, 227502833, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 242, 253, 153, 68, 167, 241, 106, 143, 13], OperandSize::Dword)
}

#[test]
fn vplzcntq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPLZCNTQ, operand1: Some(Direct(XMM19)), operand2: Some(Direct(XMM27)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 130, 253, 138, 68, 219], OperandSize::Qword)
}

#[test]
fn vplzcntq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPLZCNTQ, operand1: Some(Direct(XMM10)), operand2: Some(IndirectScaledDisplaced(RDI, Four, 428591636, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 114, 253, 138, 68, 20, 189, 20, 202, 139, 25], OperandSize::Qword)
}

#[test]
fn vplzcntq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPLZCNTQ, operand1: Some(Direct(XMM5)), operand2: Some(IndirectDisplaced(RSI, 1815714266, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 242, 253, 154, 68, 174, 218, 153, 57, 108], OperandSize::Qword)
}

#[test]
fn vplzcntq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPLZCNTQ, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 253, 169, 68, 254], OperandSize::Dword)
}

#[test]
fn vplzcntq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPLZCNTQ, operand1: Some(Direct(YMM1)), operand2: Some(IndirectScaledIndexedDisplaced(EDX, EAX, Four, 201463421, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 253, 173, 68, 140, 130, 125, 22, 2, 12], OperandSize::Dword)
}

#[test]
fn vplzcntq_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPLZCNTQ, operand1: Some(Direct(YMM0)), operand2: Some(IndirectScaledDisplaced(EDX, Eight, 1036733594, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 253, 189, 68, 4, 213, 154, 76, 203, 61], OperandSize::Dword)
}

#[test]
fn vplzcntq_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPLZCNTQ, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 253, 174, 68, 224], OperandSize::Qword)
}

#[test]
fn vplzcntq_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPLZCNTQ, operand1: Some(Direct(YMM17)), operand2: Some(IndirectScaledDisplaced(RAX, Eight, 824289342, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 226, 253, 174, 68, 12, 197, 62, 168, 33, 49], OperandSize::Qword)
}

#[test]
fn vplzcntq_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPLZCNTQ, operand1: Some(Direct(YMM8)), operand2: Some(IndirectScaledDisplaced(RDX, Eight, 608437381, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 114, 253, 189, 68, 4, 213, 133, 4, 68, 36], OperandSize::Qword)
}

#[test]
fn vplzcntq_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPLZCNTQ, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 253, 206, 68, 196], OperandSize::Dword)
}

#[test]
fn vplzcntq_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPLZCNTQ, operand1: Some(Direct(ZMM7)), operand2: Some(IndirectScaledIndexed(ECX, ESI, Two, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 253, 206, 68, 60, 113], OperandSize::Dword)
}

#[test]
fn vplzcntq_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPLZCNTQ, operand1: Some(Direct(ZMM6)), operand2: Some(Indirect(EDI, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 253, 222, 68, 55], OperandSize::Dword)
}

#[test]
fn vplzcntq_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPLZCNTQ, operand1: Some(Direct(ZMM28)), operand2: Some(Direct(ZMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 98, 253, 205, 68, 231], OperandSize::Qword)
}

#[test]
fn vplzcntq_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPLZCNTQ, operand1: Some(Direct(ZMM20)), operand2: Some(IndirectScaledDisplaced(RDI, Four, 1626705527, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 226, 253, 203, 68, 36, 189, 119, 142, 245, 96], OperandSize::Qword)
}

#[test]
fn vplzcntq_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPLZCNTQ, operand1: Some(Direct(ZMM18)), operand2: Some(IndirectDisplaced(RSI, 227467233, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 226, 253, 223, 68, 150, 225, 223, 142, 13], OperandSize::Qword)
}

