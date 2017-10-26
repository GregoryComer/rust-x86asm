use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn psrad_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PSRAD, operand1: Some(Direct(MM7)), operand2: Some(Literal8(0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 114, 231, 0], OperandSize::Dword)
}

#[test]
fn psrad_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PSRAD, operand1: Some(Direct(MM5)), operand2: Some(Literal8(112)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 114, 229, 112], OperandSize::Qword)
}

#[test]
fn psrad_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PSRAD, operand1: Some(Direct(XMM4)), operand2: Some(Literal8(34)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 114, 228, 34], OperandSize::Dword)
}

#[test]
fn psrad_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PSRAD, operand1: Some(Direct(XMM3)), operand2: Some(Literal8(30)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 114, 227, 30], OperandSize::Qword)
}

#[test]
fn psrad_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PSRAD, operand1: Some(Direct(MM2)), operand2: Some(Direct(MM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 226, 209], OperandSize::Dword)
}

#[test]
fn psrad_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PSRAD, operand1: Some(Direct(MM0)), operand2: Some(IndirectDisplaced(EDX, 1923238697, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 226, 130, 41, 75, 162, 114], OperandSize::Dword)
}

#[test]
fn psrad_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PSRAD, operand1: Some(Direct(MM5)), operand2: Some(Direct(MM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 226, 239], OperandSize::Qword)
}

#[test]
fn psrad_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PSRAD, operand1: Some(Direct(MM4)), operand2: Some(IndirectScaledDisplaced(RAX, Eight, 384137863, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 226, 36, 197, 135, 122, 229, 22], OperandSize::Qword)
}

#[test]
fn psrad_9() {
    run_test(&Instruction { mnemonic: Mnemonic::PSRAD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 226, 245], OperandSize::Dword)
}

#[test]
fn psrad_10() {
    run_test(&Instruction { mnemonic: Mnemonic::PSRAD, operand1: Some(Direct(XMM4)), operand2: Some(IndirectScaledIndexedDisplaced(ESI, EBX, Four, 1563226129, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 226, 164, 158, 17, 240, 44, 93], OperandSize::Dword)
}

#[test]
fn psrad_11() {
    run_test(&Instruction { mnemonic: Mnemonic::PSRAD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 226, 226], OperandSize::Qword)
}

#[test]
fn psrad_12() {
    run_test(&Instruction { mnemonic: Mnemonic::PSRAD, operand1: Some(Direct(XMM5)), operand2: Some(IndirectScaledIndexed(RSI, RDI, Four, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 226, 44, 190], OperandSize::Qword)
}

