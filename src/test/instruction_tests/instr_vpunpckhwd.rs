use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpunpckhwd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKHWD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 241, 105, 254], OperandSize::Dword)
}

#[test]
fn vpunpckhwd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKHWD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectScaledIndexed(EDI, EDX, Two, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 241, 105, 12, 87], OperandSize::Dword)
}

#[test]
fn vpunpckhwd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKHWD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 217, 105, 251], OperandSize::Qword)
}

#[test]
fn vpunpckhwd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKHWD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM5)), operand3: Some(Indirect(RSI, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 209, 105, 38], OperandSize::Qword)
}

#[test]
fn vpunpckhwd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKHWD, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(YMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 205, 105, 225], OperandSize::Dword)
}

#[test]
fn vpunpckhwd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKHWD, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM2)), operand3: Some(Indirect(EBX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 237, 105, 11], OperandSize::Dword)
}

#[test]
fn vpunpckhwd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKHWD, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 221, 105, 206], OperandSize::Qword)
}

#[test]
fn vpunpckhwd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKHWD, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectScaledIndexed(RAX, RDI, Four, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 245, 105, 36, 184], OperandSize::Qword)
}

#[test]
fn vpunpckhwd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKHWD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 85, 140, 105, 223], OperandSize::Dword)
}

#[test]
fn vpunpckhwd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKHWD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectDisplaced(ESI, 571875139, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 125, 142, 105, 174, 67, 31, 22, 34], OperandSize::Dword)
}

#[test]
fn vpunpckhwd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKHWD, operand1: Some(Direct(XMM17)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM9)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 193, 125, 139, 105, 201], OperandSize::Qword)
}

#[test]
fn vpunpckhwd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPUNPCKHWD, operand1: Some(Direct(XMM18)), operand2: Some(Direct(XMM22)), operand3: Some(IndirectScaledDisplaced(RBX, Eight, 222008945, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 225, 77, 132, 105, 20, 221, 113, 150, 59, 13], OperandSize::Qword)
}

