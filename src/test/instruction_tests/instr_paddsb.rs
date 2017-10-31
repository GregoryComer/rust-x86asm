use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn paddsb_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PADDSB, operand1: Some(Direct(MM7)), operand2: Some(Direct(MM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 236, 254], OperandSize::Dword)
}

#[test]
fn paddsb_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PADDSB, operand1: Some(Direct(MM6)), operand2: Some(IndirectScaledIndexedDisplaced(ECX, ESI, Eight, 505379045, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 236, 180, 241, 229, 120, 31, 30], OperandSize::Dword)
}

#[test]
fn paddsb_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PADDSB, operand1: Some(Direct(MM2)), operand2: Some(Direct(MM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 236, 214], OperandSize::Qword)
}

#[test]
fn paddsb_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PADDSB, operand1: Some(Direct(MM2)), operand2: Some(Indirect(RAX, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 236, 16], OperandSize::Qword)
}

#[test]
fn paddsb_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PADDSB, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 236, 195], OperandSize::Dword)
}

#[test]
fn paddsb_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PADDSB, operand1: Some(Direct(XMM0)), operand2: Some(IndirectDisplaced(EDI, 226493950, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 236, 135, 254, 5, 128, 13], OperandSize::Dword)
}

#[test]
fn paddsb_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PADDSB, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 236, 220], OperandSize::Qword)
}

#[test]
fn paddsb_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PADDSB, operand1: Some(Direct(XMM6)), operand2: Some(Indirect(RSI, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 236, 54], OperandSize::Qword)
}

