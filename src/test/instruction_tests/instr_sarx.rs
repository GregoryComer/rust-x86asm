use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn sarx_1() {
    run_test(&Instruction { mnemonic: Mnemonic::SARX, operand1: Some(Direct(EBX)), operand2: Some(Direct(EBX)), operand3: Some(Direct(EBX)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 98, 247, 219], OperandSize::Dword)
}

#[test]
fn sarx_2() {
    run_test(&Instruction { mnemonic: Mnemonic::SARX, operand1: Some(Direct(ECX)), operand2: Some(Indirect(ECX, Some(OperandSize::Dword), None)), operand3: Some(Direct(EDX)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 106, 247, 9], OperandSize::Dword)
}

#[test]
fn sarx_3() {
    run_test(&Instruction { mnemonic: Mnemonic::SARX, operand1: Some(Direct(ESP)), operand2: Some(Direct(EBP)), operand3: Some(Direct(ESP)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 90, 247, 229], OperandSize::Qword)
}

#[test]
fn sarx_4() {
    run_test(&Instruction { mnemonic: Mnemonic::SARX, operand1: Some(Direct(ESP)), operand2: Some(IndirectDisplaced(RCX, 2077101296, Some(OperandSize::Dword), None)), operand3: Some(Direct(EBX)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 98, 247, 161, 240, 12, 206, 123], OperandSize::Qword)
}

#[test]
fn sarx_5() {
    run_test(&Instruction { mnemonic: Mnemonic::SARX, operand1: Some(Direct(RSI)), operand2: Some(Direct(RCX)), operand3: Some(Direct(RSI)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 202, 247, 241], OperandSize::Qword)
}

#[test]
fn sarx_6() {
    run_test(&Instruction { mnemonic: Mnemonic::SARX, operand1: Some(Direct(RBX)), operand2: Some(Indirect(RBX, Some(OperandSize::Qword), None)), operand3: Some(Direct(RDX)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 234, 247, 27], OperandSize::Qword)
}

