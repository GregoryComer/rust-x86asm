use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn psraw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PSRAW, operand1: Some(Direct(MM7)), operand2: Some(Literal8(77)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 113, 231, 77], OperandSize::Dword)
}

#[test]
fn psraw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PSRAW, operand1: Some(Direct(MM2)), operand2: Some(Literal8(75)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 113, 226, 75], OperandSize::Qword)
}

#[test]
fn psraw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PSRAW, operand1: Some(Direct(XMM7)), operand2: Some(Literal8(20)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 113, 231, 20], OperandSize::Dword)
}

#[test]
fn psraw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PSRAW, operand1: Some(Direct(XMM0)), operand2: Some(Literal8(56)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 113, 224, 56], OperandSize::Qword)
}

#[test]
fn psraw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PSRAW, operand1: Some(Direct(MM7)), operand2: Some(Direct(MM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 225, 251], OperandSize::Dword)
}

#[test]
fn psraw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PSRAW, operand1: Some(Direct(MM1)), operand2: Some(IndirectScaledIndexed(ESI, EDI, Four, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 225, 12, 190], OperandSize::Dword)
}

#[test]
fn psraw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PSRAW, operand1: Some(Direct(MM7)), operand2: Some(Direct(MM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 225, 254], OperandSize::Qword)
}

#[test]
fn psraw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PSRAW, operand1: Some(Direct(MM0)), operand2: Some(IndirectDisplaced(RDI, 709011630, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 225, 135, 174, 168, 66, 42], OperandSize::Qword)
}

#[test]
fn psraw_9() {
    run_test(&Instruction { mnemonic: Mnemonic::PSRAW, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 225, 245], OperandSize::Dword)
}

#[test]
fn psraw_10() {
    run_test(&Instruction { mnemonic: Mnemonic::PSRAW, operand1: Some(Direct(XMM5)), operand2: Some(IndirectScaledIndexed(EBX, EAX, Eight, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 225, 44, 195], OperandSize::Dword)
}

#[test]
fn psraw_11() {
    run_test(&Instruction { mnemonic: Mnemonic::PSRAW, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 225, 205], OperandSize::Qword)
}

#[test]
fn psraw_12() {
    run_test(&Instruction { mnemonic: Mnemonic::PSRAW, operand1: Some(Direct(XMM7)), operand2: Some(Indirect(RAX, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 225, 56], OperandSize::Qword)
}

