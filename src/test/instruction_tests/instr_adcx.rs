use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn adcx_1() {
    run_test(&Instruction { mnemonic: Mnemonic::ADCX, operand1: Some(Direct(EBX)), operand2: Some(Direct(EBX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 246, 219], OperandSize::Dword)
}

#[test]
fn adcx_2() {
    run_test(&Instruction { mnemonic: Mnemonic::ADCX, operand1: Some(Direct(EBP)), operand2: Some(IndirectDisplaced(ESI, 1257266260, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 246, 174, 84, 92, 240, 74], OperandSize::Dword)
}

#[test]
fn adcx_3() {
    run_test(&Instruction { mnemonic: Mnemonic::ADCX, operand1: Some(Direct(EBP)), operand2: Some(Direct(EDX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 246, 234], OperandSize::Qword)
}

#[test]
fn adcx_4() {
    run_test(&Instruction { mnemonic: Mnemonic::ADCX, operand1: Some(Direct(EBX)), operand2: Some(Indirect(RDX, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 246, 26], OperandSize::Qword)
}

#[test]
fn adcx_5() {
    run_test(&Instruction { mnemonic: Mnemonic::ADCX, operand1: Some(Direct(RSI)), operand2: Some(Direct(RDI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 72, 15, 56, 246, 247], OperandSize::Qword)
}

#[test]
fn adcx_6() {
    run_test(&Instruction { mnemonic: Mnemonic::ADCX, operand1: Some(Direct(RDI)), operand2: Some(Indirect(RAX, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 72, 15, 56, 246, 56], OperandSize::Qword)
}

