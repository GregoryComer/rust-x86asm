use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn punpckldq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PUNPCKLDQ, operand1: Some(Direct(MM5)), operand2: Some(Direct(MM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 98, 236], OperandSize::Dword)
}

#[test]
fn punpckldq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PUNPCKLDQ, operand1: Some(Direct(MM7)), operand2: Some(IndirectScaledDisplaced(EDX, Four, 1922520568, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 98, 60, 149, 248, 85, 151, 114], OperandSize::Dword)
}

#[test]
fn punpckldq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PUNPCKLDQ, operand1: Some(Direct(MM3)), operand2: Some(Direct(MM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 98, 221], OperandSize::Qword)
}

#[test]
fn punpckldq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PUNPCKLDQ, operand1: Some(Direct(MM5)), operand2: Some(IndirectScaledIndexedDisplaced(RDX, RDI, Two, 980670087, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 98, 172, 122, 135, 214, 115, 58], OperandSize::Qword)
}

#[test]
fn punpckldq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PUNPCKLDQ, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 98, 193], OperandSize::Dword)
}

#[test]
fn punpckldq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PUNPCKLDQ, operand1: Some(Direct(XMM3)), operand2: Some(IndirectScaledDisplaced(EDX, Four, 190701924, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 98, 28, 149, 100, 225, 93, 11], OperandSize::Dword)
}

#[test]
fn punpckldq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PUNPCKLDQ, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 98, 225], OperandSize::Qword)
}

#[test]
fn punpckldq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PUNPCKLDQ, operand1: Some(Direct(XMM4)), operand2: Some(Indirect(RDX, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 98, 34], OperandSize::Qword)
}

