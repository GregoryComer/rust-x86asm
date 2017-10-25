use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpunpckhbw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKHBW, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 209, 104, 245], OperandSize::Dword)
}

#[test]
fn vpunpckhbw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKHBW, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM7)), operand3: Some(Indirect(EAX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 193, 104, 8], OperandSize::Dword)
}

#[test]
fn vpunpckhbw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKHBW, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 217, 104, 215], OperandSize::Qword)
}

#[test]
fn vpunpckhbw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKHBW, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledIndexedDisplaced(RCX, RCX, Eight, 162115564, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 209, 104, 148, 201, 236, 175, 169, 9], OperandSize::Qword)
}

#[test]
fn vpunpckhbw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKHBW, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 237, 104, 249], OperandSize::Dword)
}

#[test]
fn vpunpckhbw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKHBW, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM2)), operand3: Some(Indirect(EAX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 237, 104, 48], OperandSize::Dword)
}

#[test]
fn vpunpckhbw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKHBW, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 245, 104, 193], OperandSize::Qword)
}

#[test]
fn vpunpckhbw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKHBW, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectDisplaced(RDI, 1336957275, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 245, 104, 191, 91, 89, 176, 79], OperandSize::Qword)
}

#[test]
fn vpunpckhbw_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKHBW, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 77, 141, 104, 200], OperandSize::Dword)
}

#[test]
fn vpunpckhbw_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKHBW, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledDisplaced(EDI, Two, 1584337161, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 125, 137, 104, 20, 125, 9, 17, 111, 94], OperandSize::Dword)
}

#[test]
fn vpunpckhbw_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKHBW, operand1: Some(Direct(XMM13)), operand2: Some(Direct(XMM24)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 113, 61, 130, 104, 234], OperandSize::Qword)
}

#[test]
fn vpunpckhbw_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKHBW, operand1: Some(Direct(XMM27)), operand2: Some(Direct(XMM22)), operand3: Some(Indirect(RCX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 97, 77, 135, 104, 25], OperandSize::Qword)
}

