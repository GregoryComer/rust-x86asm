use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn ucomiss_1() {
    run_test(&Instruction { mnemonic: Mnemonic::UCOMISS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 46, 207], OperandSize::Dword)
}

#[test]
fn ucomiss_2() {
    run_test(&Instruction { mnemonic: Mnemonic::UCOMISS, operand1: Some(Direct(XMM7)), operand2: Some(IndirectScaledIndexedDisplaced(EBX, EBX, Eight, 1717669082, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 46, 188, 219, 218, 140, 97, 102], OperandSize::Dword)
}

#[test]
fn ucomiss_3() {
    run_test(&Instruction { mnemonic: Mnemonic::UCOMISS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 46, 215], OperandSize::Qword)
}

#[test]
fn ucomiss_4() {
    run_test(&Instruction { mnemonic: Mnemonic::UCOMISS, operand1: Some(Direct(XMM2)), operand2: Some(Indirect(RSI, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 46, 22], OperandSize::Qword)
}

