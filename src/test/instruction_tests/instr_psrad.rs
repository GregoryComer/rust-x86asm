use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn psrad_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PSRAD, operand1: Some(Direct(MM3)), operand2: Some(Literal8(99)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 114, 227, 99], OperandSize::Dword)
}

#[test]
fn psrad_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PSRAD, operand1: Some(Direct(MM7)), operand2: Some(Literal8(43)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 114, 231, 43], OperandSize::Qword)
}

#[test]
fn psrad_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PSRAD, operand1: Some(Direct(XMM0)), operand2: Some(Literal8(64)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 114, 224, 64], OperandSize::Dword)
}

#[test]
fn psrad_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PSRAD, operand1: Some(Direct(XMM4)), operand2: Some(Literal8(30)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 114, 228, 30], OperandSize::Qword)
}

#[test]
fn psrad_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PSRAD, operand1: Some(Direct(MM4)), operand2: Some(Direct(MM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 226, 230], OperandSize::Dword)
}

#[test]
fn psrad_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PSRAD, operand1: Some(Direct(MM1)), operand2: Some(IndirectScaledDisplaced(EDX, Eight, 848418466, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 226, 12, 213, 162, 214, 145, 50], OperandSize::Dword)
}

#[test]
fn psrad_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PSRAD, operand1: Some(Direct(MM4)), operand2: Some(Direct(MM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 226, 228], OperandSize::Qword)
}

#[test]
fn psrad_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PSRAD, operand1: Some(Direct(MM4)), operand2: Some(IndirectDisplaced(RSI, 816367504, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 226, 166, 144, 199, 168, 48], OperandSize::Qword)
}

#[test]
fn psrad_9() {
    run_test(&Instruction { mnemonic: Mnemonic::PSRAD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 226, 235], OperandSize::Dword)
}

#[test]
fn psrad_10() {
    run_test(&Instruction { mnemonic: Mnemonic::PSRAD, operand1: Some(Direct(XMM1)), operand2: Some(Indirect(EAX, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 226, 8], OperandSize::Dword)
}

#[test]
fn psrad_11() {
    run_test(&Instruction { mnemonic: Mnemonic::PSRAD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 226, 205], OperandSize::Qword)
}

#[test]
fn psrad_12() {
    run_test(&Instruction { mnemonic: Mnemonic::PSRAD, operand1: Some(Direct(XMM5)), operand2: Some(Indirect(RDX, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 226, 42], OperandSize::Qword)
}

