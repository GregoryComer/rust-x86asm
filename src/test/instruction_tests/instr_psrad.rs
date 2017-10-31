use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn psrad_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PSRAD, operand1: Some(Direct(MM0)), operand2: Some(Literal8(55)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 114, 224, 55], OperandSize::Dword)
}

#[test]
fn psrad_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PSRAD, operand1: Some(Direct(MM2)), operand2: Some(Literal8(23)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 114, 226, 23], OperandSize::Qword)
}

#[test]
fn psrad_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PSRAD, operand1: Some(Direct(XMM1)), operand2: Some(Literal8(25)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 114, 225, 25], OperandSize::Dword)
}

#[test]
fn psrad_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PSRAD, operand1: Some(Direct(XMM5)), operand2: Some(Literal8(56)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 114, 229, 56], OperandSize::Qword)
}

#[test]
fn psrad_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PSRAD, operand1: Some(Direct(MM1)), operand2: Some(Direct(MM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 226, 207], OperandSize::Dword)
}

#[test]
fn psrad_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PSRAD, operand1: Some(Direct(MM3)), operand2: Some(Indirect(ECX, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 226, 25], OperandSize::Dword)
}

#[test]
fn psrad_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PSRAD, operand1: Some(Direct(MM2)), operand2: Some(Direct(MM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 226, 211], OperandSize::Qword)
}

#[test]
fn psrad_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PSRAD, operand1: Some(Direct(MM4)), operand2: Some(IndirectScaledIndexedDisplaced(RDI, RSI, Four, 898138850, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 226, 164, 183, 226, 130, 136, 53], OperandSize::Qword)
}

#[test]
fn psrad_9() {
    run_test(&Instruction { mnemonic: Mnemonic::PSRAD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 226, 202], OperandSize::Dword)
}

#[test]
fn psrad_10() {
    run_test(&Instruction { mnemonic: Mnemonic::PSRAD, operand1: Some(Direct(XMM0)), operand2: Some(IndirectDisplaced(EDI, 1645430837, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 226, 135, 53, 72, 19, 98], OperandSize::Dword)
}

#[test]
fn psrad_11() {
    run_test(&Instruction { mnemonic: Mnemonic::PSRAD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 226, 234], OperandSize::Qword)
}

#[test]
fn psrad_12() {
    run_test(&Instruction { mnemonic: Mnemonic::PSRAD, operand1: Some(Direct(XMM6)), operand2: Some(IndirectScaledIndexed(RSI, RSI, Four, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 226, 52, 182], OperandSize::Qword)
}

