use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn psubsw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PSUBSW, operand1: Some(Direct(MM7)), operand2: Some(Direct(MM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 233, 248], OperandSize::Dword)
}

#[test]
fn psubsw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PSUBSW, operand1: Some(Direct(MM0)), operand2: Some(IndirectScaledIndexedDisplaced(ESI, EAX, Two, 162180237, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 233, 132, 70, 141, 172, 170, 9], OperandSize::Dword)
}

#[test]
fn psubsw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PSUBSW, operand1: Some(Direct(MM6)), operand2: Some(Direct(MM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 233, 245], OperandSize::Qword)
}

#[test]
fn psubsw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PSUBSW, operand1: Some(Direct(MM2)), operand2: Some(IndirectScaledIndexed(RCX, RDI, Four, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 233, 20, 185], OperandSize::Qword)
}

#[test]
fn psubsw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PSUBSW, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 233, 196], OperandSize::Dword)
}

#[test]
fn psubsw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PSUBSW, operand1: Some(Direct(XMM0)), operand2: Some(IndirectScaledIndexedDisplaced(ECX, ECX, Eight, 709155997, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 233, 132, 201, 157, 220, 68, 42], OperandSize::Dword)
}

#[test]
fn psubsw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PSUBSW, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 233, 222], OperandSize::Qword)
}

#[test]
fn psubsw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PSUBSW, operand1: Some(Direct(XMM1)), operand2: Some(IndirectScaledIndexed(RAX, RDI, Two, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 233, 12, 120], OperandSize::Qword)
}

