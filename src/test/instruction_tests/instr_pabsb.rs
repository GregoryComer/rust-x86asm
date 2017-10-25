use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn pabsb_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PABSB, operand1: Some(Direct(MM3)), operand2: Some(Direct(MM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 28, 221], OperandSize::Dword)
}

#[test]
fn pabsb_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PABSB, operand1: Some(Direct(MM0)), operand2: Some(Indirect(EDI, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 28, 7], OperandSize::Dword)
}

#[test]
fn pabsb_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PABSB, operand1: Some(Direct(MM6)), operand2: Some(Direct(MM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 28, 245], OperandSize::Qword)
}

#[test]
fn pabsb_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PABSB, operand1: Some(Direct(MM7)), operand2: Some(IndirectScaledIndexedDisplaced(RCX, RAX, Eight, 1775871999, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 28, 188, 193, 255, 167, 217, 105], OperandSize::Qword)
}

#[test]
fn pabsb_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PABSB, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 28, 245], OperandSize::Dword)
}

#[test]
fn pabsb_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PABSB, operand1: Some(Direct(XMM7)), operand2: Some(Indirect(ECX, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 28, 57], OperandSize::Dword)
}

#[test]
fn pabsb_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PABSB, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 28, 207], OperandSize::Qword)
}

#[test]
fn pabsb_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PABSB, operand1: Some(Direct(XMM7)), operand2: Some(Indirect(RAX, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 28, 56], OperandSize::Qword)
}

