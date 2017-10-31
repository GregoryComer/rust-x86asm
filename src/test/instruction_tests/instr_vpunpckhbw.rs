use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpunpckhbw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKHBW, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 225, 104, 247], OperandSize::Dword)
}

#[test]
fn vpunpckhbw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKHBW, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledDisplaced(EBX, Two, 8160955, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 193, 104, 44, 93, 187, 134, 124, 0], OperandSize::Dword)
}

#[test]
fn vpunpckhbw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKHBW, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 104, 230], OperandSize::Qword)
}

#[test]
fn vpunpckhbw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKHBW, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledDisplaced(RBX, Eight, 1644884415, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 217, 104, 44, 221, 191, 241, 10, 98], OperandSize::Qword)
}

#[test]
fn vpunpckhbw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKHBW, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(YMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 229, 104, 251], OperandSize::Dword)
}

#[test]
fn vpunpckhbw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKHBW, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM2)), operand3: Some(Indirect(ECX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 237, 104, 57], OperandSize::Dword)
}

#[test]
fn vpunpckhbw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKHBW, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(YMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 104, 233], OperandSize::Qword)
}

#[test]
fn vpunpckhbw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKHBW, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectScaledIndexed(RDI, RCX, Eight, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 245, 104, 36, 207], OperandSize::Qword)
}

#[test]
fn vpunpckhbw_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKHBW, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 93, 139, 104, 236], OperandSize::Dword)
}

#[test]
fn vpunpckhbw_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKHBW, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledDisplaced(ESI, Two, 1331364051, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 109, 143, 104, 28, 117, 211, 0, 91, 79], OperandSize::Dword)
}

#[test]
fn vpunpckhbw_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKHBW, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM16)), operand3: Some(Direct(XMM21)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 177, 125, 131, 104, 213], OperandSize::Qword)
}

#[test]
fn vpunpckhbw_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKHBW, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM8)), operand3: Some(IndirectDisplaced(RAX, 550533303, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 61, 137, 104, 152, 183, 120, 208, 32], OperandSize::Qword)
}

