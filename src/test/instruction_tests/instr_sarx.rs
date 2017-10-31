use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn sarx_1() {
    run_test(&Instruction { mnemonic: Mnemonic::SARX, operand1: Some(Direct(EDX)), operand2: Some(Direct(EDX)), operand3: Some(Direct(ECX)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 114, 247, 210], OperandSize::Dword)
}

#[test]
fn sarx_2() {
    run_test(&Instruction { mnemonic: Mnemonic::SARX, operand1: Some(Direct(EDI)), operand2: Some(IndirectScaledIndexed(EAX, EDX, Two, Some(OperandSize::Dword), None)), operand3: Some(Direct(ESP)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 90, 247, 60, 80], OperandSize::Dword)
}

#[test]
fn sarx_3() {
    run_test(&Instruction { mnemonic: Mnemonic::SARX, operand1: Some(Direct(ECX)), operand2: Some(Direct(EDI)), operand3: Some(Direct(EBP)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 82, 247, 207], OperandSize::Qword)
}

#[test]
fn sarx_4() {
    run_test(&Instruction { mnemonic: Mnemonic::SARX, operand1: Some(Direct(ESP)), operand2: Some(IndirectScaledDisplaced(RBX, Four, 92333656, Some(OperandSize::Dword), None)), operand3: Some(Direct(ECX)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 114, 247, 36, 157, 88, 230, 128, 5], OperandSize::Qword)
}

#[test]
fn sarx_5() {
    run_test(&Instruction { mnemonic: Mnemonic::SARX, operand1: Some(Direct(RDI)), operand2: Some(Direct(RDI)), operand3: Some(Direct(RSI)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 202, 247, 255], OperandSize::Qword)
}

#[test]
fn sarx_6() {
    run_test(&Instruction { mnemonic: Mnemonic::SARX, operand1: Some(Direct(RBX)), operand2: Some(IndirectScaledIndexed(RAX, RBX, Four, Some(OperandSize::Qword), None)), operand3: Some(Direct(RSI)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 202, 247, 28, 152], OperandSize::Qword)
}

