use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn phsubw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PHSUBW, operand1: Some(Direct(MM6)), operand2: Some(Direct(MM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 5, 247], OperandSize::Dword)
}

#[test]
fn phsubw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PHSUBW, operand1: Some(Direct(MM4)), operand2: Some(IndirectScaledIndexed(ESI, EAX, Four, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 5, 36, 134], OperandSize::Dword)
}

#[test]
fn phsubw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PHSUBW, operand1: Some(Direct(MM2)), operand2: Some(Direct(MM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 5, 208], OperandSize::Qword)
}

#[test]
fn phsubw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PHSUBW, operand1: Some(Direct(MM4)), operand2: Some(IndirectScaledIndexed(RSI, RSI, Four, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 5, 36, 182], OperandSize::Qword)
}

#[test]
fn phsubw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PHSUBW, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 5, 218], OperandSize::Dword)
}

#[test]
fn phsubw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PHSUBW, operand1: Some(Direct(XMM0)), operand2: Some(IndirectScaledDisplaced(EAX, Four, 1336813683, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 5, 4, 133, 115, 40, 174, 79], OperandSize::Dword)
}

#[test]
fn phsubw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PHSUBW, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 5, 207], OperandSize::Qword)
}

#[test]
fn phsubw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PHSUBW, operand1: Some(Direct(XMM6)), operand2: Some(IndirectDisplaced(RAX, 1835462171, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 5, 176, 27, 238, 102, 109], OperandSize::Qword)
}

