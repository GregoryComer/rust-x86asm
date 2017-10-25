use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpunpcklwd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKLWD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 241, 97, 210], OperandSize::Dword)
}

#[test]
fn vpunpcklwd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKLWD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledIndexed(EAX, ESI, Eight, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 233, 97, 52, 240], OperandSize::Dword)
}

#[test]
fn vpunpcklwd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKLWD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 217, 97, 216], OperandSize::Qword)
}

#[test]
fn vpunpcklwd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKLWD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM2)), operand3: Some(Indirect(RDI, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 233, 97, 39], OperandSize::Qword)
}

#[test]
fn vpunpcklwd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKLWD, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(YMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 97, 212], OperandSize::Dword)
}

#[test]
fn vpunpcklwd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKLWD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM5)), operand3: Some(Indirect(EAX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 213, 97, 40], OperandSize::Dword)
}

#[test]
fn vpunpcklwd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKLWD, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 221, 97, 231], OperandSize::Qword)
}

#[test]
fn vpunpcklwd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKLWD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectScaledIndexed(RSI, RCX, Two, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 97, 44, 78], OperandSize::Qword)
}

#[test]
fn vpunpcklwd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKLWD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 125, 139, 97, 255], OperandSize::Dword)
}

#[test]
fn vpunpcklwd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKLWD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectScaledIndexed(ECX, EBX, Four, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 77, 143, 97, 4, 153], OperandSize::Dword)
}

#[test]
fn vpunpcklwd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKLWD, operand1: Some(Direct(XMM8)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM15)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 81, 69, 139, 97, 199], OperandSize::Qword)
}

#[test]
fn vpunpcklwd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKLWD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM2)), operand3: Some(Indirect(RSI, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 109, 137, 97, 6], OperandSize::Qword)
}

