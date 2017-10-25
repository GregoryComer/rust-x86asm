use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn shrx_1() {
    run_test(&Instruction { mnemonic: Mnemonic::SHRX, operand1: Some(Direct(EBX)), operand2: Some(Direct(EBP)), operand3: Some(Direct(EDX)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 107, 247, 221], OperandSize::Dword)
}

#[test]
fn shrx_2() {
    run_test(&Instruction { mnemonic: Mnemonic::SHRX, operand1: Some(Direct(EBX)), operand2: Some(Indirect(ESI, Some(OperandSize::Dword), None)), operand3: Some(Direct(EDX)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 107, 247, 30], OperandSize::Dword)
}

#[test]
fn shrx_3() {
    run_test(&Instruction { mnemonic: Mnemonic::SHRX, operand1: Some(Direct(ESP)), operand2: Some(Direct(ESP)), operand3: Some(Direct(ECX)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 115, 247, 228], OperandSize::Qword)
}

#[test]
fn shrx_4() {
    run_test(&Instruction { mnemonic: Mnemonic::SHRX, operand1: Some(Direct(EBX)), operand2: Some(Indirect(RBX, Some(OperandSize::Dword), None)), operand3: Some(Direct(EBP)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 83, 247, 27], OperandSize::Qword)
}

#[test]
fn shrx_5() {
    run_test(&Instruction { mnemonic: Mnemonic::SHRX, operand1: Some(Direct(RCX)), operand2: Some(Direct(RDI)), operand3: Some(Direct(RBX)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 227, 247, 207], OperandSize::Qword)
}

#[test]
fn shrx_6() {
    run_test(&Instruction { mnemonic: Mnemonic::SHRX, operand1: Some(Direct(RBP)), operand2: Some(Indirect(RCX, Some(OperandSize::Qword), None)), operand3: Some(Direct(RSI)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 203, 247, 41], OperandSize::Qword)
}

