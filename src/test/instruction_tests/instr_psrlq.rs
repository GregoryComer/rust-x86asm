use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn psrlq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PSRLQ, operand1: Some(Direct(MM6)), operand2: Some(Literal8(23)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 115, 214, 23], OperandSize::Dword)
}

#[test]
fn psrlq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PSRLQ, operand1: Some(Direct(MM6)), operand2: Some(Literal8(103)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 115, 214, 103], OperandSize::Qword)
}

#[test]
fn psrlq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PSRLQ, operand1: Some(Direct(XMM4)), operand2: Some(Literal8(60)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 115, 212, 60], OperandSize::Dword)
}

#[test]
fn psrlq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PSRLQ, operand1: Some(Direct(XMM5)), operand2: Some(Literal8(0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 115, 213, 0], OperandSize::Qword)
}

#[test]
fn psrlq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PSRLQ, operand1: Some(Direct(MM2)), operand2: Some(Direct(MM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 211, 210], OperandSize::Dword)
}

#[test]
fn psrlq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PSRLQ, operand1: Some(Direct(MM0)), operand2: Some(IndirectScaledIndexed(EDX, ESI, Four, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 211, 4, 178], OperandSize::Dword)
}

#[test]
fn psrlq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PSRLQ, operand1: Some(Direct(MM4)), operand2: Some(Direct(MM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 211, 226], OperandSize::Qword)
}

#[test]
fn psrlq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PSRLQ, operand1: Some(Direct(MM2)), operand2: Some(IndirectScaledIndexed(RCX, RBX, Eight, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 211, 20, 217], OperandSize::Qword)
}

#[test]
fn psrlq_9() {
    run_test(&Instruction { mnemonic: Mnemonic::PSRLQ, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 211, 255], OperandSize::Dword)
}

#[test]
fn psrlq_10() {
    run_test(&Instruction { mnemonic: Mnemonic::PSRLQ, operand1: Some(Direct(XMM3)), operand2: Some(IndirectScaledIndexed(EDI, ECX, Four, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 211, 28, 143], OperandSize::Dword)
}

#[test]
fn psrlq_11() {
    run_test(&Instruction { mnemonic: Mnemonic::PSRLQ, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 211, 245], OperandSize::Qword)
}

#[test]
fn psrlq_12() {
    run_test(&Instruction { mnemonic: Mnemonic::PSRLQ, operand1: Some(Direct(XMM3)), operand2: Some(IndirectScaledIndexed(RDI, RSI, Four, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 211, 28, 183], OperandSize::Qword)
}

