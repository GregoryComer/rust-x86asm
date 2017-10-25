use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn psrlq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PSRLQ, operand1: Some(Direct(MM4)), operand2: Some(Literal8(51)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 115, 212, 51], OperandSize::Dword)
}

#[test]
fn psrlq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PSRLQ, operand1: Some(Direct(MM3)), operand2: Some(Literal8(58)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 115, 211, 58], OperandSize::Qword)
}

#[test]
fn psrlq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PSRLQ, operand1: Some(Direct(XMM1)), operand2: Some(Literal8(86)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 115, 209, 86], OperandSize::Dword)
}

#[test]
fn psrlq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PSRLQ, operand1: Some(Direct(XMM6)), operand2: Some(Literal8(99)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 115, 214, 99], OperandSize::Qword)
}

#[test]
fn psrlq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PSRLQ, operand1: Some(Direct(MM0)), operand2: Some(Direct(MM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 211, 198], OperandSize::Dword)
}

#[test]
fn psrlq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PSRLQ, operand1: Some(Direct(MM3)), operand2: Some(IndirectScaledIndexed(EAX, EDX, Eight, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 211, 28, 208], OperandSize::Dword)
}

#[test]
fn psrlq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PSRLQ, operand1: Some(Direct(MM2)), operand2: Some(Direct(MM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 211, 213], OperandSize::Qword)
}

#[test]
fn psrlq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PSRLQ, operand1: Some(Direct(MM5)), operand2: Some(IndirectScaledDisplaced(RDI, Eight, 482454969, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 211, 44, 253, 185, 173, 193, 28], OperandSize::Qword)
}

#[test]
fn psrlq_9() {
    run_test(&Instruction { mnemonic: Mnemonic::PSRLQ, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 211, 247], OperandSize::Dword)
}

#[test]
fn psrlq_10() {
    run_test(&Instruction { mnemonic: Mnemonic::PSRLQ, operand1: Some(Direct(XMM0)), operand2: Some(IndirectScaledDisplaced(EDI, Four, 889675677, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 211, 4, 189, 157, 95, 7, 53], OperandSize::Dword)
}

#[test]
fn psrlq_11() {
    run_test(&Instruction { mnemonic: Mnemonic::PSRLQ, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 211, 224], OperandSize::Qword)
}

#[test]
fn psrlq_12() {
    run_test(&Instruction { mnemonic: Mnemonic::PSRLQ, operand1: Some(Direct(XMM0)), operand2: Some(IndirectScaledDisplaced(RCX, Eight, 538984182, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 211, 4, 205, 246, 62, 32, 32], OperandSize::Qword)
}

