use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn sarx_1() {
    run_test(&Instruction { mnemonic: Mnemonic::SARX, operand1: Some(Direct(ESP)), operand2: Some(Direct(EBP)), operand3: Some(Direct(ESP)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 90, 247, 229], OperandSize::Dword)
}

#[test]
fn sarx_2() {
    run_test(&Instruction { mnemonic: Mnemonic::SARX, operand1: Some(Direct(EBX)), operand2: Some(IndirectScaledDisplaced(EDX, Eight, 402262759, Some(OperandSize::Dword), None)), operand3: Some(Direct(EBP)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 82, 247, 28, 213, 231, 10, 250, 23], OperandSize::Dword)
}

#[test]
fn sarx_3() {
    run_test(&Instruction { mnemonic: Mnemonic::SARX, operand1: Some(Direct(ECX)), operand2: Some(Direct(EBP)), operand3: Some(Direct(EDX)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 106, 247, 205], OperandSize::Qword)
}

#[test]
fn sarx_4() {
    run_test(&Instruction { mnemonic: Mnemonic::SARX, operand1: Some(Direct(EDX)), operand2: Some(IndirectScaledIndexedDisplaced(RDX, RDX, Two, 1115894547, Some(OperandSize::Dword), None)), operand3: Some(Direct(EDI)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 66, 247, 148, 82, 19, 51, 131, 66], OperandSize::Qword)
}

#[test]
fn sarx_5() {
    run_test(&Instruction { mnemonic: Mnemonic::SARX, operand1: Some(Direct(RSI)), operand2: Some(Direct(RDI)), operand3: Some(Direct(RSI)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 202, 247, 247], OperandSize::Qword)
}

#[test]
fn sarx_6() {
    run_test(&Instruction { mnemonic: Mnemonic::SARX, operand1: Some(Direct(RSI)), operand2: Some(IndirectScaledIndexedDisplaced(RAX, RAX, Two, 1286104223, Some(OperandSize::Qword), None)), operand3: Some(Direct(RSI)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 202, 247, 180, 64, 159, 100, 168, 76], OperandSize::Qword)
}

