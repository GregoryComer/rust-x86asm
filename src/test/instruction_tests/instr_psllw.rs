use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn psllw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PSLLW, operand1: Some(Direct(MM5)), operand2: Some(Literal8(30)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 113, 245, 30], OperandSize::Dword)
}

#[test]
fn psllw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PSLLW, operand1: Some(Direct(MM2)), operand2: Some(Literal8(68)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 113, 242, 68], OperandSize::Qword)
}

#[test]
fn psllw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PSLLW, operand1: Some(Direct(XMM4)), operand2: Some(Literal8(20)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 113, 244, 20], OperandSize::Dword)
}

#[test]
fn psllw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PSLLW, operand1: Some(Direct(XMM3)), operand2: Some(Literal8(66)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 113, 243, 66], OperandSize::Qword)
}

#[test]
fn psllw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PSLLW, operand1: Some(Direct(MM4)), operand2: Some(Direct(MM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 241, 230], OperandSize::Dword)
}

#[test]
fn psllw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PSLLW, operand1: Some(Direct(MM7)), operand2: Some(IndirectScaledDisplaced(EDX, Eight, 1349215125, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 241, 60, 213, 149, 99, 107, 80], OperandSize::Dword)
}

#[test]
fn psllw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PSLLW, operand1: Some(Direct(MM4)), operand2: Some(Direct(MM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 241, 224], OperandSize::Qword)
}

#[test]
fn psllw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PSLLW, operand1: Some(Direct(MM5)), operand2: Some(IndirectDisplaced(RSI, 576795522, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 241, 174, 130, 51, 97, 34], OperandSize::Qword)
}

#[test]
fn psllw_9() {
    run_test(&Instruction { mnemonic: Mnemonic::PSLLW, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 241, 229], OperandSize::Dword)
}

#[test]
fn psllw_10() {
    run_test(&Instruction { mnemonic: Mnemonic::PSLLW, operand1: Some(Direct(XMM1)), operand2: Some(IndirectScaledDisplaced(EDI, Four, 1514574181, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 241, 12, 189, 101, 145, 70, 90], OperandSize::Dword)
}

#[test]
fn psllw_11() {
    run_test(&Instruction { mnemonic: Mnemonic::PSLLW, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 241, 194], OperandSize::Qword)
}

#[test]
fn psllw_12() {
    run_test(&Instruction { mnemonic: Mnemonic::PSLLW, operand1: Some(Direct(XMM3)), operand2: Some(IndirectScaledIndexed(RSI, RCX, Four, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 241, 28, 142], OperandSize::Qword)
}

