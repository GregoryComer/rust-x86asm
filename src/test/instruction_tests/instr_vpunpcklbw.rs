use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpunpcklbw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKLBW, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 233, 96, 217], OperandSize::Dword)
}

#[test]
fn vpunpcklbw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKLBW, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledIndexed(EBX, EAX, Two, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 217, 96, 12, 67], OperandSize::Dword)
}

#[test]
fn vpunpcklbw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKLBW, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 193, 96, 214], OperandSize::Qword)
}

#[test]
fn vpunpcklbw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKLBW, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM3)), operand3: Some(Indirect(RSI, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 225, 96, 14], OperandSize::Qword)
}

#[test]
fn vpunpcklbw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKLBW, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(YMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 205, 96, 227], OperandSize::Dword)
}

#[test]
fn vpunpcklbw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKLBW, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectScaledIndexed(EDX, EDX, Two, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 213, 96, 4, 82], OperandSize::Dword)
}

#[test]
fn vpunpcklbw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKLBW, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 221, 96, 241], OperandSize::Qword)
}

#[test]
fn vpunpcklbw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKLBW, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectDisplaced(RDX, 1275955566, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 96, 154, 110, 137, 13, 76], OperandSize::Qword)
}

#[test]
fn vpunpcklbw_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKLBW, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 77, 143, 96, 242], OperandSize::Dword)
}

#[test]
fn vpunpcklbw_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKLBW, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM6)), operand3: Some(Indirect(ESI, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 77, 138, 96, 62], OperandSize::Dword)
}

#[test]
fn vpunpcklbw_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKLBW, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM23)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 69, 130, 96, 192], OperandSize::Qword)
}

#[test]
fn vpunpcklbw_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKLBW, operand1: Some(Direct(XMM19)), operand2: Some(Direct(XMM29)), operand3: Some(IndirectScaledIndexed(RCX, RDI, Two, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 225, 21, 131, 96, 28, 121], OperandSize::Qword)
}

