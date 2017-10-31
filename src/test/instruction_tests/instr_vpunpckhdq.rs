use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpunpckhdq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKHDQ, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 106, 251], OperandSize::Dword)
}

#[test]
fn vpunpckhdq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKHDQ, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledIndexedDisplaced(EDI, EBX, Eight, 423153363, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 233, 106, 132, 223, 211, 206, 56, 25], OperandSize::Dword)
}

#[test]
fn vpunpckhdq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKHDQ, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 201, 106, 218], OperandSize::Qword)
}

#[test]
fn vpunpckhdq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKHDQ, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledIndexed(RAX, RDI, Two, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 209, 106, 52, 120], OperandSize::Qword)
}

#[test]
fn vpunpckhdq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKHDQ, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(YMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 106, 234], OperandSize::Dword)
}

#[test]
fn vpunpckhdq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKHDQ, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM2)), operand3: Some(Indirect(EAX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 237, 106, 48], OperandSize::Dword)
}

#[test]
fn vpunpckhdq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKHDQ, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 197, 106, 243], OperandSize::Qword)
}

#[test]
fn vpunpckhdq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKHDQ, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectDisplaced(RSI, 378176880, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 106, 174, 112, 133, 138, 22], OperandSize::Qword)
}

#[test]
fn vpunpckhdq_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKHDQ, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 101, 141, 106, 210], OperandSize::Dword)
}

#[test]
fn vpunpckhdq_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKHDQ, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledIndexed(EAX, EAX, Two, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 93, 138, 106, 20, 64], OperandSize::Dword)
}

#[test]
fn vpunpckhdq_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKHDQ, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectScaledDisplaced(EAX, Two, 1641272244, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 117, 154, 106, 20, 69, 180, 211, 211, 97], OperandSize::Dword)
}

#[test]
fn vpunpckhdq_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKHDQ, operand1: Some(Direct(XMM21)), operand2: Some(Direct(XMM29)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 225, 21, 130, 106, 232], OperandSize::Qword)
}

#[test]
fn vpunpckhdq_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKHDQ, operand1: Some(Direct(XMM8)), operand2: Some(Direct(XMM11)), operand3: Some(IndirectScaledIndexed(RDX, RSI, Two, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 113, 37, 141, 106, 4, 114], OperandSize::Qword)
}

#[test]
fn vpunpckhdq_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKHDQ, operand1: Some(Direct(XMM16)), operand2: Some(Direct(XMM28)), operand3: Some(Indirect(RBX, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 225, 29, 150, 106, 3], OperandSize::Qword)
}

