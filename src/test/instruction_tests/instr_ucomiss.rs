use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn ucomiss_1() {
    run_test(&Instruction { mnemonic: Mnemonic::UCOMISS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 46, 208], OperandSize::Dword)
}

#[test]
fn ucomiss_2() {
    run_test(&Instruction { mnemonic: Mnemonic::UCOMISS, operand1: Some(Direct(XMM3)), operand2: Some(Indirect(EDI, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 46, 31], OperandSize::Dword)
}

#[test]
fn ucomiss_3() {
    run_test(&Instruction { mnemonic: Mnemonic::UCOMISS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 46, 227], OperandSize::Qword)
}

#[test]
fn ucomiss_4() {
    run_test(&Instruction { mnemonic: Mnemonic::UCOMISS, operand1: Some(Direct(XMM7)), operand2: Some(IndirectScaledIndexedDisplaced(RBX, RDI, Two, 2137053184, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 46, 188, 123, 0, 216, 96, 127], OperandSize::Qword)
}

