use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpunpckhbw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKHBW, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 241, 104, 206], OperandSize::Dword)
}

#[test]
fn vpunpckhbw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKHBW, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledIndexedDisplaced(EDX, EBX, Two, 840676149, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 233, 104, 132, 90, 53, 179, 27, 50], OperandSize::Dword)
}

#[test]
fn vpunpckhbw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKHBW, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 225, 104, 215], OperandSize::Qword)
}

#[test]
fn vpunpckhbw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKHBW, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledIndexedDisplaced(RBX, RDX, Two, 979017390, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 104, 132, 83, 174, 158, 90, 58], OperandSize::Qword)
}

#[test]
fn vpunpckhbw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKHBW, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 237, 104, 230], OperandSize::Dword)
}

#[test]
fn vpunpckhbw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKHBW, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectDisplaced(ECX, 1733236272, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 213, 104, 153, 48, 22, 79, 103], OperandSize::Dword)
}

#[test]
fn vpunpckhbw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKHBW, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(YMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 229, 104, 253], OperandSize::Qword)
}

#[test]
fn vpunpckhbw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKHBW, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectScaledIndexed(RAX, RBX, Two, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 229, 104, 12, 88], OperandSize::Qword)
}

#[test]
fn vpunpckhbw_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKHBW, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 125, 142, 104, 250], OperandSize::Dword)
}

#[test]
fn vpunpckhbw_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKHBW, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectScaledIndexed(EDI, EDX, Eight, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 117, 140, 104, 12, 215], OperandSize::Dword)
}

#[test]
fn vpunpckhbw_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKHBW, operand1: Some(Direct(XMM24)), operand2: Some(Direct(XMM25)), operand3: Some(Direct(XMM8)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 65, 53, 134, 104, 192], OperandSize::Qword)
}

#[test]
fn vpunpckhbw_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKHBW, operand1: Some(Direct(XMM15)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledIndexedDisplaced(RDX, RBX, Four, 732055851, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 113, 69, 140, 104, 188, 154, 43, 73, 162, 43], OperandSize::Qword)
}

