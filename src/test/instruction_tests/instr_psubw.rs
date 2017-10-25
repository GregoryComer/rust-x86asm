use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn psubw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PSUBW, operand1: Some(Direct(MM1)), operand2: Some(Direct(MM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 249, 207], OperandSize::Dword)
}

#[test]
fn psubw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PSUBW, operand1: Some(Direct(MM3)), operand2: Some(IndirectScaledIndexedDisplaced(EAX, EDX, Two, 577394144, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 249, 156, 80, 224, 85, 106, 34], OperandSize::Dword)
}

#[test]
fn psubw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PSUBW, operand1: Some(Direct(MM5)), operand2: Some(Direct(MM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 249, 233], OperandSize::Qword)
}

#[test]
fn psubw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PSUBW, operand1: Some(Direct(MM1)), operand2: Some(IndirectScaledIndexedDisplaced(RCX, RAX, Four, 1161456129, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 249, 140, 129, 1, 106, 58, 69], OperandSize::Qword)
}

#[test]
fn psubw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PSUBW, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 249, 201], OperandSize::Dword)
}

#[test]
fn psubw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PSUBW, operand1: Some(Direct(XMM6)), operand2: Some(IndirectScaledDisplaced(EBX, Two, 653811850, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 249, 52, 93, 138, 96, 248, 38], OperandSize::Dword)
}

#[test]
fn psubw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PSUBW, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 249, 222], OperandSize::Qword)
}

#[test]
fn psubw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PSUBW, operand1: Some(Direct(XMM0)), operand2: Some(IndirectDisplaced(RSI, 2011189142, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 249, 134, 150, 79, 224, 119], OperandSize::Qword)
}

