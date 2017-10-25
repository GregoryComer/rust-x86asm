use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn punpckhbw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PUNPCKHBW, operand1: Some(Direct(MM1)), operand2: Some(Direct(MM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 104, 204], OperandSize::Dword)
}

#[test]
fn punpckhbw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PUNPCKHBW, operand1: Some(Direct(MM5)), operand2: Some(IndirectScaledIndexed(ECX, ECX, Two, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 104, 44, 73], OperandSize::Dword)
}

#[test]
fn punpckhbw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PUNPCKHBW, operand1: Some(Direct(MM5)), operand2: Some(Direct(MM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 104, 233], OperandSize::Qword)
}

#[test]
fn punpckhbw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PUNPCKHBW, operand1: Some(Direct(MM3)), operand2: Some(IndirectScaledIndexed(RCX, RBX, Four, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 104, 28, 153], OperandSize::Qword)
}

#[test]
fn punpckhbw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PUNPCKHBW, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 104, 201], OperandSize::Dword)
}

#[test]
fn punpckhbw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PUNPCKHBW, operand1: Some(Direct(XMM2)), operand2: Some(IndirectScaledIndexedDisplaced(EBX, EDI, Two, 25438983, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 104, 148, 123, 7, 43, 132, 1], OperandSize::Dword)
}

#[test]
fn punpckhbw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PUNPCKHBW, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 104, 239], OperandSize::Qword)
}

#[test]
fn punpckhbw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PUNPCKHBW, operand1: Some(Direct(XMM4)), operand2: Some(IndirectScaledDisplaced(RDI, Eight, 423704195, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 104, 36, 253, 131, 54, 65, 25], OperandSize::Qword)
}

