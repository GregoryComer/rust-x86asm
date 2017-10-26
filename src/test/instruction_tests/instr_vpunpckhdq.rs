use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpunpckhdq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKHDQ, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 241, 106, 210], OperandSize::Dword)
}

#[test]
fn vpunpckhdq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKHDQ, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectDisplaced(EDI, 1858578561, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 225, 106, 143, 129, 168, 199, 110], OperandSize::Dword)
}

#[test]
fn vpunpckhdq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKHDQ, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 233, 106, 209], OperandSize::Qword)
}

#[test]
fn vpunpckhdq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKHDQ, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledDisplaced(RAX, Four, 1815769703, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 193, 106, 52, 133, 103, 114, 58, 108], OperandSize::Qword)
}

#[test]
fn vpunpckhdq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKHDQ, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 237, 106, 192], OperandSize::Dword)
}

#[test]
fn vpunpckhdq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKHDQ, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectScaledIndexedDisplaced(EAX, EAX, Two, 1412770788, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 213, 106, 140, 64, 228, 43, 53, 84], OperandSize::Dword)
}

#[test]
fn vpunpckhdq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKHDQ, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(YMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 229, 106, 247], OperandSize::Qword)
}

#[test]
fn vpunpckhdq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKHDQ, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM6)), operand3: Some(Indirect(RAX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 205, 106, 32], OperandSize::Qword)
}

#[test]
fn vpunpckhdq_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKHDQ, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 109, 143, 106, 220], OperandSize::Dword)
}

#[test]
fn vpunpckhdq_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKHDQ, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledDisplaced(EAX, Eight, 527019078, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 101, 141, 106, 60, 197, 70, 172, 105, 31], OperandSize::Dword)
}

#[test]
fn vpunpckhdq_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKHDQ, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectScaledIndexed(EAX, ESI, Two, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 77, 158, 106, 28, 112], OperandSize::Dword)
}

#[test]
fn vpunpckhdq_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKHDQ, operand1: Some(Direct(XMM28)), operand2: Some(Direct(XMM31)), operand3: Some(Direct(XMM15)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 65, 5, 131, 106, 231], OperandSize::Qword)
}

#[test]
fn vpunpckhdq_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKHDQ, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM17)), operand3: Some(IndirectScaledIndexedDisplaced(RDX, RAX, Four, 348599955, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 117, 135, 106, 156, 130, 147, 54, 199, 20], OperandSize::Qword)
}

#[test]
fn vpunpckhdq_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKHDQ, operand1: Some(Direct(XMM9)), operand2: Some(Direct(XMM19)), operand3: Some(Indirect(RDX, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 113, 101, 150, 106, 10], OperandSize::Qword)
}

