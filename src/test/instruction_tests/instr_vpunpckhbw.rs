use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpunpckhbw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKHBW, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 241, 104, 218], OperandSize::Dword)
}

#[test]
fn vpunpckhbw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKHBW, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectScaledIndexed(EAX, ESI, Eight, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 201, 104, 44, 240], OperandSize::Dword)
}

#[test]
fn vpunpckhbw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKHBW, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 241, 104, 245], OperandSize::Qword)
}

#[test]
fn vpunpckhbw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKHBW, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM6)), operand3: Some(Indirect(RSI, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 201, 104, 6], OperandSize::Qword)
}

#[test]
fn vpunpckhbw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKHBW, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(YMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 205, 104, 205], OperandSize::Dword)
}

#[test]
fn vpunpckhbw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKHBW, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM5)), operand3: Some(Indirect(EAX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 213, 104, 40], OperandSize::Dword)
}

#[test]
fn vpunpckhbw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKHBW, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 197, 104, 228], OperandSize::Qword)
}

#[test]
fn vpunpckhbw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKHBW, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectScaledIndexed(RDI, RDI, Four, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 237, 104, 28, 191], OperandSize::Qword)
}

#[test]
fn vpunpckhbw_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKHBW, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 109, 141, 104, 231], OperandSize::Dword)
}

#[test]
fn vpunpckhbw_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKHBW, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM2)), operand3: Some(Indirect(EAX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 109, 137, 104, 48], OperandSize::Dword)
}

#[test]
fn vpunpckhbw_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKHBW, operand1: Some(Direct(XMM31)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM19)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 33, 109, 139, 104, 251], OperandSize::Qword)
}

#[test]
fn vpunpckhbw_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKHBW, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledIndexedDisplaced(RCX, RAX, Two, 792334979, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 93, 143, 104, 148, 65, 131, 18, 58, 47], OperandSize::Qword)
}

